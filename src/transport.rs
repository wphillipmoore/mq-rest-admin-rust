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
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .danger_accept_invalid_certs(false)
                .build()
                .expect("failed to build reqwest client"),
        }
    }

    /// Create a new transport that accepts invalid TLS certificates.
    pub fn new_insecure() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .expect("failed to build reqwest client"),
        }
    }

    /// Create a new transport with a client certificate for mTLS.
    pub fn new_with_cert(cert_pem: &[u8], key_pem: Option<&[u8]>) -> crate::error::Result<Self> {
        let mut builder = reqwest::blocking::Client::builder();
        let identity_pem = match key_pem {
            Some(key) => {
                let mut combined = cert_pem.to_vec();
                combined.extend_from_slice(key);
                combined
            }
            None => cert_pem.to_vec(),
        };
        let identity =
            reqwest::Identity::from_pem(&identity_pem).map_err(|e| MqRestError::Transport {
                url: String::new(),
                source: e,
            })?;
        builder = builder.identity(identity);
        let client = builder.build().map_err(|e| MqRestError::Transport {
            url: String::new(),
            source: e,
        })?;
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
        let text = response.text().map_err(|e| MqRestError::Transport {
            url: url.to_owned(),
            source: e,
        })?;
        Ok(TransportResponse {
            status_code,
            text,
            headers: response_headers,
        })
    }
}
