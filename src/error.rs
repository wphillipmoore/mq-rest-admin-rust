//! Error types for the MQ REST admin library.

use std::collections::HashMap;
use std::fmt;

use serde_json::Value;

/// A single mapping issue recorded during attribute translation.
#[derive(Debug, Clone)]
pub struct MappingIssue {
    /// Whether the issue occurred during `"request"` or `"response"` mapping.
    pub direction: String,
    /// Category of the mapping failure.
    pub reason: String,
    /// The attribute name that triggered the issue.
    pub attribute_name: String,
    /// The attribute value, if relevant to the issue.
    pub attribute_value: Option<Value>,
    /// Zero-based index within a response list.
    pub object_index: Option<usize>,
    /// The qualifier that was being mapped.
    pub qualifier: Option<String>,
}

impl MappingIssue {
    /// Return the issue as a JSON-serialisable map.
    #[must_use]
    pub fn to_payload(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("direction".into(), Value::String(self.direction.clone()));
        map.insert("reason".into(), Value::String(self.reason.clone()));
        map.insert(
            "attribute_name".into(),
            Value::String(self.attribute_name.clone()),
        );
        map.insert(
            "attribute_value".into(),
            self.attribute_value.clone().unwrap_or(Value::Null),
        );
        map.insert(
            "object_index".into(),
            self.object_index
                .map_or(Value::Null, |i| Value::Number(i.into())),
        );
        map.insert(
            "qualifier".into(),
            self.qualifier
                .as_ref()
                .map_or(Value::Null, |q| Value::String(q.clone())),
        );
        map
    }
}

/// Error raised when attribute mapping fails in strict mode.
#[derive(Debug, Clone)]
pub struct MappingError {
    /// The mapping issues captured during the failed operation.
    pub issues: Vec<MappingIssue>,
    message: String,
}

impl MappingError {
    /// Create a new mapping error from the captured issues.
    #[must_use]
    pub fn new(issues: Vec<MappingIssue>) -> Self {
        let message = build_mapping_message(&issues);
        Self { issues, message }
    }

    /// Return mapping issues as JSON-serialisable maps.
    #[must_use]
    pub fn to_payload(&self) -> Vec<HashMap<String, Value>> {
        self.issues.iter().map(MappingIssue::to_payload).collect()
    }
}

impl fmt::Display for MappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for MappingError {}

fn build_mapping_message(issues: &[MappingIssue]) -> String {
    if issues.is_empty() {
        return "Mapping failed with no issues reported.".into();
    }
    let mut lines = vec![format!("Mapping failed with {} issue(s):", issues.len())];
    for issue in issues {
        let index_label = issue
            .object_index
            .map_or_else(|| "-".into(), |i| i.to_string());
        let qualifier_label = issue.qualifier.as_deref().unwrap_or("-");
        let value_label = issue
            .attribute_value
            .as_ref()
            .map_or_else(|| "-".into(), |v| format!("{v}"));
        lines.push(format!(
            "index={} | qualifier={} | direction={} | reason={} | attribute={} | value={}",
            index_label,
            qualifier_label,
            issue.direction,
            issue.reason,
            issue.attribute_name,
            value_label,
        ));
    }
    lines.join("\n")
}

/// All error types for the MQ REST admin library.
#[derive(Debug, thiserror::Error)]
pub enum MqRestError {
    /// The transport failed to reach the MQ REST endpoint.
    #[error("Failed to reach MQ REST endpoint: {url}")]
    Transport {
        /// The endpoint URL that could not be reached.
        url: String,
        /// The underlying transport error.
        #[source]
        source: reqwest::Error,
    },

    /// The MQ REST response was malformed or unexpected.
    #[error("{message}")]
    Response {
        /// Human-readable error description.
        message: String,
        /// The raw response body, if available.
        response_text: Option<String>,
    },

    /// Authentication with the MQ REST API failed.
    #[error("{message}")]
    Auth {
        /// The endpoint URL where authentication failed.
        url: String,
        /// The HTTP status code, if available.
        status_code: Option<u16>,
        /// Human-readable error description.
        message: String,
    },

    /// The MQ REST response indicates MQSC command failure.
    #[error("{message}")]
    Command {
        /// The full JSON response payload.
        payload: HashMap<String, Value>,
        /// The HTTP status code, if available.
        status_code: Option<u16>,
        /// Human-readable error description.
        message: String,
    },

    /// A synchronous operation exceeded its timeout.
    #[error("{message}")]
    Timeout {
        /// The MQ object name that timed out.
        name: String,
        /// A description of the operation that timed out.
        operation: String,
        /// Seconds elapsed before the timeout was raised.
        elapsed: f64,
        /// Human-readable error description.
        message: String,
    },

    /// Attribute mapping failed.
    #[error(transparent)]
    Mapping(#[from] MappingError),
}

/// Convenience alias for `Result<T, MqRestError>`.
pub type Result<T> = std::result::Result<T, MqRestError>;
