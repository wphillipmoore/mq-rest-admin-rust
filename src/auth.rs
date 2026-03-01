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

/// Perform an LTPA login and return the cookie name and token value.
///
/// The cookie name may be `"LtpaToken2"` or a suffixed variant like
/// `"LtpaToken2_xyz"`.
pub(crate) fn perform_ltpa_login(
    transport: &dyn MqRestTransport,
    rest_base_url: &str,
    username: &str,
    password: &str,
    csrf_token: Option<&str>,
    timeout_seconds: Option<f64>,
    verify_tls: bool,
) -> crate::error::Result<(String, String)> {
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
        Some(result) => Ok(result),
        None => Err(MqRestError::Auth {
            url: login_url,
            status_code: Some(response.status_code),
            message: ERROR_LTPA_TOKEN_MISSING.into(),
        }),
    }
}

/// Extract an `LtpaToken2` cookie from response `Set-Cookie` headers.
///
/// Matches any cookie whose name equals `"LtpaToken2"` or starts with
/// `"LtpaToken2"` (e.g. `"LtpaToken2_abcdef"`), using prefix matching
/// to support Liberty's suffixed cookie names.
///
/// Returns a `(cookie_name, token_value)` tuple, or `None` if not found.
fn extract_ltpa_token(response: &TransportResponse) -> Option<(String, String)> {
    let set_cookie = response
        .headers
        .get("Set-Cookie")
        .or_else(|| response.headers.get("set-cookie"))?;
    // Parse cookie header to find LtpaToken2 (exact or prefixed name)
    for cookie_part in set_cookie.split(';') {
        let cookie_part = cookie_part.trim();
        if cookie_part.starts_with(LTPA_COOKIE_NAME)
            && let Some(eq_index) = cookie_part.find('=')
        {
            let name = &cookie_part[..eq_index];
            let value = &cookie_part[eq_index + 1..];
            return Some((name.to_owned(), value.to_owned()));
        }
    }
    // Also try comma-separated cookies
    for cookie_entry in set_cookie.split(',') {
        for cookie_part in cookie_entry.split(';') {
            let cookie_part = cookie_part.trim();
            if cookie_part.starts_with(LTPA_COOKIE_NAME)
                && let Some(eq_index) = cookie_part.find('=')
            {
                let name = &cookie_part[..eq_index];
                let value = &cookie_part[eq_index + 1..];
                return Some((name.to_owned(), value.to_owned()));
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
        let (name, value) = result.unwrap();
        assert_eq!(name, "LtpaToken2");
        assert_eq!(value, "abc123");
    }

    #[test]
    fn ltpa_login_success_with_suffixed_cookie() {
        let transport = MockTransport::new(vec![login_response_with_cookie(
            "Set-Cookie",
            "LtpaToken2_abcdef=suffixed_tok; Path=/",
        )]);
        let result = perform_ltpa_login(
            &transport,
            "https://host/ibmmq/rest/v2",
            "user",
            "pass",
            None,
            None,
            false,
        );
        let (name, value) = result.unwrap();
        assert_eq!(name, "LtpaToken2_abcdef");
        assert_eq!(value, "suffixed_tok");
    }

    #[test]
    fn ltpa_login_case_insensitive_header() {
        let transport = MockTransport::new(vec![login_response_with_cookie(
            "set-cookie",
            "LtpaToken2=token456; Path=/",
        )]);
        let result = perform_ltpa_login(&transport, "https://h", "u", "p", None, None, false);
        let (name, value) = result.unwrap();
        assert_eq!(name, "LtpaToken2");
        assert_eq!(value, "token456");
    }

    #[test]
    fn ltpa_login_comma_separated_cookies() {
        let transport = MockTransport::new(vec![login_response_with_cookie(
            "Set-Cookie",
            "other=x, LtpaToken2=fromcomma; Path=/",
        )]);
        let result = perform_ltpa_login(&transport, "https://h", "u", "p", None, None, false);
        let (name, value) = result.unwrap();
        assert_eq!(name, "LtpaToken2");
        assert_eq!(value, "fromcomma");
    }

    #[test]
    fn ltpa_login_http_401() {
        let transport = MockTransport::new(vec![TransportResponse {
            status_code: 401,
            text: "Unauthorized".into(),
            headers: HashMap::new(),
        }]);
        let result = perform_ltpa_login(&transport, "https://h", "u", "p", None, None, false);
        assert!(format!("{:?}", result.unwrap_err()).starts_with("Auth"));
    }

    #[test]
    fn ltpa_login_missing_token() {
        let transport = MockTransport::new(vec![TransportResponse {
            status_code: 200,
            text: "{}".into(),
            headers: HashMap::new(),
        }]);
        let result = perform_ltpa_login(&transport, "https://h", "u", "p", None, None, false);
        assert!(format!("{:?}", result.unwrap_err()).starts_with("Auth"));
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
        assert!(format!("{:?}", result.unwrap_err()).starts_with("Auth"));
    }

    #[test]
    fn ltpa_login_transport_error() {
        let transport = MockTransport::new(vec![]);
        let result = perform_ltpa_login(&transport, "https://h", "u", "p", None, None, false);
        assert!(result.is_err());
    }
}
