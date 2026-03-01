//! MQ REST API session and command execution.

use std::collections::HashMap;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde_json::Value;

use crate::auth::{Credentials, perform_ltpa_login};
use crate::error::{MappingError, MappingIssue, MqRestError, Result};
use crate::mapping::{map_request_attributes, map_response_list};
use crate::mapping_data::MAPPING_DATA;
use crate::mapping_merge::{
    MappingOverrideMode, merge_mapping_data, replace_mapping_data, validate_mapping_overrides,
    validate_mapping_overrides_complete,
};
use crate::transport::{MqRestTransport, ReqwestTransport};

/// Default response parameters for DISPLAY commands.
pub const DEFAULT_RESPONSE_PARAMETERS: &[&str] = &["all"];

/// Default CSRF token value.
pub const DEFAULT_CSRF_TOKEN: &str = "local";

const GATEWAY_HEADER: &str = "ibm-mq-rest-gateway-qmgr";
const ERROR_INVALID_JSON: &str = "Response body was not valid JSON.";
const ERROR_NON_OBJECT_RESPONSE: &str = "Response payload was not a JSON object.";
const ERROR_COMMAND_RESPONSE_NOT_LIST: &str = "Response commandResponse was not a list.";
const ERROR_COMMAND_RESPONSE_ITEM_NOT_OBJECT: &str =
    "Response commandResponse item was not an object.";

/// Default MQSC qualifier fallback mappings.
fn default_mapping_qualifiers() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("QUEUE", "queue");
    m.insert("QLOCAL", "queue");
    m.insert("QREMOTE", "queue");
    m.insert("QALIAS", "queue");
    m.insert("QMODEL", "queue");
    m.insert("QMSTATUS", "qmstatus");
    m.insert("QSTATUS", "qstatus");
    m.insert("CHANNEL", "channel");
    m.insert("QMGR", "qmgr");
    m
}

/// Builder for constructing an [`MqRestSession`].
pub struct MqRestSessionBuilder {
    rest_base_url: String,
    qmgr_name: String,
    credentials: Credentials,
    gateway_qmgr: Option<String>,
    verify_tls: bool,
    timeout_seconds: Option<f64>,
    map_attributes: bool,
    mapping_strict: bool,
    mapping_overrides: Option<Value>,
    mapping_overrides_mode: MappingOverrideMode,
    csrf_token: Option<String>,
    transport: Option<Box<dyn MqRestTransport>>,
}

impl MqRestSessionBuilder {
    /// Create a new builder with required parameters.
    #[must_use]
    pub fn new(
        rest_base_url: impl Into<String>,
        qmgr_name: impl Into<String>,
        credentials: Credentials,
    ) -> Self {
        Self {
            rest_base_url: rest_base_url.into(),
            qmgr_name: qmgr_name.into(),
            credentials,
            gateway_qmgr: None,
            verify_tls: true,
            timeout_seconds: Some(30.0),
            map_attributes: true,
            mapping_strict: true,
            mapping_overrides: None,
            mapping_overrides_mode: MappingOverrideMode::Merge,
            csrf_token: Some(DEFAULT_CSRF_TOKEN.into()),
            transport: None,
        }
    }

    /// Set the gateway queue manager name.
    #[must_use]
    pub fn gateway_qmgr(mut self, name: impl Into<String>) -> Self {
        self.gateway_qmgr = Some(name.into());
        self
    }

    /// Set whether to verify TLS certificates.
    #[must_use]
    pub const fn verify_tls(mut self, verify: bool) -> Self {
        self.verify_tls = verify;
        self
    }

    /// Set the HTTP request timeout in seconds.
    #[must_use]
    pub const fn timeout_seconds(mut self, timeout: Option<f64>) -> Self {
        self.timeout_seconds = timeout;
        self
    }

    /// Set whether to map attributes between `snake_case` and MQSC names.
    #[must_use]
    pub const fn map_attributes(mut self, enabled: bool) -> Self {
        self.map_attributes = enabled;
        self
    }

    /// Set whether mapping failures are strict errors.
    #[must_use]
    pub const fn mapping_strict(mut self, strict: bool) -> Self {
        self.mapping_strict = strict;
        self
    }

    /// Set mapping overrides.
    #[must_use]
    pub fn mapping_overrides(mut self, overrides: Value) -> Self {
        self.mapping_overrides = Some(overrides);
        self
    }

    /// Set the mapping overrides mode.
    #[must_use]
    pub const fn mapping_overrides_mode(mut self, mode: MappingOverrideMode) -> Self {
        self.mapping_overrides_mode = mode;
        self
    }

    /// Set the CSRF token value.
    #[must_use]
    pub fn csrf_token(mut self, token: Option<String>) -> Self {
        self.csrf_token = token;
        self
    }

    /// Set a custom transport implementation.
    #[must_use]
    pub fn transport(mut self, transport: Box<dyn MqRestTransport>) -> Self {
        self.transport = Some(transport);
        self
    }

    /// Build the session, performing LTPA login if needed.
    ///
    /// # Errors
    ///
    /// Returns an error if mapping override validation fails, certificate files
    /// cannot be read, or LTPA login fails.
    pub fn build(self) -> Result<MqRestSession> {
        let rest_base_url = self.rest_base_url.trim_end_matches('/').to_owned();

        let mapping_data = if let Some(ref overrides) = self.mapping_overrides {
            validate_mapping_overrides(overrides).map_err(|msg| MqRestError::Response {
                message: msg,
                response_text: None,
            })?;
            if self.mapping_overrides_mode == MappingOverrideMode::Replace {
                validate_mapping_overrides_complete(&MAPPING_DATA, overrides).map_err(|msg| {
                    MqRestError::Response {
                        message: msg,
                        response_text: None,
                    }
                })?;
                replace_mapping_data(overrides)
            } else {
                merge_mapping_data(&MAPPING_DATA, overrides)
            }
        } else {
            MAPPING_DATA.clone()
        };

        let transport: Box<dyn MqRestTransport> = if let Some(t) = self.transport {
            t
        } else if let Credentials::Certificate {
            ref cert_path,
            ref key_path,
        } = self.credentials
        {
            let cert_pem = std::fs::read(cert_path).map_err(|_| MqRestError::Response {
                message: format!("Failed to read certificate file: {cert_path}"),
                response_text: None,
            })?;
            let key_pem = key_path
                .as_ref()
                .map(|p| {
                    std::fs::read(p).map_err(|_| MqRestError::Response {
                        message: format!("Failed to read key file: {p}"),
                        response_text: None,
                    })
                })
                .transpose()?;
            Box::new(ReqwestTransport::new_with_cert(
                &cert_pem,
                key_pem.as_deref(),
            )?)
        } else if self.verify_tls {
            Box::new(ReqwestTransport::new())
        } else {
            Box::new(ReqwestTransport::new_insecure())
        };

        let (ltpa_cookie_name, ltpa_token) = if let Credentials::Ltpa {
            ref username,
            ref password,
        } = self.credentials
        {
            let (name, token) = perform_ltpa_login(
                transport.as_ref(),
                &rest_base_url,
                username,
                password,
                self.csrf_token.as_deref(),
                self.timeout_seconds,
                self.verify_tls,
            )?;
            (Some(name), Some(token))
        } else {
            (None, None)
        };

        Ok(MqRestSession {
            rest_base_url,
            qmgr_name: self.qmgr_name,
            gateway_qmgr: self.gateway_qmgr,
            verify_tls: self.verify_tls,
            timeout_seconds: self.timeout_seconds,
            map_attributes: self.map_attributes,
            mapping_strict: self.mapping_strict,
            csrf_token: self.csrf_token,
            credentials: self.credentials,
            mapping_data,
            transport,
            ltpa_cookie_name,
            ltpa_token,
            last_response_payload: None,
            last_response_text: None,
            last_http_status: None,
            last_command_payload: None,
        })
    }
}

/// Session wrapper for MQ REST admin calls.
pub struct MqRestSession {
    rest_base_url: String,
    qmgr_name: String,
    gateway_qmgr: Option<String>,
    verify_tls: bool,
    timeout_seconds: Option<f64>,
    map_attributes: bool,
    mapping_strict: bool,
    csrf_token: Option<String>,
    credentials: Credentials,
    mapping_data: Value,
    transport: Box<dyn MqRestTransport>,
    ltpa_cookie_name: Option<String>,
    ltpa_token: Option<String>,

    /// The parsed JSON payload from the most recent command.
    pub last_response_payload: Option<HashMap<String, Value>>,
    /// The raw HTTP response body from the most recent command.
    pub last_response_text: Option<String>,
    /// The HTTP status code from the most recent command.
    pub last_http_status: Option<u16>,
    /// The `runCommandJSON` request payload sent for the most recent command.
    pub last_command_payload: Option<HashMap<String, Value>>,
}

impl MqRestSession {
    /// Create a builder for constructing a session.
    #[must_use]
    pub fn builder(
        rest_base_url: impl Into<String>,
        qmgr_name: impl Into<String>,
        credentials: Credentials,
    ) -> MqRestSessionBuilder {
        MqRestSessionBuilder::new(rest_base_url, qmgr_name, credentials)
    }

    /// The queue manager name this session targets.
    #[must_use]
    pub fn qmgr_name(&self) -> &str {
        &self.qmgr_name
    }

    /// The gateway queue manager name, or `None` for direct access.
    #[must_use]
    pub fn gateway_qmgr(&self) -> Option<&str> {
        self.gateway_qmgr.as_deref()
    }

    /// Core MQSC command dispatch method.
    ///
    /// # Errors
    ///
    /// Returns an error if attribute mapping, HTTP transport, response parsing,
    /// or the MQ command itself fails.
    pub(crate) fn mqsc_command(
        &mut self,
        command: &str,
        mqsc_qualifier: &str,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        let command_upper = command.trim().to_uppercase();
        let qualifier_upper = mqsc_qualifier.trim().to_uppercase();
        let mut normalized_request_parameters: HashMap<String, Value> =
            request_parameters.cloned().unwrap_or_default();
        let mut normalized_response_parameters =
            normalize_response_parameters(response_parameters, command_upper == "DISPLAY");
        let do_map = self.map_attributes;
        let mapping_qualifier = self.resolve_mapping_qualifier(&command_upper, &qualifier_upper);

        if do_map {
            normalized_request_parameters = map_request_attributes(
                &mapping_qualifier,
                &normalized_request_parameters,
                self.mapping_strict,
                Some(&self.mapping_data),
            )?;
            normalized_response_parameters = self.map_response_parameters(
                &command_upper,
                &qualifier_upper,
                &mapping_qualifier,
                &normalized_response_parameters,
            )?;
        }

        if let Some(where_str) = where_clause {
            let trimmed = where_str.trim();
            if !trimmed.is_empty() {
                let mapped_where = if do_map {
                    map_where_keyword(
                        trimmed,
                        &mapping_qualifier,
                        self.mapping_strict,
                        &self.mapping_data,
                    )?
                } else {
                    trimmed.to_owned()
                };
                normalized_request_parameters.insert("WHERE".into(), Value::String(mapped_where));
            }
        }

        let payload = build_command_payload(
            &command_upper,
            &qualifier_upper,
            name,
            &normalized_request_parameters,
            &normalized_response_parameters,
        );
        self.last_command_payload = Some(payload.clone());

        let headers = self.build_headers();
        let url = self.build_mqsc_url();
        let transport_response = self.transport.post_json(
            &url,
            &payload,
            &headers,
            self.timeout_seconds,
            self.verify_tls,
        )?;

        self.last_http_status = Some(transport_response.status_code);
        self.last_response_text = Some(transport_response.text.clone());

        let response_payload = parse_response_payload(&transport_response.text)?;
        self.last_response_payload = Some(response_payload.clone());

        raise_for_command_errors(&response_payload, transport_response.status_code)?;

        let command_response = extract_command_response(&response_payload)?;
        let mut parameter_objects: Vec<HashMap<String, Value>> = Vec::new();
        for item in &command_response {
            if let Some(parameters) = item.get("parameters").and_then(Value::as_object) {
                let map: HashMap<String, Value> = parameters
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                parameter_objects.push(map);
            } else {
                parameter_objects.push(HashMap::new());
            }
        }

        parameter_objects = flatten_nested_objects(parameter_objects);

        if do_map {
            let normalized: Vec<HashMap<String, Value>> = parameter_objects
                .iter()
                .map(normalize_response_attributes)
                .collect();
            Ok(map_response_list(
                &mapping_qualifier,
                &normalized,
                self.mapping_strict,
                Some(&self.mapping_data),
            )?)
        } else {
            Ok(parameter_objects)
        }
    }

    fn build_mqsc_url(&self) -> String {
        format!(
            "{}/admin/action/qmgr/{}/mqsc",
            self.rest_base_url, self.qmgr_name
        )
    }

    fn build_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Accept".into(), "application/json".into());
        match &self.credentials {
            Credentials::Basic { username, password } => {
                headers.insert(
                    "Authorization".into(),
                    build_basic_auth_header(username, password),
                );
            }
            Credentials::Ltpa { .. } => {
                if let (Some(name), Some(token)) = (&self.ltpa_cookie_name, &self.ltpa_token) {
                    headers.insert("Cookie".into(), format!("{name}={token}"));
                }
            }
            Credentials::Certificate { .. } => {}
        }
        if let Some(ref token) = self.csrf_token {
            headers.insert("ibm-mq-rest-csrf-token".into(), token.clone());
        }
        if let Some(ref gw) = self.gateway_qmgr {
            headers.insert(GATEWAY_HEADER.into(), gw.clone());
        }
        headers
    }

    fn map_response_parameters(
        &self,
        command: &str,
        mqsc_qualifier: &str,
        mapping_qualifier: &str,
        response_parameters: &[String],
    ) -> std::result::Result<Vec<String>, MappingError> {
        if is_all_response_parameters(response_parameters) {
            return Ok(response_parameters.to_vec());
        }
        let macros = get_response_parameter_macros(command, mqsc_qualifier, &self.mapping_data);
        let macro_lookup: HashMap<String, String> = macros
            .iter()
            .map(|m| (m.to_lowercase(), m.clone()))
            .collect();
        let qualifier_entry = get_qualifier_entry(mapping_qualifier, &self.mapping_data);
        let Some(entry) = qualifier_entry else {
            if self.mapping_strict {
                return Err(MappingError::new(build_unknown_qualifier_issue(
                    mapping_qualifier,
                )));
            }
            return Ok(response_parameters.to_vec());
        };
        let combined_map = build_snake_to_mqsc_map(entry);
        let (mapped, issues) = map_response_parameter_names(
            response_parameters,
            &macro_lookup,
            &combined_map,
            mapping_qualifier,
        );
        if self.mapping_strict && !issues.is_empty() {
            return Err(MappingError::new(issues));
        }
        Ok(mapped)
    }

    fn resolve_mapping_qualifier(&self, command: &str, mqsc_qualifier: &str) -> String {
        let command_map = get_command_map(&self.mapping_data);
        let command_key = format!("{command} {mqsc_qualifier}");
        if let Some(def) = command_map.get(&command_key).and_then(Value::as_object)
            && let Some(qualifier) = def.get("qualifier").and_then(Value::as_str)
        {
            return qualifier.to_owned();
        }
        let defaults = default_mapping_qualifiers();
        if let Some(fallback) = defaults.get(mqsc_qualifier) {
            return (*fallback).to_owned();
        }
        mqsc_qualifier.to_lowercase()
    }
}

fn build_basic_auth_header(username: &str, password: &str) -> String {
    let token = BASE64.encode(format!("{username}:{password}"));
    format!("Basic {token}")
}

fn build_command_payload(
    command: &str,
    qualifier: &str,
    name: Option<&str>,
    request_parameters: &HashMap<String, Value>,
    response_parameters: &[String],
) -> HashMap<String, Value> {
    let mut payload = HashMap::new();
    payload.insert("type".into(), Value::String("runCommandJSON".into()));
    payload.insert("command".into(), Value::String(command.into()));
    payload.insert("qualifier".into(), Value::String(qualifier.into()));
    if let Some(n) = name
        && !n.is_empty()
    {
        payload.insert("name".into(), Value::String(n.into()));
    }
    if !request_parameters.is_empty() {
        payload.insert(
            "parameters".into(),
            serde_json::to_value(request_parameters).unwrap(),
        );
    }
    if !response_parameters.is_empty() {
        let params: Vec<Value> = response_parameters
            .iter()
            .map(|s| Value::String(s.clone()))
            .collect();
        payload.insert("responseParameters".into(), Value::Array(params));
    }
    payload
}

fn normalize_response_parameters(
    response_parameters: Option<&[&str]>,
    is_display: bool,
) -> Vec<String> {
    let Some(params) = response_parameters else {
        return if is_display {
            DEFAULT_RESPONSE_PARAMETERS
                .iter()
                .map(|s| (*s).to_owned())
                .collect()
        } else {
            Vec::new()
        };
    };
    let normalized: Vec<String> = params.iter().map(|s| (*s).to_owned()).collect();
    if is_all_response_parameters(&normalized) {
        DEFAULT_RESPONSE_PARAMETERS
            .iter()
            .map(|s| (*s).to_owned())
            .collect()
    } else {
        normalized
    }
}

fn is_all_response_parameters(params: &[String]) -> bool {
    params.iter().any(|p| p.eq_ignore_ascii_case("all"))
}

fn flatten_nested_objects(
    parameter_objects: Vec<HashMap<String, Value>>,
) -> Vec<HashMap<String, Value>> {
    let mut flattened = Vec::new();
    for item in parameter_objects {
        if let Some(Value::Array(objects)) = item.get("objects") {
            let shared: HashMap<String, Value> = item
                .iter()
                .filter(|(k, _)| k.as_str() != "objects")
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            for nested in objects {
                if let Some(obj) = nested.as_object() {
                    let mut merged = shared.clone();
                    for (key, value) in obj {
                        merged.insert(key.clone(), value.clone());
                    }
                    flattened.push(merged);
                }
            }
        } else {
            flattened.push(item);
        }
    }
    flattened
}

fn normalize_response_attributes(attributes: &HashMap<String, Value>) -> HashMap<String, Value> {
    attributes
        .iter()
        .map(|(k, v)| (k.to_uppercase(), v.clone()))
        .collect()
}

fn parse_response_payload(response_text: &str) -> Result<HashMap<String, Value>> {
    let decoded: Value =
        serde_json::from_str(response_text).map_err(|_| MqRestError::Response {
            message: ERROR_INVALID_JSON.into(),
            response_text: Some(response_text.to_owned()),
        })?;
    match decoded {
        Value::Object(map) => Ok(map.into_iter().collect()),
        _ => Err(MqRestError::Response {
            message: ERROR_NON_OBJECT_RESPONSE.into(),
            response_text: Some(response_text.to_owned()),
        }),
    }
}

fn extract_command_response(
    payload: &HashMap<String, Value>,
) -> Result<Vec<HashMap<String, Value>>> {
    let Some(cr) = payload.get("commandResponse") else {
        return Ok(Vec::new());
    };
    let arr = cr.as_array().ok_or_else(|| MqRestError::Response {
        message: ERROR_COMMAND_RESPONSE_NOT_LIST.into(),
        response_text: None,
    })?;
    let mut items = Vec::new();
    for item in arr {
        let obj = item.as_object().ok_or_else(|| MqRestError::Response {
            message: ERROR_COMMAND_RESPONSE_ITEM_NOT_OBJECT.into(),
            response_text: None,
        })?;
        items.push(obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect());
    }
    Ok(items)
}

fn raise_for_command_errors(payload: &HashMap<String, Value>, status_code: u16) -> Result<()> {
    let completion_code = extract_optional_i64(payload.get("overallCompletionCode"));
    let reason_code = extract_optional_i64(payload.get("overallReasonCode"));
    let has_overall_error = has_error_codes(completion_code, reason_code);

    let mut command_issues: Vec<String> = Vec::new();
    if let Some(Value::Array(cr)) = payload.get("commandResponse") {
        for (idx, item) in cr.iter().enumerate() {
            if let Some(obj) = item.as_object() {
                let completion_code = extract_optional_i64(obj.get("completionCode"));
                let reason_code = extract_optional_i64(obj.get("reasonCode"));
                if has_error_codes(completion_code, reason_code) {
                    command_issues.push(format!(
                        "index={idx} completionCode={} reasonCode={}",
                        completion_code.unwrap_or(0),
                        reason_code.unwrap_or(0),
                    ));
                }
            }
        }
    }

    if has_overall_error || !command_issues.is_empty() {
        let mut lines = vec!["MQ REST command failed.".to_owned()];
        if has_overall_error {
            lines.push(format!(
                "overallCompletionCode={} overallReasonCode={}",
                completion_code.unwrap_or(0),
                reason_code.unwrap_or(0),
            ));
        }
        if !command_issues.is_empty() {
            lines.push("commandResponse:".into());
            lines.extend(command_issues);
        }
        return Err(MqRestError::Command {
            payload: payload.clone(),
            status_code: Some(status_code),
            message: lines.join("\n"),
        });
    }
    Ok(())
}

fn extract_optional_i64(value: Option<&Value>) -> Option<i64> {
    value.and_then(Value::as_i64)
}

const fn has_error_codes(completion_code: Option<i64>, reason_code: Option<i64>) -> bool {
    matches!(completion_code, Some(c) if c != 0) || matches!(reason_code, Some(r) if r != 0)
}

fn get_command_map(mapping_data: &Value) -> HashMap<String, Value> {
    mapping_data
        .get("commands")
        .and_then(Value::as_object)
        .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
        .unwrap_or_default()
}

fn get_response_parameter_macros(
    command: &str,
    mqsc_qualifier: &str,
    mapping_data: &Value,
) -> Vec<String> {
    let command_key = format!("{command} {mqsc_qualifier}");
    let entry = mapping_data
        .get("commands")
        .and_then(|c| c.get(&command_key));
    let Some(entry) = entry.and_then(Value::as_object) else {
        return Vec::new();
    };
    let Some(macros) = entry
        .get("response_parameter_macros")
        .and_then(Value::as_array)
    else {
        return Vec::new();
    };
    macros
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
}

fn build_unknown_qualifier_issue(qualifier: &str) -> Vec<MappingIssue> {
    vec![MappingIssue {
        direction: "request".into(),
        reason: "unknown_qualifier".into(),
        attribute_name: "*".into(),
        attribute_value: None,
        object_index: None,
        qualifier: Some(qualifier.into()),
    }]
}

fn build_snake_to_mqsc_map(qualifier_entry: &Value) -> HashMap<String, String> {
    let mut response_lookup: HashMap<String, String> = HashMap::new();
    if let Some(response_key_map) = qualifier_entry
        .get("response_key_map")
        .and_then(Value::as_object)
    {
        for (mqsc_key, snake_val) in response_key_map {
            if let Some(snake_key) = snake_val.as_str() {
                response_lookup
                    .entry(snake_key.to_owned())
                    .or_insert_with(|| mqsc_key.clone());
            }
        }
    }
    let mut combined = response_lookup;
    if let Some(request_key_map) = qualifier_entry
        .get("request_key_map")
        .and_then(Value::as_object)
    {
        for (snake_key, mqsc_val) in request_key_map {
            if let Some(mqsc_key) = mqsc_val.as_str() {
                combined.insert(snake_key.clone(), mqsc_key.to_owned());
            }
        }
    }
    combined
}

fn map_where_keyword(
    where_str: &str,
    mapping_qualifier: &str,
    strict: bool,
    mapping_data: &Value,
) -> std::result::Result<String, MappingError> {
    let parts: Vec<&str> = where_str.splitn(2, char::is_whitespace).collect();
    let keyword = parts[0];
    let rest = if parts.len() > 1 { parts[1] } else { "" };

    let qualifier_entry = get_qualifier_entry(mapping_qualifier, mapping_data);
    let Some(entry) = qualifier_entry else {
        if strict {
            return Err(MappingError::new(build_unknown_qualifier_issue(
                mapping_qualifier,
            )));
        }
        return Ok(where_str.to_owned());
    };

    let combined_map = build_snake_to_mqsc_map(entry);
    let mapped_keyword = if let Some(mqsc_key) = combined_map.get(keyword) {
        mqsc_key.clone()
    } else {
        if strict {
            return Err(MappingError::new(vec![MappingIssue {
                direction: "request".into(),
                reason: "unknown_key".into(),
                attribute_name: keyword.into(),
                attribute_value: None,
                object_index: None,
                qualifier: Some(mapping_qualifier.into()),
            }]));
        }
        keyword.to_owned()
    };

    if rest.is_empty() {
        Ok(mapped_keyword)
    } else {
        Ok(format!("{mapped_keyword} {rest}"))
    }
}

fn map_response_parameter_names(
    response_parameters: &[String],
    macro_lookup: &HashMap<String, String>,
    combined_map: &HashMap<String, String>,
    mapping_qualifier: &str,
) -> (Vec<String>, Vec<MappingIssue>) {
    let mut mapped = Vec::new();
    let mut issues = Vec::new();
    for name in response_parameters {
        if let Some(macro_key) = macro_lookup.get(&name.to_lowercase()) {
            mapped.push(macro_key.clone());
            continue;
        }
        if let Some(mapped_key) = combined_map.get(name.as_str()) {
            mapped.push(mapped_key.clone());
        } else {
            issues.push(MappingIssue {
                direction: "request".into(),
                reason: "unknown_key".into(),
                attribute_name: name.clone(),
                attribute_value: None,
                object_index: None,
                qualifier: Some(mapping_qualifier.into()),
            });
            mapped.push(name.clone());
        }
    }
    (mapped, issues)
}

fn get_qualifier_entry<'a>(qualifier: &str, mapping_data: &'a Value) -> Option<&'a Value> {
    mapping_data
        .get("qualifiers")
        .and_then(|q| q.get(qualifier))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{
        MockTransport, command_error_response, empty_success_response, error_response,
        mock_session, mock_session_with_mapping, success_response,
    };
    use crate::transport::TransportResponse;
    use serde_json::json;

    // =====================================================================
    // Builder tests
    // =====================================================================

    #[test]
    fn builder_basic_auth() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2/",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .transport(Box::new(transport))
        .build()
        .unwrap();
        assert_eq!(session.qmgr_name(), "QM1");
        assert!(session.gateway_qmgr().is_none());
    }

    #[test]
    fn builder_gateway_qmgr() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .gateway_qmgr("GW1")
        .transport(Box::new(transport))
        .build()
        .unwrap();
        assert_eq!(session.gateway_qmgr(), Some("GW1"));
    }

    #[test]
    fn builder_ltpa_credentials_performs_login() {
        let mut headers = HashMap::new();
        headers.insert("Set-Cookie".into(), "LtpaToken2=tok; Path=/".into());
        let login_response = TransportResponse {
            status_code: 200,
            text: "{}".into(),
            headers,
        };
        let transport = MockTransport::new(vec![login_response]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Ltpa {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .transport(Box::new(transport))
        .build()
        .unwrap();
        assert_eq!(session.ltpa_cookie_name.as_deref(), Some("LtpaToken2"));
        assert_eq!(session.ltpa_token.as_deref(), Some("tok"));
    }

    #[test]
    fn builder_mapping_overrides_merge() {
        let transport = MockTransport::new(vec![]);
        let overrides = json!({"commands": {}, "qualifiers": {}});
        let _session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .mapping_overrides(overrides)
        .mapping_overrides_mode(MappingOverrideMode::Merge)
        .transport(Box::new(transport))
        .build()
        .unwrap();
    }

    #[test]
    fn builder_invalid_overrides() {
        let transport = MockTransport::new(vec![]);
        let overrides = json!("not an object");
        let result = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .mapping_overrides(overrides)
        .transport(Box::new(transport))
        .build();
        assert!(result.is_err());
    }

    #[test]
    fn builder_fluent_setters() {
        let transport = MockTransport::new(vec![]);
        let _session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .verify_tls(false)
        .timeout_seconds(Some(60.0))
        .map_attributes(false)
        .mapping_strict(false)
        .csrf_token(None)
        .transport(Box::new(transport))
        .build()
        .unwrap();
    }

    // =====================================================================
    // mqsc_command tests
    // =====================================================================

    #[test]
    fn mqsc_command_basic_display() {
        let mut params = HashMap::new();
        params.insert("DESCR".into(), json!("test"));
        let transport = MockTransport::new(vec![success_response(vec![params])]);
        let mut session = mock_session(transport);
        let result = session
            .mqsc_command("DISPLAY", "QUEUE", Some("Q1"), None, None, None)
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["DESCR"], json!("test"));
    }

    #[test]
    fn mqsc_command_with_mapping() {
        let mut params = HashMap::new();
        params.insert("DESCR".into(), json!("test"));
        let transport = MockTransport::new(vec![success_response(vec![params])]);
        let mut session = mock_session_with_mapping(transport);
        let result = session
            .mqsc_command("DISPLAY", "QUEUE", Some("Q1"), None, None, None)
            .unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].contains_key("description"));
    }

    #[test]
    fn mqsc_command_non_display_default_params() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = mock_session(transport);
        session
            .mqsc_command("ALTER", "QMGR", None, None, None, None)
            .unwrap();
        let payload = session.last_command_payload.unwrap();
        assert!(!payload.contains_key("responseParameters"));
    }

    #[test]
    fn mqsc_command_display_default_params() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = mock_session(transport);
        session
            .mqsc_command("DISPLAY", "QMGR", None, None, None, None)
            .unwrap();
        let payload = session.last_command_payload.unwrap();
        assert!(payload.contains_key("responseParameters"));
    }

    #[test]
    fn mqsc_command_where_clause() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = mock_session(transport);
        session
            .mqsc_command(
                "DISPLAY",
                "QUEUE",
                Some("*"),
                None,
                None,
                Some("DESCR LK test*"),
            )
            .unwrap();
        let payload = session.last_command_payload.unwrap();
        let params = payload["parameters"].as_object().unwrap();
        assert!(params.contains_key("WHERE"));
    }

    #[test]
    fn mqsc_command_where_clause_with_mapping() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = mock_session_with_mapping(transport);
        session
            .mqsc_command(
                "DISPLAY",
                "QUEUE",
                Some("*"),
                None,
                None,
                Some("description LK test*"),
            )
            .unwrap();
        let payload = session.last_command_payload.unwrap();
        let params = payload["parameters"].as_object().unwrap();
        let where_val = params["WHERE"].as_str().unwrap();
        assert!(where_val.starts_with("DESCR"));
    }

    #[test]
    fn mqsc_command_empty_where_clause_ignored() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = mock_session(transport);
        session
            .mqsc_command("DISPLAY", "QUEUE", Some("*"), None, None, Some("  "))
            .unwrap();
        let payload = session.last_command_payload.unwrap();
        assert!(!payload.contains_key("parameters"));
    }

    #[test]
    fn mqsc_command_transport_error() {
        let transport = MockTransport::new(vec![]);
        let mut session = mock_session(transport);
        let result = session.mqsc_command("DISPLAY", "QMGR", None, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn mqsc_command_invalid_json() {
        let transport = MockTransport::new(vec![TransportResponse {
            status_code: 200,
            text: "not json".into(),
            headers: HashMap::new(),
        }]);
        let mut session = mock_session(transport);
        let result = session.mqsc_command("DISPLAY", "QMGR", None, None, None, None);
        assert!(format!("{:?}", result.unwrap_err()).starts_with("Response"));
    }

    #[test]
    fn mqsc_command_non_object_json() {
        let transport = MockTransport::new(vec![TransportResponse {
            status_code: 200,
            text: "[1,2,3]".into(),
            headers: HashMap::new(),
        }]);
        let mut session = mock_session(transport);
        let result = session.mqsc_command("DISPLAY", "QMGR", None, None, None, None);
        assert!(format!("{:?}", result.unwrap_err()).starts_with("Response"));
    }

    #[test]
    fn mqsc_command_error_response() {
        let transport = MockTransport::new(vec![error_response(2, 3008)]);
        let mut session = mock_session(transport);
        let result = session.mqsc_command("DISPLAY", "QMGR", None, None, None, None);
        assert!(format!("{:?}", result.unwrap_err()).starts_with("Command"));
    }

    #[test]
    fn mqsc_command_command_response_not_list() {
        let body = json!({
            "overallCompletionCode": 0,
            "overallReasonCode": 0,
            "commandResponse": "not_a_list"
        });
        let transport = MockTransport::new(vec![TransportResponse {
            status_code: 200,
            text: body.to_string(),
            headers: HashMap::new(),
        }]);
        let mut session = mock_session(transport);
        let result = session.mqsc_command("DISPLAY", "QMGR", None, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn mqsc_command_empty_command_response() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = mock_session(transport);
        let result = session
            .mqsc_command("DISPLAY", "QMGR", None, None, None, None)
            .unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn mqsc_command_nested_objects_flattened() {
        let body = json!({
            "overallCompletionCode": 0,
            "overallReasonCode": 0,
            "commandResponse": [{
                "completionCode": 0,
                "reasonCode": 0,
                "parameters": {
                    "shared_key": "shared_val",
                    "objects": [
                        {"nested_key": "val1"},
                        {"nested_key": "val2"}
                    ]
                }
            }]
        });
        let transport = MockTransport::new(vec![TransportResponse {
            status_code: 200,
            text: body.to_string(),
            headers: HashMap::new(),
        }]);
        let mut session = mock_session(transport);
        let result = session
            .mqsc_command("DISPLAY", "QUEUE", Some("*"), None, None, None)
            .unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0]["shared_key"], json!("shared_val"));
        assert_eq!(result[0]["nested_key"], json!("val1"));
        assert_eq!(result[1]["nested_key"], json!("val2"));
    }

    #[test]
    fn mqsc_command_with_request_parameters() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = mock_session(transport);
        let mut req_params = HashMap::new();
        req_params.insert("FORCE".into(), json!("YES"));
        session
            .mqsc_command("ALTER", "QMGR", None, Some(&req_params), None, None)
            .unwrap();
        let payload = session.last_command_payload.unwrap();
        assert!(payload.contains_key("parameters"));
    }

    #[test]
    fn mqsc_command_with_explicit_response_parameters() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = mock_session(transport);
        let resp_params: &[&str] = &["DESCR", "MAXDEPTH"];
        session
            .mqsc_command("DISPLAY", "QUEUE", Some("*"), None, Some(resp_params), None)
            .unwrap();
        let payload = session.last_command_payload.unwrap();
        let rp = payload["responseParameters"].as_array().unwrap();
        assert_eq!(rp.len(), 2);
    }

    // =====================================================================
    // Private helper tests
    // =====================================================================

    #[test]
    fn build_headers_basic_auth() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "admin".into(),
                password: "secret".into(),
            },
        )
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let headers = session.build_headers();
        assert!(headers["Authorization"].starts_with("Basic "));
        assert!(headers.contains_key("ibm-mq-rest-csrf-token"));
    }

    #[test]
    fn build_headers_ltpa() {
        let mut login_headers = HashMap::new();
        login_headers.insert("Set-Cookie".into(), "LtpaToken2=tok; Path=/".into());
        let transport = MockTransport::new(vec![TransportResponse {
            status_code: 200,
            text: "{}".into(),
            headers: login_headers,
        }]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Ltpa {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let headers = session.build_headers();
        assert!(headers["Cookie"].contains("LtpaToken2=tok"));
    }

    #[test]
    fn build_headers_certificate() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Certificate {
                cert_path: "/fake".into(),
                key_path: None,
            },
        )
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let headers = session.build_headers();
        assert!(!headers.contains_key("Authorization"));
        assert!(!headers.contains_key("Cookie"));
    }

    #[test]
    fn build_headers_gateway() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .gateway_qmgr("GW1")
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let headers = session.build_headers();
        assert_eq!(headers["ibm-mq-rest-gateway-qmgr"], "GW1");
    }

    #[test]
    fn build_headers_no_csrf() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .csrf_token(None)
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let headers = session.build_headers();
        assert!(!headers.contains_key("ibm-mq-rest-csrf-token"));
    }

    #[test]
    fn build_command_payload_with_name() {
        let params = HashMap::new();
        let resp: Vec<String> = vec![];
        let payload = build_command_payload("DISPLAY", "QUEUE", Some("Q1"), &params, &resp);
        assert_eq!(payload["command"], json!("DISPLAY"));
        assert_eq!(payload["qualifier"], json!("QUEUE"));
        assert_eq!(payload["name"], json!("Q1"));
    }

    #[test]
    fn build_command_payload_without_name() {
        let params = HashMap::new();
        let resp: Vec<String> = vec![];
        let payload = build_command_payload("DISPLAY", "QMGR", None, &params, &resp);
        assert!(!payload.contains_key("name"));
    }

    #[test]
    fn build_command_payload_empty_name() {
        let params = HashMap::new();
        let resp: Vec<String> = vec![];
        let payload = build_command_payload("DISPLAY", "QMGR", Some(""), &params, &resp);
        assert!(!payload.contains_key("name"));
    }

    #[test]
    fn build_command_payload_with_params() {
        let mut params = HashMap::new();
        params.insert("FORCE".into(), json!("YES"));
        let resp: Vec<String> = vec!["DESCR".into()];
        let payload = build_command_payload("ALTER", "QMGR", None, &params, &resp);
        assert!(payload.contains_key("parameters"));
        assert!(payload.contains_key("responseParameters"));
    }

    #[test]
    fn normalize_response_parameters_none_display() {
        let result = normalize_response_parameters(None, true);
        assert_eq!(result, vec!["all".to_owned()]);
    }

    #[test]
    fn normalize_response_parameters_none_non_display() {
        let result = normalize_response_parameters(None, false);
        assert!(result.is_empty());
    }

    #[test]
    fn normalize_response_parameters_all() {
        let result = normalize_response_parameters(Some(&["ALL"]), false);
        assert_eq!(result, vec!["all".to_owned()]);
    }

    #[test]
    fn normalize_response_parameters_explicit() {
        let result = normalize_response_parameters(Some(&["DESCR", "MAXDEPTH"]), true);
        assert_eq!(result, vec!["DESCR".to_owned(), "MAXDEPTH".to_owned()]);
    }

    #[test]
    fn flatten_nested_objects_no_nesting() {
        let item = {
            let mut m = HashMap::new();
            m.insert("key".into(), json!("val"));
            m
        };
        let result = flatten_nested_objects(vec![item]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["key"], json!("val"));
    }

    #[test]
    fn flatten_nested_objects_with_nesting() {
        let item = {
            let mut m = HashMap::new();
            m.insert("shared".into(), json!("s"));
            m.insert("objects".into(), json!([{"a": 1}, {"b": 2}]));
            m
        };
        let result = flatten_nested_objects(vec![item]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0]["shared"], json!("s"));
        assert_eq!(result[0]["a"], json!(1));
        assert_eq!(result[1]["b"], json!(2));
    }

    #[test]
    fn parse_response_payload_valid() {
        let result = parse_response_payload(r#"{"key": "value"}"#).unwrap();
        assert_eq!(result["key"], json!("value"));
    }

    #[test]
    fn parse_response_payload_invalid_json() {
        let result = parse_response_payload("not json");
        assert!(result.is_err());
    }

    #[test]
    fn parse_response_payload_non_object() {
        let result = parse_response_payload("[1,2]");
        assert!(result.is_err());
    }

    #[test]
    fn extract_command_response_present() {
        let mut payload = HashMap::new();
        payload.insert(
            "commandResponse".into(),
            json!([{"completionCode": 0, "parameters": {}}]),
        );
        let result = extract_command_response(&payload).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn extract_command_response_missing() {
        let payload = HashMap::new();
        let result = extract_command_response(&payload).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn extract_command_response_not_list() {
        let mut payload = HashMap::new();
        payload.insert("commandResponse".into(), json!("string"));
        let result = extract_command_response(&payload);
        assert!(result.is_err());
    }

    #[test]
    fn extract_command_response_item_not_object() {
        let mut payload = HashMap::new();
        payload.insert("commandResponse".into(), json!([42]));
        let result = extract_command_response(&payload);
        assert!(result.is_err());
    }

    #[test]
    fn raise_for_command_errors_none() {
        let mut payload = HashMap::new();
        payload.insert("overallCompletionCode".into(), json!(0));
        payload.insert("overallReasonCode".into(), json!(0));
        assert!(raise_for_command_errors(&payload, 200).is_ok());
    }

    #[test]
    fn raise_for_command_errors_overall() {
        let mut payload = HashMap::new();
        payload.insert("overallCompletionCode".into(), json!(2));
        payload.insert("overallReasonCode".into(), json!(3008));
        assert!(
            format!("{:?}", raise_for_command_errors(&payload, 200).unwrap_err())
                .starts_with("Command")
        );
    }

    #[test]
    fn raise_for_command_errors_item_level() {
        let mut payload = HashMap::new();
        payload.insert("overallCompletionCode".into(), json!(0));
        payload.insert("overallReasonCode".into(), json!(0));
        payload.insert(
            "commandResponse".into(),
            json!([{"completionCode": 2, "reasonCode": 3008}]),
        );
        assert!(
            format!("{:?}", raise_for_command_errors(&payload, 200).unwrap_err())
                .starts_with("Command")
        );
    }

    #[test]
    fn raise_for_command_errors_both_overall_and_item() {
        let mut payload = HashMap::new();
        payload.insert("overallCompletionCode".into(), json!(2));
        payload.insert("overallReasonCode".into(), json!(3008));
        payload.insert(
            "commandResponse".into(),
            json!([{"completionCode": 2, "reasonCode": 3008}]),
        );
        let err_msg = format!("{}", raise_for_command_errors(&payload, 200).unwrap_err());
        assert!(err_msg.contains("overallCompletionCode"));
        assert!(err_msg.contains("commandResponse"));
    }

    #[test]
    fn map_where_keyword_with_known_key() {
        let data = json!({
            "qualifiers": {
                "queue": {
                    "request_key_map": {"description": "DESCR"},
                    "response_key_map": {}
                }
            }
        });
        let result = map_where_keyword("description LK test*", "queue", false, &data).unwrap();
        assert_eq!(result, "DESCR LK test*");
    }

    #[test]
    fn map_where_keyword_unknown_key_non_strict() {
        let data =
            json!({"qualifiers": {"queue": {"request_key_map": {}, "response_key_map": {}}}});
        let result = map_where_keyword("unknown_attr LK test*", "queue", false, &data).unwrap();
        assert_eq!(result, "unknown_attr LK test*");
    }

    #[test]
    fn map_where_keyword_unknown_key_strict() {
        let data =
            json!({"qualifiers": {"queue": {"request_key_map": {}, "response_key_map": {}}}});
        let result = map_where_keyword("unknown_attr LK test*", "queue", true, &data);
        assert!(result.is_err());
    }

    #[test]
    fn map_where_keyword_unknown_qualifier_non_strict() {
        let data = json!({"qualifiers": {}});
        let result = map_where_keyword("desc LK x", "noexist", false, &data).unwrap();
        assert_eq!(result, "desc LK x");
    }

    #[test]
    fn map_where_keyword_unknown_qualifier_strict() {
        let data = json!({"qualifiers": {}});
        let result = map_where_keyword("desc LK x", "noexist", true, &data);
        assert!(result.is_err());
    }

    #[test]
    fn map_where_keyword_no_rest() {
        let data = json!({
            "qualifiers": {
                "queue": {
                    "request_key_map": {"description": "DESCR"},
                    "response_key_map": {}
                }
            }
        });
        let result = map_where_keyword("description", "queue", false, &data).unwrap();
        assert_eq!(result, "DESCR");
    }

    #[test]
    fn resolve_mapping_qualifier_from_commands() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .transport(Box::new(transport))
        .build()
        .unwrap();
        // DISPLAY CHSTATUS should resolve to "chstatus" via commands map
        let qualifier = session.resolve_mapping_qualifier("DISPLAY", "CHSTATUS");
        assert_eq!(qualifier, "chstatus");
    }

    #[test]
    fn resolve_mapping_qualifier_default_fallback() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let qualifier = session.resolve_mapping_qualifier("DISPLAY", "QLOCAL");
        assert_eq!(qualifier, "queue");
    }

    #[test]
    fn resolve_mapping_qualifier_lowercase_fallback() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let qualifier = session.resolve_mapping_qualifier("DISPLAY", "UNKNOWNOBJ");
        assert_eq!(qualifier, "unknownobj");
    }

    #[test]
    fn last_response_fields_populated() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = mock_session(transport);
        session
            .mqsc_command("DISPLAY", "QMGR", None, None, None, None)
            .unwrap();
        assert!(session.last_response_payload.is_some());
        assert!(session.last_response_text.is_some());
        assert!(session.last_http_status.is_some());
        assert!(session.last_command_payload.is_some());
    }

    #[test]
    fn url_trailing_slash_stripped() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2/",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .map_attributes(false)
        .transport(Box::new(transport))
        .build()
        .unwrap();
        session
            .mqsc_command("DISPLAY", "QMGR", None, None, None, None)
            .unwrap();
        assert_eq!(session.last_http_status, Some(200));
    }

    #[test]
    fn command_response_item_without_parameters() {
        let body = json!({
            "overallCompletionCode": 0,
            "overallReasonCode": 0,
            "commandResponse": [{"completionCode": 0, "reasonCode": 0}]
        });
        let transport = MockTransport::new(vec![TransportResponse {
            status_code: 200,
            text: body.to_string(),
            headers: HashMap::new(),
        }]);
        let mut session = mock_session(transport);
        let result = session
            .mqsc_command("DISPLAY", "QMGR", None, None, None, None)
            .unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].is_empty());
    }

    #[test]
    fn command_error_response_test() {
        let transport = MockTransport::new(vec![command_error_response()]);
        let mut session = mock_session(transport);
        let result = session.mqsc_command("DISPLAY", "QUEUE", Some("Q1"), None, None, None);
        assert!(format!("{:?}", result.unwrap_err()).starts_with("Command"));
    }

    // =====================================================================
    // Replace mode and Certificate credential paths
    // =====================================================================

    #[test]
    fn builder_replace_mode_incomplete_errors() {
        let transport = MockTransport::new(vec![]);
        let overrides = json!({"commands": {}, "qualifiers": {}});
        let result = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .mapping_overrides(overrides)
        .mapping_overrides_mode(MappingOverrideMode::Replace)
        .transport(Box::new(transport))
        .build();
        assert!(result.is_err());
    }

    #[test]
    fn builder_certificate_no_transport_fails_missing_file() {
        let result = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Certificate {
                cert_path: "/nonexistent/cert.pem".into(),
                key_path: None,
            },
        )
        .build();
        assert!(result.is_err());
    }

    #[test]
    fn builder_certificate_missing_key_file() {
        // Create a temp cert file
        let cert_path = std::env::temp_dir().join("test_cert_session.pem");
        std::fs::write(&cert_path, b"fake-cert").unwrap();
        let result = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Certificate {
                cert_path: cert_path.to_str().unwrap().into(),
                key_path: Some("/nonexistent/key.pem".into()),
            },
        )
        .build();
        assert!(result.is_err());
        let _ = std::fs::remove_file(&cert_path);
    }

    #[test]
    fn builder_certificate_invalid_pem() {
        let cert_path = std::env::temp_dir().join("test_cert_session2.pem");
        std::fs::write(&cert_path, b"not-a-valid-pem").unwrap();
        let result = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Certificate {
                cert_path: cert_path.to_str().unwrap().into(),
                key_path: None,
            },
        )
        .build();
        assert!(result.is_err());
        let _ = std::fs::remove_file(&cert_path);
    }

    #[test]
    fn builder_certificate_valid_pem_no_transport() {
        let cert_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test-fixtures/test-combined.pem"
        );
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Certificate {
                cert_path: cert_path.into(),
                key_path: None,
            },
        )
        .build()
        .unwrap();
        assert_eq!(session.qmgr_name(), "QM1");
    }

    #[test]
    fn builder_verify_tls_true_default_transport() {
        // This path creates a ReqwestTransport::new() — just verify it doesn't panic
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .verify_tls(true)
        .build()
        .unwrap();
        assert_eq!(session.qmgr_name(), "QM1");
    }

    #[test]
    fn builder_verify_tls_false_default_transport() {
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .verify_tls(false)
        .build()
        .unwrap();
        assert_eq!(session.qmgr_name(), "QM1");
    }

    // =====================================================================
    // map_response_parameters tests
    // =====================================================================

    #[test]
    fn map_response_parameters_all_passthrough() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let params = vec!["all".to_owned()];
        let result = session
            .map_response_parameters("DISPLAY", "QUEUE", "queue", &params)
            .unwrap();
        assert_eq!(result, vec!["all".to_owned()]);
    }

    #[test]
    fn map_response_parameters_maps_snake_case() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let params = vec!["description".to_owned()];
        let result = session
            .map_response_parameters("DISPLAY", "QUEUE", "queue", &params)
            .unwrap();
        assert!(result.contains(&"DESCR".to_owned()));
    }

    #[test]
    fn map_response_parameters_unknown_qualifier_non_strict() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .mapping_strict(false)
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let params = vec!["foo".to_owned()];
        let result = session
            .map_response_parameters("DISPLAY", "NONEXIST", "nonexist", &params)
            .unwrap();
        assert_eq!(result, vec!["foo".to_owned()]);
    }

    #[test]
    fn map_response_parameters_unknown_qualifier_strict() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .mapping_strict(true)
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let params = vec!["foo".to_owned()];
        let result = session.map_response_parameters("DISPLAY", "NONEXIST", "nonexist", &params);
        assert!(result.is_err());
    }

    #[test]
    fn map_response_parameters_unknown_key_strict() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .mapping_strict(true)
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let params = vec!["unknown_snake_key".to_owned()];
        let result = session.map_response_parameters("DISPLAY", "QUEUE", "queue", &params);
        assert!(result.is_err());
    }

    #[test]
    fn map_response_parameters_macro_name() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .mapping_strict(false)
        .transport(Box::new(transport))
        .build()
        .unwrap();
        // Use a response parameter macro name if available for DISPLAY QUEUE
        let params = vec!["description".to_owned(), "max_queue_depth".to_owned()];
        let result = session
            .map_response_parameters("DISPLAY", "QUEUE", "queue", &params)
            .unwrap();
        assert_eq!(result.len(), 2);
    }

    // =====================================================================
    // mqsc_command with mapping — error propagation paths
    // =====================================================================

    #[test]
    fn mqsc_command_mapping_request_error_propagates() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .map_attributes(true)
        .mapping_strict(true)
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let mut params = HashMap::new();
        params.insert("totally_unknown_key".into(), json!("val"));
        let result = session.mqsc_command("DISPLAY", "QUEUE", Some("*"), Some(&params), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn mqsc_command_mapping_response_param_error_propagates() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .map_attributes(true)
        .mapping_strict(true)
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let resp_params: &[&str] = &["totally_unknown_snake_param"];
        let result =
            session.mqsc_command("DISPLAY", "QUEUE", Some("*"), None, Some(resp_params), None);
        assert!(result.is_err());
    }

    #[test]
    fn mqsc_command_mapping_response_list_error_propagates() {
        // Strict mode + unknown response key should propagate the mapping error
        let mut params = HashMap::new();
        params.insert("UNKNOWN_RESP_KEY".into(), json!("val"));
        let transport = MockTransport::new(vec![success_response(vec![params])]);
        let mut session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .map_attributes(true)
        .mapping_strict(true)
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let result = session.mqsc_command("DISPLAY", "QUEUE", Some("*"), None, None, None);
        assert!(result.is_err());
    }

    // =====================================================================
    // get_response_parameter_macros / build_snake_to_mqsc_map
    // =====================================================================

    #[test]
    fn get_response_parameter_macros_existing() {
        let data = &*MAPPING_DATA;
        let macros = get_response_parameter_macros("DISPLAY", "QUEUE", data);
        // DISPLAY QUEUE should have some macros in the mapping data
        let _ = macros.len(); // just exercises the code path
    }

    #[test]
    fn get_response_parameter_macros_missing_command() {
        let data = json!({"commands": {}});
        let macros = get_response_parameter_macros("DISPLAY", "NONEXIST", &data);
        assert!(macros.is_empty());
    }

    #[test]
    fn get_response_parameter_macros_no_macros_key() {
        let data = json!({"commands": {"DISPLAY QUEUE": {}}});
        let macros = get_response_parameter_macros("DISPLAY", "QUEUE", &data);
        assert!(macros.is_empty());
    }

    #[test]
    fn build_snake_to_mqsc_map_combines_both_maps() {
        let entry = json!({
            "request_key_map": {"snake_req": "MQSC_REQ"},
            "response_key_map": {"MQSC_RESP": "snake_resp"}
        });
        let result = build_snake_to_mqsc_map(&entry);
        assert_eq!(result.get("snake_req").unwrap(), "MQSC_REQ");
        assert_eq!(result.get("snake_resp").unwrap(), "MQSC_RESP");
    }

    #[test]
    fn build_snake_to_mqsc_map_empty() {
        let entry = json!({});
        let result = build_snake_to_mqsc_map(&entry);
        assert!(result.is_empty());
    }

    #[test]
    fn map_response_parameter_names_with_macros() {
        let mut macro_lookup = HashMap::new();
        macro_lookup.insert("events".into(), "EVENTS".into());
        let mut combined_map = HashMap::new();
        combined_map.insert("description".into(), "DESCR".into());
        let params = vec!["events".into(), "description".into(), "unknown".into()];
        let (mapped, issues) =
            map_response_parameter_names(&params, &macro_lookup, &combined_map, "queue");
        assert_eq!(mapped[0], "EVENTS");
        assert_eq!(mapped[1], "DESCR");
        assert_eq!(mapped[2], "unknown");
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn get_command_map_valid() {
        let data = json!({"commands": {"DISPLAY QUEUE": {"qualifier": "queue"}}});
        let result = get_command_map(&data);
        assert!(result.contains_key("DISPLAY QUEUE"));
    }

    #[test]
    fn get_command_map_missing() {
        let data = json!({});
        let result = get_command_map(&data);
        assert!(result.is_empty());
    }

    #[test]
    fn get_qualifier_entry_present() {
        let data = json!({"qualifiers": {"queue": {"request_key_map": {}}}});
        assert!(get_qualifier_entry("queue", &data).is_some());
    }

    #[test]
    fn get_qualifier_entry_missing() {
        let data = json!({"qualifiers": {}});
        assert!(get_qualifier_entry("nonexist", &data).is_none());
    }

    #[test]
    fn normalize_response_attributes_uppercases_keys() {
        let mut attrs = HashMap::new();
        attrs.insert("descr".into(), json!("test"));
        let result = normalize_response_attributes(&attrs);
        assert!(result.contains_key("DESCR"));
    }

    #[test]
    fn is_all_response_parameters_true() {
        assert!(is_all_response_parameters(&["ALL".to_owned()]));
        assert!(is_all_response_parameters(&["all".to_owned()]));
    }

    #[test]
    fn is_all_response_parameters_false() {
        assert!(!is_all_response_parameters(&["DESCR".to_owned()]));
        assert!(!is_all_response_parameters(&[]));
    }

    #[test]
    fn build_basic_auth_header_format() {
        let header = build_basic_auth_header("admin", "secret");
        assert!(header.starts_with("Basic "));
    }

    #[test]
    fn build_unknown_qualifier_issue_format() {
        let issues = build_unknown_qualifier_issue("test_q");
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].reason, "unknown_qualifier");
        assert_eq!(issues[0].qualifier, Some("test_q".into()));
    }

    #[test]
    fn mqsc_command_where_clause_mapping_error_strict() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .map_attributes(true)
        .mapping_strict(true)
        .transport(Box::new(transport))
        .build()
        .unwrap();
        let result = session.mqsc_command(
            "DISPLAY",
            "QUEUE",
            Some("*"),
            None,
            None,
            Some("totally_bogus_key LK test*"),
        );
        assert!(result.is_err());
    }

    #[test]
    fn flatten_nested_objects_non_object_in_array() {
        let item = {
            let mut m = HashMap::new();
            m.insert("shared".into(), json!("s"));
            m.insert("objects".into(), json!([42, {"a": 1}]));
            m
        };
        let result = flatten_nested_objects(vec![item]);
        // Only the object element is flattened, non-objects are skipped
        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["a"], json!(1));
    }

    #[test]
    fn extract_optional_i64_some() {
        assert_eq!(extract_optional_i64(Some(&json!(42))), Some(42));
    }

    #[test]
    fn extract_optional_i64_none() {
        assert_eq!(extract_optional_i64(None), None);
    }

    #[test]
    fn extract_optional_i64_non_number() {
        assert_eq!(extract_optional_i64(Some(&json!("not a number"))), None);
    }

    #[test]
    fn has_error_codes_both_zero() {
        assert!(!has_error_codes(Some(0), Some(0)));
    }

    #[test]
    fn has_error_codes_completion_nonzero() {
        assert!(has_error_codes(Some(2), Some(0)));
    }

    #[test]
    fn has_error_codes_reason_nonzero() {
        assert!(has_error_codes(Some(0), Some(3008)));
    }

    #[test]
    fn has_error_codes_both_none() {
        assert!(!has_error_codes(None, None));
    }

    #[test]
    fn builder_replace_mode_with_complete_overrides() {
        let base = &*MAPPING_DATA;
        let base_obj = base.as_object().unwrap();
        let commands = base_obj.get("commands").cloned().unwrap();
        let qualifiers = base_obj.get("qualifiers").cloned().unwrap();
        let overrides = json!({"commands": commands, "qualifiers": qualifiers});
        let transport = MockTransport::new(vec![]);
        let _session = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Basic {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .mapping_overrides(overrides)
        .mapping_overrides_mode(MappingOverrideMode::Replace)
        .transport(Box::new(transport))
        .build()
        .unwrap();
    }

    #[test]
    fn builder_ltpa_login_fails_propagates() {
        // LTPA login transport error
        let transport = MockTransport::new(vec![]);
        let result = MqRestSession::builder(
            "https://host/ibmmq/rest/v2",
            "QM1",
            Credentials::Ltpa {
                username: "u".into(),
                password: "p".into(),
            },
        )
        .transport(Box::new(transport))
        .build();
        assert!(result.is_err());
    }

    #[test]
    fn raise_for_command_errors_non_object_item_in_response() {
        let mut payload = HashMap::new();
        payload.insert("overallCompletionCode".into(), json!(0));
        payload.insert("overallReasonCode".into(), json!(0));
        payload.insert("commandResponse".into(), json!([42, "string"]));
        // Non-object items are skipped — no error
        assert!(raise_for_command_errors(&payload, 200).is_ok());
    }

    #[test]
    fn raise_for_command_errors_with_ok_items() {
        let mut payload = HashMap::new();
        payload.insert("overallCompletionCode".into(), json!(0));
        payload.insert("overallReasonCode".into(), json!(0));
        payload.insert(
            "commandResponse".into(),
            json!([
                {"completionCode": 0, "reasonCode": 0, "parameters": {"key": "val"}}
            ]),
        );
        assert!(raise_for_command_errors(&payload, 200).is_ok());
    }

    #[test]
    fn build_snake_to_mqsc_map_response_key_map_non_string_ignored() {
        let entry = json!({
            "response_key_map": {"MQSC": 42},
            "request_key_map": {}
        });
        let result = build_snake_to_mqsc_map(&entry);
        assert!(result.is_empty());
    }

    #[test]
    fn build_snake_to_mqsc_map_request_key_map_non_string_ignored() {
        let entry = json!({
            "response_key_map": {},
            "request_key_map": {"snake": 42}
        });
        let result = build_snake_to_mqsc_map(&entry);
        assert!(result.is_empty());
    }

    // =====================================================================
    // build_headers LTPA without token
    // =====================================================================

    #[test]
    fn build_headers_ltpa_no_token_omits_cookie() {
        let transport = MockTransport::new(vec![]);
        let session = MqRestSession {
            rest_base_url: "https://host/ibmmq/rest/v2".into(),
            qmgr_name: "QM1".into(),
            gateway_qmgr: None,
            verify_tls: true,
            timeout_seconds: None,
            map_attributes: false,
            mapping_strict: false,
            csrf_token: None,
            credentials: Credentials::Ltpa {
                username: "user".into(),
                password: "pass".into(),
            },
            mapping_data: json!({}),
            transport: Box::new(transport),
            ltpa_cookie_name: None,
            ltpa_token: None,
            last_response_payload: None,
            last_response_text: None,
            last_http_status: None,
            last_command_payload: None,
        };
        let headers = session.build_headers();
        assert!(!headers.contains_key("Cookie"));
    }

    // =====================================================================
    // get_qualifier_entry missing qualifier
    // =====================================================================

    #[test]
    fn get_qualifier_entry_qualifiers_exist_but_missing_qualifier() {
        let data = json!({"qualifiers": {"queue": {}}});
        assert!(get_qualifier_entry("nonexistent", &data).is_none());
    }
}
