//! Shared test utilities for the MQ REST admin library.

use std::cell::RefCell;
use std::collections::HashMap;

use serde_json::{Value, json};

use crate::auth::Credentials;
use crate::error::{MqRestError, Result};
use crate::session::MqRestSession;
use crate::transport::{MqRestTransport, TransportResponse};

/// Captures the details of a single request made via `MockTransport`.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RecordedRequest {
    pub url: String,
    pub payload: HashMap<String, Value>,
    pub headers: HashMap<String, String>,
    pub timeout_seconds: Option<f64>,
    pub verify_tls: bool,
}

/// A test transport that records requests and returns canned responses.
pub struct MockTransport {
    responses: RefCell<Vec<TransportResponse>>,
    requests: RefCell<Vec<RecordedRequest>>,
}

impl MockTransport {
    /// Create a new mock with the given FIFO response queue.
    pub fn new(responses: Vec<TransportResponse>) -> Self {
        Self {
            responses: RefCell::new(responses),
            requests: RefCell::new(Vec::new()),
        }
    }

    /// Return all recorded requests.
    pub fn requests(&self) -> Vec<RecordedRequest> {
        self.requests.borrow().clone()
    }
}

impl MqRestTransport for MockTransport {
    fn post_json(
        &self,
        url: &str,
        payload: &HashMap<String, Value>,
        headers: &HashMap<String, String>,
        timeout_seconds: Option<f64>,
        verify_tls: bool,
    ) -> Result<TransportResponse> {
        self.requests.borrow_mut().push(RecordedRequest {
            url: url.to_owned(),
            payload: payload.clone(),
            headers: headers.clone(),
            timeout_seconds,
            verify_tls,
        });
        let mut responses = self.responses.borrow_mut();
        if responses.is_empty() {
            return Err(MqRestError::Response {
                message: "MockTransport: no more canned responses".into(),
                response_text: None,
            });
        }
        Ok(responses.remove(0))
    }
}

/// Build a standard MQ success response body.
pub fn success_response(params: Vec<HashMap<String, Value>>) -> TransportResponse {
    let command_response: Vec<Value> = params
        .into_iter()
        .map(|p| json!({"completionCode": 0, "reasonCode": 0, "parameters": p}))
        .collect();
    let body = json!({
        "overallCompletionCode": 0,
        "overallReasonCode": 0,
        "commandResponse": command_response,
    });
    TransportResponse {
        status_code: 200,
        text: body.to_string(),
        headers: HashMap::new(),
    }
}

/// Build a success response with no `commandResponse` items.
pub fn empty_success_response() -> TransportResponse {
    let body = json!({
        "overallCompletionCode": 0,
        "overallReasonCode": 0,
        "commandResponse": [],
    });
    TransportResponse {
        status_code: 200,
        text: body.to_string(),
        headers: HashMap::new(),
    }
}

/// Build a command error response.
pub fn error_response(completion: i64, reason: i64) -> TransportResponse {
    let body = json!({
        "overallCompletionCode": completion,
        "overallReasonCode": reason,
        "commandResponse": [],
    });
    TransportResponse {
        status_code: 200,
        text: body.to_string(),
        headers: HashMap::new(),
    }
}

/// Build a command error response with item-level errors (for ensure not-found).
pub fn command_error_response() -> TransportResponse {
    let body = json!({
        "overallCompletionCode": 2,
        "overallReasonCode": 3008,
        "commandResponse": [
            {"completionCode": 2, "reasonCode": 3008}
        ],
    });
    TransportResponse {
        status_code: 200,
        text: body.to_string(),
        headers: HashMap::new(),
    }
}

/// Create a session with mapping disabled and Basic auth.
pub fn mock_session(transport: MockTransport) -> MqRestSession {
    MqRestSession::builder(
        "https://localhost:9443/ibmmq/rest/v2",
        "QM1",
        Credentials::Basic {
            username: "admin".into(),
            password: "admin".into(),
        },
    )
    .map_attributes(false)
    .transport(Box::new(transport))
    .build()
    .expect("mock session build failed")
}

/// Create a session with mapping enabled and Basic auth.
pub fn mock_session_with_mapping(transport: MockTransport) -> MqRestSession {
    MqRestSession::builder(
        "https://localhost:9443/ibmmq/rest/v2",
        "QM1",
        Credentials::Basic {
            username: "admin".into(),
            password: "admin".into(),
        },
    )
    .map_attributes(true)
    .mapping_strict(false)
    .transport(Box::new(transport))
    .build()
    .expect("mock session build failed")
}
