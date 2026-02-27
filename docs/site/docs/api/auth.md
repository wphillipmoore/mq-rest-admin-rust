# Authentication

The authentication module provides credential types for the three
authentication modes supported by the IBM MQ REST API: mutual TLS (mTLS)
client certificates, LTPA token, and HTTP Basic.

Pass a credential variant to `MqRestSessionBuilder` via the
`.credentials()` method. Always use TLS (`https://`) for production
deployments to protect credentials and data in transit.

```rust
use mq_rest_admin::{MqRestSession, Credentials};

// mTLS client certificate auth — strongest; no shared secrets
let session = MqRestSession::builder()
    .rest_base_url("https://...")
    .qmgr_name("QM1")
    .credentials(Credentials::Certificate {
        cert_path: "/cert.pem".into(),
        key_path: Some("/key.pem".into()),
    })
    .build()?;

// LTPA token auth — credentials sent once at login, then cookie-based
let session = MqRestSession::builder()
    .rest_base_url("https://...")
    .qmgr_name("QM1")
    .credentials(Credentials::Ltpa {
        username: "user".into(),
        password: "pass".into(),
    })
    .build()?;

// Basic auth — credentials sent with every request
let session = MqRestSession::builder()
    .rest_base_url("https://...")
    .qmgr_name("QM1")
    .credentials(Credentials::Basic {
        username: "user".into(),
        password: "pass".into(),
    })
    .build()?;
```

## Credential types

### `Credentials::Certificate`

Mutual TLS authentication using a client certificate and optional private key.

| Field | Type | Description |
| --- | --- | --- |
| `cert_path` | `String` | Path to the PEM-encoded client certificate |
| `key_path` | `Option<String>` | Path to the PEM-encoded private key (if separate from cert) |

### `Credentials::Ltpa`

LTPA token-based authentication. Sends username and password to the `/login`
endpoint once, then uses the `LtpaToken2` cookie for subsequent requests.

| Field | Type | Description |
| --- | --- | --- |
| `username` | `String` | MQ admin username |
| `password` | `String` | MQ admin password |

### `Credentials::Basic`

HTTP Basic authentication. Sends credentials with every request as a
Base64-encoded `Authorization` header.

| Field | Type | Description |
| --- | --- | --- |
| `username` | `String` | MQ admin username |
| `password` | `String` | MQ admin password |

## Choosing between LTPA and Basic authentication

Both LTPA and Basic authentication use a username and password. The key
difference is how often those credentials cross the wire.

**LTPA is the recommended choice for username/password authentication.**
Credentials are sent once during the `/login` request; subsequent API
calls carry only the LTPA cookie. This reduces credential exposure and
is more efficient for sessions that issue many commands. All examples
and documentation in this project use LTPA as the default.

**Use Basic authentication as a fallback when:**

- The mqweb configuration does not enable the `/login` endpoint (for
  example, minimal container images that only expose the REST API).
- A reverse proxy or API gateway handles authentication and forwards a
  Basic auth header; cookie-based flows may not survive the proxy.
- Single-command scripts where the login round-trip doubles the request
  count for no security benefit.
- Long-running sessions where LTPA token expiry (typically two hours)
  could cause mid-operation failures; mq-rest-admin does not currently
  re-authenticate automatically.
- Local development or CI against a `localhost` container, where
  transport security is not a concern.
