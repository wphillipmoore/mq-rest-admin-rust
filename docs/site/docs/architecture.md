# Architecture

## Component overview

--8<-- "architecture/component-overview.md"

In the Rust implementation, the core components map to these modules:

- **`MqRestSession`** (`session.rs`): The main entry point. Owns
  authentication, base URL construction, request/response handling, and
  diagnostic state. Constructed via `MqRestSessionBuilder`.
- **Command methods** (`commands.rs`): Provides ~148 MQSC command methods.
  Each method is a thin wrapper that calls `mqsc_command()` with the correct
  command verb and qualifier.
- **Ensure methods** (`ensure.rs`): Provides 16 idempotent `ensure_*`
  methods for declarative object management. `ensure_qmgr()` is a special
  singleton variant (no name, no DEFINE).
- **Mapping pipeline** (`mapping.rs`, `mapping_data.rs`): Bidirectional
  attribute translation between Rust `snake_case` names and native MQSC
  parameter names. See the [mapping pipeline](mapping-pipeline.md) for
  details.
- **Error types** (`error.rs`): Structured error types using `thiserror`.
  `MqRestError` is the top-level enum with variants for each failure mode.

## Request lifecycle

--8<-- "architecture/request-lifecycle.md"

In Rust, the command dispatcher is the `mqsc_command()` method on
`MqRestSession`. Every public command method (e.g. `display_queue()`,
`define_qlocal()`) delegates to it with the appropriate verb and qualifier.

The session retains diagnostic state from the most recent command for
inspection:

```rust
session.display_queue(Some("MY.QUEUE"), None, None, None)?;

session.last_command_payload();    // the JSON sent to MQ
session.last_response_payload();   // the parsed JSON response
session.last_http_status();        // HTTP status code
session.last_response_text();      // raw response body
```

## Transport abstraction

--8<-- "architecture/transport-abstraction.md"

In Rust, the transport is defined by the `MqRestTransport` trait:

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

The default implementation, `ReqwestTransport`, wraps the `reqwest` blocking
client.

For testing, inject a mock transport:

```rust
use mq_rest_admin::{MqRestSession, Credentials, MqRestTransport, TransportResponse};

struct MockTransport;

impl MqRestTransport for MockTransport {
    fn post_json(
        &self, _url: &str, _payload: &serde_json::Value,
        _headers: &HashMap<String, String>,
        _timeout: Option<f64>, _verify: bool,
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

This makes the entire command pipeline testable without an MQ server.

## Single-endpoint design

--8<-- "architecture/single-endpoint-design.md"

In Rust, this means every command method on `MqRestSession` ultimately calls
the same `post_json()` method on the transport with the same URL pattern. The
only variation is the JSON payload content.

## Gateway routing

--8<-- "architecture/gateway-routing.md"

In Rust, configure gateway routing via the builder:

```rust
let session = MqRestSession::builder()
    .rest_base_url("https://qm1-host:9443/ibmmq/rest/v2")
    .qmgr_name("QM2")           // target (remote) queue manager
    .credentials(Credentials::Ltpa {
        username: "mqadmin".into(),
        password: "mqadmin".into(),
    })
    .gateway_qmgr("QM1")        // local gateway queue manager
    .build()?;
```

## Minimal dependencies

The library depends on:

- **reqwest** — HTTP client (blocking, rustls-tls)
- **serde / serde_json** — JSON serialization
- **thiserror** — Error type derivation
- **base64** — CSRF token encoding

No other runtime dependencies are required.

## Generated command methods

The ~148 command methods in `commands.rs` are generated from the command
definitions in `mapping-data.json`. Each method:

- Accepts `name`, `request_parameters`, `response_parameters`, and
  `where_clause` (for DISPLAY commands).
- Calls `self.mqsc_command()` with the correct verb and qualifier.
- Returns `Vec<HashMap<String, Value>>` for DISPLAY commands,
  `()` for others.

Queue manager commands (`display_qmgr`, `display_qmstatus`, etc.)
have singleton helpers that return `Option<HashMap<String, Value>>`
instead of a list.

## Ensure pipeline

See [ensure](api/ensure.md) for details on the idempotent
create-or-update pipeline.

## Sync pipeline

See [sync](api/sync.md) for details on the synchronous
polling pipeline.
