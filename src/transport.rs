//! HTTP transport trait and reqwest implementation.

use std::collections::HashMap;

use serde_json::Value;

use crate::error::MqRestError;

/// Container for the raw HTTP response returned by a transport.
#[derive(Debug, Clone)]
pub struct TransportResponse {
    /// The HTTP status code.
    pub status_code: u16,
    /// The response body as text.
    pub text: String,
    /// The response headers.
    pub headers: HashMap<String, String>,
}

/// Trait for MQ REST transport implementations.
pub trait MqRestTransport {
    /// Send a JSON payload via HTTP POST and return the response.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    fn post_json(
        &self,
        url: &str,
        payload: &HashMap<String, Value>,
        headers: &HashMap<String, String>,
        timeout_seconds: Option<f64>,
        verify_tls: bool,
    ) -> crate::error::Result<TransportResponse>;
}

/// Default transport implementation using `reqwest::blocking`.
pub struct ReqwestTransport {
    client: reqwest::blocking::Client,
}

impl ReqwestTransport {
    /// Create a new transport with default settings.
    ///
    /// # Panics
    ///
    /// Panics if the underlying reqwest client builder fails.
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .danger_accept_invalid_certs(false)
                .build()
                .expect("failed to build reqwest client"),
        }
    }

    /// Create a new transport that accepts invalid TLS certificates.
    ///
    /// # Panics
    ///
    /// Panics if the underlying reqwest client builder fails.
    #[must_use]
    pub fn new_insecure() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .expect("failed to build reqwest client"),
        }
    }

    /// Create a new transport with a client certificate for mTLS.
    ///
    /// # Errors
    ///
    /// Returns an error if the PEM identity cannot be parsed.
    ///
    /// # Panics
    ///
    /// Panics if the underlying reqwest client builder fails.
    pub fn new_with_cert(cert_pem: &[u8], key_pem: Option<&[u8]>) -> crate::error::Result<Self> {
        let mut builder = reqwest::blocking::Client::builder();
        let identity_pem = key_pem.map_or_else(
            || cert_pem.to_vec(),
            |key| {
                let mut combined = cert_pem.to_vec();
                combined.extend_from_slice(key);
                combined
            },
        );
        let identity =
            reqwest::Identity::from_pem(&identity_pem).map_err(|e| MqRestError::Transport {
                url: String::new(),
                source: e,
            })?;
        builder = builder.identity(identity);
        let client = builder
            .build()
            .expect("failed to build reqwest client with certificate");
        Ok(Self { client })
    }
}

impl Default for ReqwestTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl MqRestTransport for ReqwestTransport {
    fn post_json(
        &self,
        url: &str,
        payload: &HashMap<String, Value>,
        headers: &HashMap<String, String>,
        timeout_seconds: Option<f64>,
        _verify_tls: bool,
    ) -> crate::error::Result<TransportResponse> {
        let mut builder = self.client.post(url).json(payload);
        for (key, value) in headers {
            builder = builder.header(key.as_str(), value.as_str());
        }
        if let Some(timeout) = timeout_seconds {
            builder = builder.timeout(std::time::Duration::from_secs_f64(timeout));
        }
        let response = builder.send().map_err(|e| MqRestError::Transport {
            url: url.to_owned(),
            source: e,
        })?;
        let status_code = response.status().as_u16();
        let response_headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_owned()))
            .collect();
        let text = response.text().expect("failed to read response body text");
        Ok(TransportResponse {
            status_code,
            text,
            headers: response_headers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reqwest_transport_new_succeeds() {
        let _transport = ReqwestTransport::new();
    }

    #[test]
    fn reqwest_transport_new_insecure_succeeds() {
        let _transport = ReqwestTransport::new_insecure();
    }

    #[test]
    fn reqwest_transport_default_succeeds() {
        let _transport = ReqwestTransport::default();
    }

    #[test]
    fn reqwest_transport_new_with_cert_invalid_pem() {
        let result = ReqwestTransport::new_with_cert(b"not-a-pem", None);
        assert!(result.is_err());
    }

    #[test]
    fn post_json_to_tcp_listener() {
        use std::io::{Read, Write};
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 4096];
            let _n = stream.read(&mut buf).unwrap();
            let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nX-Test: hello\r\nContent-Length: 13\r\n\r\n{\"status\":\"ok\"}";
            stream.write_all(response.as_bytes()).unwrap();
        });

        let transport = ReqwestTransport::new();
        let url = format!("http://{addr}/test");
        let payload = HashMap::new();
        let headers = HashMap::new();
        let result = transport.post_json(&url, &payload, &headers, Some(5.0), false);
        let response = result.unwrap();
        assert_eq!(response.status_code, 200);
        assert!(response.text.contains("ok"));
        assert_eq!(response.headers.get("x-test").unwrap(), "hello");
        handle.join().unwrap();
    }

    #[test]
    fn post_json_with_headers_and_no_timeout() {
        use std::io::{Read, Write};
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 4096];
            let _n = stream.read(&mut buf).unwrap();
            let body = r#"{"ok":true}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).unwrap();
        });

        let transport = ReqwestTransport::new();
        let url = format!("http://{addr}/test");
        let payload = HashMap::new();
        let mut headers = HashMap::new();
        headers.insert("X-Custom".into(), "value".into());
        // No timeout (None)
        let result = transport.post_json(&url, &payload, &headers, None, false);
        assert!(result.is_ok());
        handle.join().unwrap();
    }

    #[test]
    fn reqwest_transport_new_with_cert_with_key_invalid() {
        let result = ReqwestTransport::new_with_cert(b"not-pem", Some(b"also-not-pem"));
        assert!(result.is_err());
    }

    #[test]
    fn reqwest_transport_new_with_cert_combined_pem() {
        // Combined cert+key PEM (required by reqwest Identity::from_pem)
        let pem = include_bytes!("../test-fixtures/test-combined.pem");
        let result = ReqwestTransport::new_with_cert(pem, None);
        assert!(result.is_ok());
    }

    #[test]
    fn reqwest_transport_new_with_cert_and_separate_key() {
        let cert = include_bytes!("../test-fixtures/test-cert.pem");
        let key = include_bytes!("../test-fixtures/test-key.pem");
        let result = ReqwestTransport::new_with_cert(cert, Some(key));
        assert!(result.is_ok());
    }

    #[test]
    fn post_json_connection_refused() {
        let transport = ReqwestTransport::new();
        let payload = HashMap::new();
        let headers = HashMap::new();
        let result = transport.post_json(
            "http://127.0.0.1:1/bad",
            &payload,
            &headers,
            Some(1.0),
            true,
        );
        assert!(format!("{:?}", result.unwrap_err()).starts_with("Transport"));
    }
}
