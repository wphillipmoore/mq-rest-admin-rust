# Transport

## Overview

The transport layer abstracts HTTP communication from the session logic. The
session builds `runCommandJSON` payloads and delegates HTTP delivery to a
transport implementation. This separation enables testing the entire command
pipeline without an MQ server by injecting a mock transport.

## MqRestTransport

The transport trait defines a single method for posting JSON payloads:

```rust
pub trait MqRestTransport: Send + Sync {
    fn post_json(
        &self,
        url: &str,
        payload: &serde_json::Value,
        headers: &HashMap<String, String>,
        timeout_seconds: Option<f64>,
        verify_tls: bool,
    ) -> Result<TransportResponse, MqRestError>;
}
```

## TransportResponse

A struct containing the HTTP response data:

```rust
pub struct TransportResponse {
    pub status_code: u16,
    pub text: String,
    pub headers: HashMap<String, String>,
}
```

| Field | Type | Description |
| --- | --- | --- |
| `status_code` | `u16` | HTTP response status code |
| `text` | `String` | Response body as text |
| `headers` | `HashMap<String, String>` | Response headers |

## ReqwestTransport

The default transport implementation using the `reqwest` blocking client:

```rust
pub struct ReqwestTransport { /* ... */ }

impl ReqwestTransport {
    /// Standard HTTPS client
    pub fn new() -> Self;

    /// Client that accepts invalid TLS certificates
    pub fn new_insecure() -> Self;

    /// Client with mutual TLS (client certificate)
    pub fn new_with_cert(cert_path: &str, key_path: Option<&str>) -> Result<Self>;
}
```

## Custom transport

Implement the `MqRestTransport` trait to provide custom HTTP behavior or
for testing:

```rust
use mq_rest_admin::{MqRestTransport, TransportResponse, MqRestError};
use std::collections::HashMap;

struct MockTransport;

impl MqRestTransport for MockTransport {
    fn post_json(
        &self,
        _url: &str,
        _payload: &serde_json::Value,
        _headers: &HashMap<String, String>,
        _timeout: Option<f64>,
        _verify: bool,
    ) -> Result<TransportResponse, MqRestError> {
        Ok(TransportResponse {
            status_code: 200,
            text: r#"{"commandResponse": []}"#.into(),
            headers: HashMap::new(),
        })
    }
}

let session = MqRestSession::builder()
    .rest_base_url("https://localhost:9443/ibmmq/rest/v2")
    .qmgr_name("QM1")
    .credentials(Credentials::Basic {
        username: "admin".into(),
        password: "passw0rd".into(),
    })
    .transport(Box::new(MockTransport))
    .build()?;
```

This pattern makes the entire command pipeline testable without network access.
