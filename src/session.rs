//! MQ REST API session and command execution.

use std::collections::HashMap;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde_json::Value;

use crate::auth::{Credentials, LTPA_COOKIE_NAME, perform_ltpa_login};
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
    pub fn gateway_qmgr(mut self, name: impl Into<String>) -> Self {
        self.gateway_qmgr = Some(name.into());
        self
    }

    /// Set whether to verify TLS certificates.
    pub fn verify_tls(mut self, verify: bool) -> Self {
        self.verify_tls = verify;
        self
    }

    /// Set the HTTP request timeout in seconds.
    pub fn timeout_seconds(mut self, timeout: Option<f64>) -> Self {
        self.timeout_seconds = timeout;
        self
    }

    /// Set whether to map attributes between snake_case and MQSC names.
    pub fn map_attributes(mut self, enabled: bool) -> Self {
        self.map_attributes = enabled;
        self
    }

    /// Set whether mapping failures are strict errors.
    pub fn mapping_strict(mut self, strict: bool) -> Self {
        self.mapping_strict = strict;
        self
    }

    /// Set mapping overrides.
    pub fn mapping_overrides(mut self, overrides: Value) -> Self {
        self.mapping_overrides = Some(overrides);
        self
    }

    /// Set the mapping overrides mode.
    pub fn mapping_overrides_mode(mut self, mode: MappingOverrideMode) -> Self {
        self.mapping_overrides_mode = mode;
        self
    }

    /// Set the CSRF token value.
    pub fn csrf_token(mut self, token: Option<String>) -> Self {
        self.csrf_token = token;
        self
    }

    /// Set a custom transport implementation.
    pub fn transport(mut self, transport: Box<dyn MqRestTransport>) -> Self {
        self.transport = Some(transport);
        self
    }

    /// Build the session, performing LTPA login if needed.
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

        let ltpa_token = if let Credentials::Ltpa {
            ref username,
            ref password,
        } = self.credentials
        {
            Some(perform_ltpa_login(
                transport.as_ref(),
                &rest_base_url,
                username,
                password,
                self.csrf_token.as_deref(),
                self.timeout_seconds,
                self.verify_tls,
            )?)
        } else {
            None
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
    pub fn builder(
        rest_base_url: impl Into<String>,
        qmgr_name: impl Into<String>,
        credentials: Credentials,
    ) -> MqRestSessionBuilder {
        MqRestSessionBuilder::new(rest_base_url, qmgr_name, credentials)
    }

    /// The queue manager name this session targets.
    pub fn qmgr_name(&self) -> &str {
        &self.qmgr_name
    }

    /// The gateway queue manager name, or `None` for direct access.
    pub fn gateway_qmgr(&self) -> Option<&str> {
        self.gateway_qmgr.as_deref()
    }

    /// Core MQSC command dispatch method.
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
                if let Some(ref token) = self.ltpa_token {
                    headers.insert("Cookie".into(), format!("{LTPA_COOKIE_NAME}={token}"));
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
    match response_parameters {
        None => {
            if is_display {
                DEFAULT_RESPONSE_PARAMETERS
                    .iter()
                    .map(|s| (*s).to_owned())
                    .collect()
            } else {
                Vec::new()
            }
        }
        Some(params) => {
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
                    for (k, v) in obj {
                        merged.insert(k.clone(), v.clone());
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
    let overall_cc = extract_optional_i64(payload.get("overallCompletionCode"));
    let overall_rc = extract_optional_i64(payload.get("overallReasonCode"));
    let has_overall_error = has_error_codes(overall_cc, overall_rc);

    let mut command_issues: Vec<String> = Vec::new();
    if let Some(Value::Array(cr)) = payload.get("commandResponse") {
        for (idx, item) in cr.iter().enumerate() {
            if let Some(obj) = item.as_object() {
                let cc = extract_optional_i64(obj.get("completionCode"));
                let rc = extract_optional_i64(obj.get("reasonCode"));
                if has_error_codes(cc, rc) {
                    command_issues.push(format!(
                        "index={idx} completionCode={} reasonCode={}",
                        cc.unwrap_or(0),
                        rc.unwrap_or(0),
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
                overall_cc.unwrap_or(0),
                overall_rc.unwrap_or(0),
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

fn has_error_codes(cc: Option<i64>, rc: Option<i64>) -> bool {
    matches!(cc, Some(c) if c != 0) || matches!(rc, Some(r) if r != 0)
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
    if let Some(rkm) = qualifier_entry
        .get("response_key_map")
        .and_then(Value::as_object)
    {
        for (mqsc_key, snake_val) in rkm {
            if let Some(snake_key) = snake_val.as_str() {
                response_lookup
                    .entry(snake_key.to_owned())
                    .or_insert_with(|| mqsc_key.clone());
            }
        }
    }
    let mut combined = response_lookup;
    if let Some(rkm) = qualifier_entry
        .get("request_key_map")
        .and_then(Value::as_object)
    {
        for (snake_key, mqsc_val) in rkm {
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
    let mapped_keyword = match combined_map.get(keyword) {
        Some(mk) => mk.clone(),
        None => {
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
        }
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
        match combined_map.get(name.as_str()) {
            Some(mapped_key) => {
                mapped.push(mapped_key.clone());
            }
            None => {
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
    }
    (mapped, issues)
}

fn get_qualifier_entry<'a>(qualifier: &str, mapping_data: &'a Value) -> Option<&'a Value> {
    mapping_data.get("qualifiers")?.get(qualifier)
}
