# Errors

All errors are represented by variants of the `MqRestError` enum, which
implements `std::error::Error` via `thiserror`.

```text
MqRestError
├── Auth        — authentication failures
├── Transport   — network/connection failures
├── Response    — malformed responses
├── Command     — MQSC command failures
├── Timeout     — sync operation timeouts
└── Mapping     — attribute mapping failures
```

## MqRestError

The top-level error enum. All library operations return
`Result<T, MqRestError>`.

```rust
pub enum MqRestError {
    Transport { message: String, source: Option<Box<dyn std::error::Error + Send + Sync>> },
    Response { message: String },
    Auth { url: String, status_code: u16, message: String },
    Command { message: String, payload: serde_json::Value },
    Timeout { message: String },
    Mapping(MappingError),
}
```

## MqRestError::Transport

Returned when the HTTP request fails at the network level — connection refused,
DNS resolution failure, TLS handshake error, etc.

```rust
use mq_rest_admin::MqRestError;

match session.display_queue(Some("MY.QUEUE"), None, None, None) {
    Err(MqRestError::Transport { message, .. }) => {
        eprintln!("Cannot reach MQ: {}", message);
    }
    _ => {}
}
```

## MqRestError::Response

Returned when the HTTP request succeeds but the response cannot be parsed —
invalid JSON, missing expected fields, unexpected response structure.

## MqRestError::Auth

Returned when authentication or authorization fails — invalid credentials,
expired tokens, insufficient permissions (HTTP 401/403).

```rust
use mq_rest_admin::MqRestError;

match session.display_qmgr(None, None) {
    Err(MqRestError::Auth { status_code, message, .. }) => {
        eprintln!("Authentication failed ({}): {}", status_code, message);
    }
    _ => {}
}
```

## MqRestError::Command

Returned when the MQSC command returns a non-zero completion or reason code.
This is the most commonly caught error — it indicates the command was delivered
to MQ but the queue manager rejected it.

```rust
use mq_rest_admin::MqRestError;

match session.define_qlocal(Some("MY.QUEUE"), None, None, None) {
    Err(MqRestError::Command { message, payload }) => {
        eprintln!("Command failed: {}", message);
        eprintln!("Response payload: {}", payload);
    }
    _ => {}
}
```

!!! note
    For DISPLAY commands with no matches, MQ returns reason code 2085
    (MQRC_UNKNOWN_OBJECT_NAME). The library treats this as an empty `Vec`
    rather than returning an error.

## MqRestError::Timeout

Returned when a polling operation exceeds the configured timeout duration.

```rust
use mq_rest_admin::{MqRestError, SyncConfig};

let config = SyncConfig {
    timeout_seconds: 15.0,
    poll_interval_seconds: 1.0,
};

match session.start_channel_sync("BROKEN.CHL", Some(&config)) {
    Err(MqRestError::Timeout { message }) => {
        eprintln!("Timed out: {}", message);
    }
    _ => {}
}
```

## MqRestError::Mapping

Returned when attribute mapping fails in strict mode. Wraps a `MappingError`
containing the list of issues.

```rust
use mq_rest_admin::MqRestError;

match session.display_queue(Some("MY.QUEUE"), None,
    Some(vec!["invalid_name".into()]), None)
{
    Err(MqRestError::Mapping(e)) => {
        eprintln!("Mapping failed: {}", e.message);
        for issue in &e.issues {
            eprintln!("  - {}: {}", issue.attribute_name, issue.reason);
        }
    }
    _ => {}
}
```

## Catching errors

Match specific variants for targeted recovery, or use the `?` operator
to propagate all errors:

```rust
use mq_rest_admin::MqRestError;

match session.define_qlocal(Some("MY.QUEUE"), Some(params), None, None) {
    Ok(()) => println!("Queue created"),
    Err(MqRestError::Command { message, .. }) => {
        // MQSC command failed — check reason code in payload
        eprintln!("Command failed: {}", message);
    }
    Err(MqRestError::Auth { .. }) => {
        // Credentials rejected
        eprintln!("Not authorized");
    }
    Err(MqRestError::Transport { .. }) => {
        // Network error
        eprintln!("Connection failed");
    }
    Err(e) => {
        // Catch-all for any other error
        eprintln!("Unexpected error: {}", e);
    }
}
```
