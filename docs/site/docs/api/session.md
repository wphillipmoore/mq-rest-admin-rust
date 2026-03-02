# Session

## Overview

The `MqRestSession` struct is the main entry point for interacting with
IBM MQ via the REST API. A session encapsulates connection details,
authentication, attribute mapping configuration, and diagnostic state.
It provides MQSC command methods (see [commands](commands.md)),
idempotent ensure methods (see [ensure](ensure.md)), and synchronous
sync methods (see [sync](sync.md)).

Sessions are constructed via the builder pattern using
`MqRestSession::builder()`.

## Creating a session

```rust
use mq_rest_admin::{MqRestSession, Credentials};

let session = MqRestSession::builder()
    .rest_base_url("https://localhost:9443/ibmmq/rest/v2")
    .qmgr_name("QM1")
    .credentials(Credentials::Ltpa {
        username: "mqadmin".into(),
        password: "mqadmin".into(),
    })
    .build()?;
```

The builder validates all required fields and constructs the transport,
mapping data, and authentication state at creation time. Errors in
configuration (e.g. invalid mapping overrides) are caught immediately.

## Builder methods

| Method | Type | Description |
| --- | --- | --- |
| `rest_base_url()` | Required | Base URL of the MQ REST API (e.g. `https://host:9443/ibmmq/rest/v2`) |
| `qmgr_name()` | Required | Target queue manager name |
| `credentials()` | Required | Authentication credentials (`Credentials::Ltpa`, `Basic`, or `Certificate`) |
| `gateway_qmgr()` | Optional | Gateway queue manager for remote routing |
| `map_attributes()` | Optional | Enable/disable attribute mapping (default: `true`) |
| `mapping_strict()` | Optional | Strict or lenient mapping mode (default: `true`) |
| `mapping_overrides()` | Optional | Custom mapping overrides as `serde_json::Value` |
| `verify_tls()` | Optional | Verify server TLS certificates (default: `true`) |
| `timeout()` | Optional | Default request timeout in seconds |
| `csrf_token()` | Optional | Custom CSRF token value |
| `transport()` | Optional | Custom transport implementation (`Box<dyn MqRestTransport>`) |

### Minimal example

```rust
let session = MqRestSession::builder()
    .rest_base_url("https://localhost:9443/ibmmq/rest/v2")
    .qmgr_name("QM1")
    .credentials(Credentials::Ltpa {
        username: "mqadmin".into(),
        password: "mqadmin".into(),
    })
    .build()?;
```

### Full example

```rust
let session = MqRestSession::builder()
    .rest_base_url("https://mq-server.example.com:9443/ibmmq/rest/v2")
    .qmgr_name("QM2")
    .credentials(Credentials::Ltpa {
        username: "mqadmin".into(),
        password: "mqadmin".into(),
    })
    .gateway_qmgr("QM1")
    .map_attributes(true)
    .mapping_strict(false)
    .mapping_overrides(overrides)
    .verify_tls(true)
    .timeout(30.0)
    .build()?;
```

## Command methods

The session provides ~148 command methods, one for each MQSC verb + qualifier
combination. See [Commands](commands.md) for the full list.

```rust
// DISPLAY commands return a Vec of HashMaps
let queues = session.display_queue(Some("APP.*"), None, None, None)?;

// Queue manager singletons return Option<HashMap>
let qmgr = session.display_qmgr(None, None)?;

// Non-DISPLAY commands return () on success, Err on failure
session.define_qlocal(Some("MY.QUEUE"), Some(params), None, None)?;
session.delete_queue(Some("MY.QUEUE"), None, None, None)?;
```

## Ensure methods

The session provides 16 ensure methods for declarative object management. Each
method implements an idempotent upsert: DEFINE if the object does not exist,
ALTER only the attributes that differ, or no-op if already correct.

```rust
let result = session.ensure_qlocal("MY.QUEUE", Some(params))?;
// result.action is EnsureAction::Created, Updated, or Unchanged
```

See [Ensure](ensure.md) for detailed usage and the full list
of available ensure methods.

## Diagnostic state

The session retains the most recent request and response for inspection. This
is useful for debugging command failures or understanding what the library sent
to the MQ REST API:

```rust
session.display_queue(Some("MY.QUEUE"), None, None, None)?;

session.last_command_payload();    // the JSON sent to MQ
session.last_response_payload();   // the parsed JSON response
session.last_http_status();        // HTTP status code
session.last_response_text();      // raw response body
```

### Diagnostic properties

| Property | Type | Description |
| --- | --- | --- |
| `qmgr_name` | `&str` | Queue manager name |
| `gateway_qmgr` | `Option<&str>` | Gateway queue manager (or `None`) |
| `last_http_status()` | `u16` | HTTP status code from last command |
| `last_response_text()` | `&str` | Raw response body from last command |
| `last_response_payload()` | `&Value` | Parsed response from last command |
| `last_command_payload()` | `&Value` | Command sent in last request |

## Transport

See [Transport](transport.md) for the transport trait, response type,
and mock transport examples.

## docs.rs

For generated API documentation, see
[mq-rest-admin on docs.rs](https://docs.rs/mq-rest-admin).
