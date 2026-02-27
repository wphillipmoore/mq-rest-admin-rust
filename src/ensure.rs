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
    pub fn ensure_qlocal(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "QUEUE", "QLOCAL", "QLOCAL")
    }

    /// Ensure a remote queue exists with the specified attributes.
    pub fn ensure_qremote(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "QUEUE", "QREMOTE", "QREMOTE")
    }

    /// Ensure an alias queue exists with the specified attributes.
    pub fn ensure_qalias(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "QUEUE", "QALIAS", "QALIAS")
    }

    /// Ensure a model queue exists with the specified attributes.
    pub fn ensure_qmodel(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "QUEUE", "QMODEL", "QMODEL")
    }

    /// Ensure a channel exists with the specified attributes.
    pub fn ensure_channel(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "CHANNEL", "CHANNEL", "CHANNEL")
    }

    /// Ensure an authentication information object exists with the specified attributes.
    pub fn ensure_authinfo(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "AUTHINFO", "AUTHINFO", "AUTHINFO")
    }

    /// Ensure a listener exists with the specified attributes.
    pub fn ensure_listener(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "LISTENER", "LISTENER", "LISTENER")
    }

    /// Ensure a namelist exists with the specified attributes.
    pub fn ensure_namelist(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "NAMELIST", "NAMELIST", "NAMELIST")
    }

    /// Ensure a process exists with the specified attributes.
    pub fn ensure_process(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "PROCESS", "PROCESS", "PROCESS")
    }

    /// Ensure a service exists with the specified attributes.
    pub fn ensure_service(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "SERVICE", "SERVICE", "SERVICE")
    }

    /// Ensure a topic exists with the specified attributes.
    pub fn ensure_topic(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "TOPIC", "TOPIC", "TOPIC")
    }

    /// Ensure a subscription exists with the specified attributes.
    pub fn ensure_sub(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "SUB", "SUB", "SUB")
    }

    /// Ensure a storage class exists with the specified attributes.
    pub fn ensure_stgclass(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "STGCLASS", "STGCLASS", "STGCLASS")
    }

    /// Ensure a communication information object exists with the specified attributes.
    pub fn ensure_comminfo(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
    ) -> Result<EnsureResult> {
        self.ensure_object(name, request_parameters, "COMMINFO", "COMMINFO", "COMMINFO")
    }

    /// Ensure a CF structure exists with the specified attributes.
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
