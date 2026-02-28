//! Idempotent ensure methods for MQ object management.

use std::collections::HashMap;

use serde_json::Value;

use crate::error::{MqRestError, Result};
use crate::session::MqRestSession;

/// Action taken by an ensure operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnsureAction {
    /// The object did not exist and was defined.
    Created,
    /// The object existed but attributes differed and were altered.
    Updated,
    /// The object existed and all specified attributes already matched.
    Unchanged,
}

/// Result of an ensure operation.
#[derive(Debug, Clone)]
pub struct EnsureResult {
    /// The action indicating what happened.
    pub action: EnsureAction,
    /// Attribute names that triggered the ALTER.
    pub changed: Vec<String>,
}

impl MqRestSession {
    /// Ensure the queue manager has the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY or ALTER command fails.
    pub fn ensure_qmgr(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        let params: HashMap<String, Value> = request_parameters.cloned().unwrap_or_default();
        if params.is_empty() {
            return Ok(EnsureResult {
                action: EnsureAction::Unchanged,
                changed: Vec::new(),
            });
        }

        let all_params: &[&str] = &["all"];
        let current_objects =
            self.mqsc_command("DISPLAY", "QMGR", None, None, Some(all_params), None)?;

        let current = current_objects.into_iter().next().unwrap_or_default();
        let mut changed: HashMap<String, Value> = HashMap::new();
        for (key, desired_value) in &params {
            let current_value = current.get(key);
            if !values_match(desired_value, current_value) {
                changed.insert(key.clone(), desired_value.clone());
            }
        }

        if changed.is_empty() {
            return Ok(EnsureResult {
                action: EnsureAction::Unchanged,
                changed: Vec::new(),
            });
        }

        self.mqsc_command("ALTER", "QMGR", None, Some(&changed), None, None)?;
        Ok(EnsureResult {
            action: EnsureAction::Updated,
            changed: changed.keys().cloned().collect(),
        })
    }

    /// Ensure a local queue exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_qlocal(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "QUEUE", "QLOCAL", "QLOCAL")
    }

    /// Ensure a remote queue exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_qremote(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "QUEUE", "QREMOTE", "QREMOTE")
    }

    /// Ensure an alias queue exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_qalias(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "QUEUE", "QALIAS", "QALIAS")
    }

    /// Ensure a model queue exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_qmodel(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "QUEUE", "QMODEL", "QMODEL")
    }

    /// Ensure a channel exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_channel(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "CHANNEL", "CHANNEL", "CHANNEL")
    }

    /// Ensure an authentication information object exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_authinfo(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "AUTHINFO", "AUTHINFO", "AUTHINFO")
    }

    /// Ensure a listener exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_listener(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "LISTENER", "LISTENER", "LISTENER")
    }

    /// Ensure a namelist exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_namelist(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "NAMELIST", "NAMELIST", "NAMELIST")
    }

    /// Ensure a process exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_process(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "PROCESS", "PROCESS", "PROCESS")
    }

    /// Ensure a service exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_service(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "SERVICE", "SERVICE", "SERVICE")
    }

    /// Ensure a topic exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_topic(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "TOPIC", "TOPIC", "TOPIC")
    }

    /// Ensure a subscription exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_sub(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "SUB", "SUB", "SUB")
    }

    /// Ensure a storage class exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_stgclass(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "STGCLASS", "STGCLASS", "STGCLASS")
    }

    /// Ensure a communication information object exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_comminfo(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "COMMINFO", "COMMINFO", "COMMINFO")
    }

    /// Ensure a CF structure exists with the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if the DISPLAY, DEFINE, or ALTER command fails.
    pub fn ensure_cfstruct(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "CFSTRUCT", "CFSTRUCT", "CFSTRUCT")
    }

    fn ensure_object(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
        display_qualifier: &str,
        define_qualifier: &str,
        alter_qualifier: &str,
    ) -> Result<EnsureResult> {
        let all_params: &[&str] = &["all"];
        let current_objects = match self.mqsc_command(
            "DISPLAY",
            display_qualifier,
            Some(name),
            None,
            Some(all_params),
            None,
        ) {
            Ok(objects) => objects,
            Err(MqRestError::Command { .. }) => Vec::new(),
            Err(e) => return Err(e),
        };

        let params: HashMap<String, Value> = request_parameters.cloned().unwrap_or_default();

        if current_objects.is_empty() {
            let define_params = if params.is_empty() {
                None
            } else {
                Some(&params)
            };
            self.mqsc_command(
                "DEFINE",
                define_qualifier,
                Some(name),
                define_params,
                None,
                None,
            )?;
            return Ok(EnsureResult {
                action: EnsureAction::Created,
                changed: Vec::new(),
            });
        }

        if params.is_empty() {
            return Ok(EnsureResult {
                action: EnsureAction::Unchanged,
                changed: Vec::new(),
            });
        }

        let current = &current_objects[0];
        let mut changed: HashMap<String, Value> = HashMap::new();
        for (key, desired_value) in &params {
            let current_value = current.get(key);
            if !values_match(desired_value, current_value) {
                changed.insert(key.clone(), desired_value.clone());
            }
        }

        if changed.is_empty() {
            return Ok(EnsureResult {
                action: EnsureAction::Unchanged,
                changed: Vec::new(),
            });
        }

        self.mqsc_command(
            "ALTER",
            alter_qualifier,
            Some(name),
            Some(&changed),
            None,
            None,
        )?;
        Ok(EnsureResult {
            action: EnsureAction::Updated,
            changed: changed.keys().cloned().collect(),
        })
    }
}

fn values_match(desired: &Value, current: Option<&Value>) -> bool {
    let Some(current) = current else {
        return false;
    };
    let desired_str = value_to_string(desired);
    let current_str = value_to_string(current);
    desired_str.trim().eq_ignore_ascii_case(current_str.trim())
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{
        MockTransport, command_error_response, empty_success_response, mock_session,
        success_response,
    };
    use serde_json::json;

    // ---- ensure_qmgr ----

    #[test]
    fn ensure_qmgr_empty_params_unchanged() {
        let transport = MockTransport::new(vec![]);
        let mut session = mock_session(transport);
        let result = session.ensure_qmgr(None).unwrap();
        assert_eq!(result.action, EnsureAction::Unchanged);
    }

    #[test]
    fn ensure_qmgr_matching_params_unchanged() {
        let mut current = HashMap::new();
        current.insert("DESCR".into(), json!("test"));
        let transport = MockTransport::new(vec![success_response(vec![current])]);
        let mut session = mock_session(transport);
        let mut params = HashMap::new();
        params.insert("DESCR".into(), json!("test"));
        let result = session.ensure_qmgr(Some(&params)).unwrap();
        assert_eq!(result.action, EnsureAction::Unchanged);
    }

    #[test]
    fn ensure_qmgr_differing_params_updated() {
        let mut current = HashMap::new();
        current.insert("DESCR".into(), json!("old"));
        let transport = MockTransport::new(vec![
            success_response(vec![current]),
            empty_success_response(),
        ]);
        let mut session = mock_session(transport);
        let mut params = HashMap::new();
        params.insert("DESCR".into(), json!("new"));
        let result = session.ensure_qmgr(Some(&params)).unwrap();
        assert_eq!(result.action, EnsureAction::Updated);
        assert!(result.changed.contains(&"DESCR".to_owned()));
    }

    // ---- ensure_object (via ensure_qlocal) ----

    #[test]
    fn ensure_qlocal_not_found_created() {
        let transport =
            MockTransport::new(vec![command_error_response(), empty_success_response()]);
        let mut session = mock_session(transport);
        let result = session.ensure_qlocal("MY.Q", None).unwrap();
        assert_eq!(result.action, EnsureAction::Created);
    }

    #[test]
    fn ensure_qlocal_exists_unchanged() {
        let mut current = HashMap::new();
        current.insert("DESCR".into(), json!("test"));
        let transport = MockTransport::new(vec![success_response(vec![current])]);
        let mut session = mock_session(transport);
        let mut params = HashMap::new();
        params.insert("DESCR".into(), json!("test"));
        let result = session.ensure_qlocal("MY.Q", Some(&params)).unwrap();
        assert_eq!(result.action, EnsureAction::Unchanged);
    }

    #[test]
    fn ensure_qlocal_exists_updated() {
        let mut current = HashMap::new();
        current.insert("DESCR".into(), json!("old"));
        let transport = MockTransport::new(vec![
            success_response(vec![current]),
            empty_success_response(),
        ]);
        let mut session = mock_session(transport);
        let mut params = HashMap::new();
        params.insert("DESCR".into(), json!("new"));
        let result = session.ensure_qlocal("MY.Q", Some(&params)).unwrap();
        assert_eq!(result.action, EnsureAction::Updated);
    }

    #[test]
    fn ensure_qlocal_empty_params_not_found_created() {
        let transport =
            MockTransport::new(vec![command_error_response(), empty_success_response()]);
        let mut session = mock_session(transport);
        let result = session.ensure_qlocal("MY.Q", None).unwrap();
        assert_eq!(result.action, EnsureAction::Created);
    }

    #[test]
    fn ensure_qlocal_empty_params_exists_unchanged() {
        let current = HashMap::new();
        let transport = MockTransport::new(vec![success_response(vec![current])]);
        let mut session = mock_session(transport);
        let result = session.ensure_qlocal("MY.Q", None).unwrap();
        assert_eq!(result.action, EnsureAction::Unchanged);
    }

    #[test]
    fn ensure_qlocal_non_command_error_propagated() {
        let transport = MockTransport::new(vec![]);
        let mut session = mock_session(transport);
        let result = session.ensure_qlocal("MY.Q", None);
        assert!(result.is_err());
    }

    // ---- values_match ----

    #[test]
    fn values_match_case_insensitive() {
        assert!(values_match(&json!("YES"), Some(&json!("yes"))));
    }

    #[test]
    fn values_match_trimmed() {
        assert!(values_match(&json!("YES"), Some(&json!(" YES "))));
    }

    #[test]
    fn values_match_no_match() {
        assert!(!values_match(&json!("YES"), Some(&json!("NO"))));
    }

    #[test]
    fn values_match_none_current() {
        assert!(!values_match(&json!("YES"), None));
    }

    // ---- value_to_string ----

    #[test]
    fn value_to_string_string() {
        assert_eq!(value_to_string(&json!("hello")), "hello");
    }

    #[test]
    fn value_to_string_number() {
        assert_eq!(value_to_string(&json!(42)), "42");
    }

    #[test]
    fn value_to_string_bool() {
        assert_eq!(value_to_string(&json!(true)), "true");
    }

    #[test]
    fn value_to_string_null() {
        assert_eq!(value_to_string(&json!(null)), "");
    }

    #[test]
    fn value_to_string_other() {
        let val = json!({"key": "val"});
        let result = value_to_string(&val);
        assert!(result.contains("key"));
    }

    // ---- Macro-generated per-method ensure tests ----

    macro_rules! test_ensure_created {
        ($method:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ $method _created>]() {
                    let transport = MockTransport::new(vec![
                        command_error_response(),
                        empty_success_response(),
                    ]);
                    let mut session = mock_session(transport);
                    let result = session.$method("OBJ", None).unwrap();
                    assert_eq!(result.action, EnsureAction::Created);
                }
            }
        };
    }

    test_ensure_created!(ensure_qlocal);
    test_ensure_created!(ensure_qremote);
    test_ensure_created!(ensure_qalias);
    test_ensure_created!(ensure_qmodel);
    test_ensure_created!(ensure_channel);
    test_ensure_created!(ensure_authinfo);
    test_ensure_created!(ensure_listener);
    test_ensure_created!(ensure_namelist);
    test_ensure_created!(ensure_process);
    test_ensure_created!(ensure_service);
    test_ensure_created!(ensure_topic);
    test_ensure_created!(ensure_sub);
    test_ensure_created!(ensure_stgclass);
    test_ensure_created!(ensure_comminfo);
    test_ensure_created!(ensure_cfstruct);

    #[test]
    fn ensure_qlocal_not_found_with_params_created() {
        let mut params = HashMap::new();
        params.insert("DESCR".into(), json!("my queue"));
        let transport =
            MockTransport::new(vec![command_error_response(), empty_success_response()]);
        let mut session = mock_session(transport);
        let result = session.ensure_qlocal("MY.Q", Some(&params)).unwrap();
        assert_eq!(result.action, EnsureAction::Created);
    }

    #[test]
    fn ensure_qlocal_define_fails() {
        // Object not found → DEFINE fails
        let transport = MockTransport::new(vec![
            command_error_response(),
            // No response for DEFINE → transport error
        ]);
        let mut session = mock_session(transport);
        let result = session.ensure_qlocal("MY.Q", None);
        assert!(result.is_err());
    }

    #[test]
    fn ensure_qlocal_alter_fails() {
        // Object exists with different params → ALTER fails
        let mut current = HashMap::new();
        current.insert("DESCR".into(), json!("old"));
        let transport = MockTransport::new(vec![
            success_response(vec![current]),
            // No response for ALTER → transport error
        ]);
        let mut session = mock_session(transport);
        let mut params = HashMap::new();
        params.insert("DESCR".into(), json!("new"));
        let result = session.ensure_qlocal("MY.Q", Some(&params));
        assert!(result.is_err());
    }
}
