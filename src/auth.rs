//! Authentication credential types and LTPA login support.

use std::collections::HashMap;

use serde_json::Value;

use crate::error::MqRestError;
use crate::transport::{MqRestTransport, TransportResponse};

/// LTPA cookie name used by IBM MQ.
pub const LTPA_COOKIE_NAME: &str = "LtpaToken2";
const LTPA_LOGIN_PATH: &str = "/login";
const ERROR_LTPA_LOGIN_FAILED: &str = "LTPA login failed.";
const ERROR_LTPA_TOKEN_MISSING: &str =
    "LTPA login succeeded but no LtpaToken2 cookie was returned.";

/// Supported credential types for MQ REST authentication.
#[derive(Debug, Clone)]
pub enum Credentials {
    /// HTTP Basic authentication.
    Basic {
        /// Username for HTTP Basic authentication.
        username: String,
        /// Password for HTTP Basic authentication.
        password: String,
    },
    /// LTPA token-based authentication.
    Ltpa {
        /// Username for the LTPA login request.
        username: String,
        /// Password for the LTPA login request.
        password: String,
    },
    /// Mutual TLS (mTLS) client certificate authentication.
    Certificate {
        /// Path to the client certificate PEM file.
        cert_path: String,
        /// Path to the private key PEM file.
        key_path: Option<String>,
    },
}

/// Perform an LTPA login and return the `LtpaToken2` token value.
pub(crate) fn perform_ltpa_login(
    transport: &dyn MqRestTransport,
    rest_base_url: &str,
    username: &str,
    password: &str,
    csrf_token: Option<&str>,
    timeout_seconds: Option<f64>,
    verify_tls: bool,
) -> crate::error::Result<String> {
    let login_url = format!("{rest_base_url}{LTPA_LOGIN_PATH}");
    let mut headers = HashMap::new();
    headers.insert("Accept".into(), "application/json".into());
    if let Some(token) = csrf_token {
        headers.insert("ibm-mq-rest-csrf-token".into(), token.into());
    }
    let mut payload = HashMap::new();
    payload.insert("username".into(), Value::String(username.into()));
    payload.insert("password".into(), Value::String(password.into()));
    let response =
        transport.post_json(&login_url, &payload, &headers, timeout_seconds, verify_tls)?;
    if response.status_code >= 400 {
        return Err(MqRestError::Auth {
            url: login_url,
            status_code: Some(response.status_code),
            message: ERROR_LTPA_LOGIN_FAILED.into(),
        });
    }
    match extract_ltpa_token(&response) {
        Some(token) => Ok(token),
        None => Err(MqRestError::Auth {
            url: login_url,
            status_code: Some(response.status_code),
            message: ERROR_LTPA_TOKEN_MISSING.into(),
        }),
    }
}

/// Extract the `LtpaToken2` value from response `Set-Cookie` headers.
fn extract_ltpa_token(response: &TransportResponse) -> Option<String> {
    let set_cookie = response
        .headers
        .get("Set-Cookie")
        .or_else(|| response.headers.get("set-cookie"))?;
    // Parse cookie header to find LtpaToken2
    for cookie_part in set_cookie.split(';') {
        let cookie_part = cookie_part.trim();
        if let Some(value) = cookie_part.strip_prefix(&format!("{LTPA_COOKIE_NAME}=")) {
            return Some(value.to_owned());
        }
    }
    // Also try comma-separated cookies
    for cookie_entry in set_cookie.split(',') {
        for cookie_part in cookie_entry.split(';') {
            let cookie_part = cookie_part.trim();
            if let Some(value) = cookie_part.strip_prefix(&format!("{LTPA_COOKIE_NAME}=")) {
                return Some(value.to_owned());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::MockTransport;

    fn login_response_with_cookie(cookie_header: &str, cookie_value: &str) -> TransportResponse {
        let mut headers = HashMap::new();
        headers.insert(cookie_header.into(), cookie_value.into());
        TransportResponse {
            status_code: 200,
            text: "{}".into(),
            headers,
        }
    }

    #[test]
    fn ltpa_login_success() {
        let transport = MockTransport::new(vec![login_response_with_cookie(
            "Set-Cookie",
            "LtpaToken2=abc123; Path=/",
        )]);
        let result = perform_ltpa_login(
            &transport,
            "https://host/ibmmq/rest/v2",
            "user",
            "pass",
            Some("csrf"),
            Some(10.0),
            true,
        );
        assert_eq!(result.unwrap(), "abc123");
    }

    #[test]
    fn ltpa_login_case_insensitive_header() {
        let transport = MockTransport::new(vec![login_response_with_cookie(
            "set-cookie",
            "LtpaToken2=token456; Path=/",
        )]);
        let result = perform_ltpa_login(&transport, "https://h", "u", "p", None, None, false);
        assert_eq!(result.unwrap(), "token456");
    }

    #[test]
    fn ltpa_login_comma_separated_cookies() {
        let transport = MockTransport::new(vec![login_response_with_cookie(
            "Set-Cookie",
            "other=x, LtpaToken2=fromcomma; Path=/",
        )]);
        let result = perform_ltpa_login(&transport, "https://h", "u", "p", None, None, false);
        assert_eq!(result.unwrap(), "fromcomma");
    }

    #[test]
    fn ltpa_login_http_401() {
        let transport = MockTransport::new(vec![TransportResponse {
            status_code: 401,
            text: "Unauthorized".into(),
            headers: HashMap::new(),
        }]);
        let result = perform_ltpa_login(&transport, "https://h", "u", "p", None, None, false);
        assert!(matches!(result.unwrap_err(), MqRestError::Auth { .. }));
    }

    #[test]
    fn ltpa_login_missing_token() {
        let transport = MockTransport::new(vec![TransportResponse {
            status_code: 200,
            text: "{}".into(),
            headers: HashMap::new(),
        }]);
        let result = perform_ltpa_login(&transport, "https://h", "u", "p", None, None, false);
        let err = result.unwrap_err();
        assert!(matches!(err, MqRestError::Auth { .. }));
    }

    #[test]
    fn ltpa_login_csrf_token_present_in_request() {
        let transport = MockTransport::new(vec![login_response_with_cookie(
            "Set-Cookie",
            "LtpaToken2=tok; Path=/",
        )]);
        perform_ltpa_login(
            &transport,
            "https://h",
            "u",
            "p",
            Some("mytoken"),
            None,
            false,
        )
        .unwrap();
        let requests = transport.requests();
        assert_eq!(
            requests[0].headers.get("ibm-mq-rest-csrf-token").unwrap(),
            "mytoken"
        );
    }

    #[test]
    fn ltpa_login_csrf_token_absent() {
        let transport = MockTransport::new(vec![login_response_with_cookie(
            "Set-Cookie",
            "LtpaToken2=tok; Path=/",
        )]);
        perform_ltpa_login(&transport, "https://h", "u", "p", None, None, false).unwrap();
        let requests = transport.requests();
        assert!(!requests[0].headers.contains_key("ibm-mq-rest-csrf-token"));
    }

    #[test]
    fn ltpa_login_cookie_present_but_no_ltpa_token() {
        let transport = MockTransport::new(vec![login_response_with_cookie(
            "Set-Cookie",
            "SomeOtherCookie=value; Path=/",
        )]);
        let result = perform_ltpa_login(&transport, "https://h", "u", "p", None, None, false);
        assert!(matches!(result.unwrap_err(), MqRestError::Auth { .. }));
    }

    #[test]
    fn ltpa_login_transport_error() {
        let transport = MockTransport::new(vec![]);
        let result = perform_ltpa_login(&transport, "https://h", "u", "p", None, None, false);
        assert!(result.is_err());
    }
}
