# Getting started

## Prerequisites

- **Rust**: 1.92 or later (edition 2024)
- **IBM MQ**: A running queue manager with the administrative REST API enabled

## Installation

Add `mq-rest-admin` to your project:

```bash
cargo add mq-rest-admin
```

## Creating a session

All interaction with IBM MQ goes through an `MqRestSession`. You need the
REST API base URL, queue manager name, and credentials:

```rust
use mq_rest_admin::{MqRestSession, Credentials};

let session = MqRestSession::builder()
    .rest_base_url("https://localhost:9443/ibmmq/rest/v2")
    .qmgr_name("QM1")
    .credentials(Credentials::Ltpa {
        username: "mqadmin".into(),
        password: "mqadmin".into(),
    })
    .verify_tls(false)  // for local development only
    .build()?;
```

## Running a command

Every MQSC command has a corresponding method on the session. Method names
follow the pattern `<verb>_<qualifier>` in lowercase:

```rust
// DISPLAY QUEUE — returns a Vec of HashMaps
let queues = session.display_queue(Some("SYSTEM.*"), None, None, None)?;

for queue in &queues {
    println!("{} {:?}",
        queue.get("queue_name").unwrap(),
        queue.get("current_queue_depth"));
}
```

```rust
// DISPLAY QMGR — returns Option<HashMap>
if let Some(qmgr) = session.display_qmgr(None, None)? {
    println!("{}", qmgr.get("queue_manager_name").unwrap());
}
```

## Attribute mapping

By default, `mq-rest-admin` maps between Rust-friendly `snake_case` names and
MQSC parameter names. This applies to both request and response attributes:

```rust
use std::collections::HashMap;

// With mapping enabled (default)
let queues = session.display_queue(
    Some("MY.QUEUE"),
    None,
    Some(vec!["current_queue_depth".into(), "max_queue_depth".into()]),
    None,
)?;
// Returns: [{"queue_name": "MY.QUEUE", "current_queue_depth": 0, "max_queue_depth": 5000}]
```

See [mapping pipeline](mapping-pipeline.md) for a detailed explanation of how mapping works.

## Strict vs lenient mapping

By default, mapping runs in strict mode. Unknown attribute names or values
return a `MqRestError::Mapping` error. In lenient mode, unknown attributes
pass through unchanged:

```rust
let session = MqRestSession::builder()
    .rest_base_url("https://localhost:9443/ibmmq/rest/v2")
    .qmgr_name("QM1")
    .credentials(Credentials::Ltpa {
        username: "mqadmin".into(),
        password: "mqadmin".into(),
    })
    .mapping_strict(false)
    .build()?;
```

## Custom mapping overrides

Sites with existing naming conventions can override individual entries in the
built-in mapping tables without forking or replacing them entirely. Pass
`mapping_overrides` when building the session:

```rust
use serde_json::json;

let overrides = json!({
    "qualifiers": {
        "queue": {
            "response_key_map": {
                "CURDEPTH": "queue_depth",
                "MAXDEPTH": "queue_max_depth"
            }
        }
    }
});

let session = MqRestSession::builder()
    .rest_base_url("https://localhost:9443/ibmmq/rest/v2")
    .qmgr_name("QM1")
    .credentials(Credentials::Ltpa {
        username: "mqadmin".into(),
        password: "mqadmin".into(),
    })
    .verify_tls(false)
    .mapping_overrides(overrides)
    .build()?;

let queues = session.display_queue(Some("MY.QUEUE"), None, None, None)?;
// Returns: [{"queue_depth": 0, "queue_max_depth": 5000, ...}]
```

Overrides are **sparse** — you only specify the entries you want to change. All
other mappings in the qualifier continue to work as normal.

Overrides support all five sub-maps per qualifier: `request_key_map`,
`request_value_map`, `request_key_value_map`, `response_key_map`, and
`response_value_map`. See [mapping pipeline](mapping-pipeline.md) for details on how each
sub-map is used.

Invalid override structures return an error at session construction time, so
errors are caught early.

## Gateway queue manager

The MQ REST API is available on all supported IBM MQ platforms (Linux, AIX,
Windows, z/OS, and IBM i). mq-rest-admin is developed and tested against the
**Linux** implementation only.

In enterprise environments, a **gateway queue manager** can route MQSC
commands to remote queue managers via MQ channels — the same mechanism used
by `runmqsc -w` and the MQ Console.

To use a gateway, pass `gateway_qmgr` when building the session. The
`qmgr_name` specifies the **target** (remote) queue manager, while
`gateway_qmgr` names the **local** queue manager whose REST API routes the
command:

```rust
use mq_rest_admin::{MqRestSession, Credentials};

// Route commands to QM2 through QM1's REST API
let session = MqRestSession::builder()
    .rest_base_url("https://qm1-host:9443/ibmmq/rest/v2")
    .qmgr_name("QM2")
    .credentials(Credentials::Basic {
        username: "mqadmin".into(),
        password: "mqadmin".into(),
    })
    .gateway_qmgr("QM1")
    .verify_tls(false)
    .build()?;

let qmgr = session.display_qmgr(None, None)?;
// Returns QM2's queue manager attributes, routed through QM1
```

Prerequisites:

- The gateway queue manager must have a running REST API.
- MQ channels must be configured between the gateway and target queue managers.
- A QM alias (QREMOTE with empty RNAME) must map the target QM name to the
  correct transmission queue on the gateway.

## Error handling

`DISPLAY` commands return an empty `Vec` when no objects match. Queue manager
display methods return `None` when no match is found. Non-display commands
return `Err(MqRestError::Command)` on failure:

```rust
use mq_rest_admin::MqRestError;

// Empty Vec — no error
let result = session.display_queue(Some("NONEXISTENT.*"), None, None, None)?;
assert!(result.is_empty());

// Define returns an error on failure
match session.define_qlocal(Some("MY.QUEUE"), None, None, None) {
    Ok(_) => println!("Queue created"),
    Err(MqRestError::Command { payload, .. }) => {
        eprintln!("Command failed: {:?}", payload);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Diagnostic state

The session retains the most recent request and response for inspection:

```rust
session.display_queue(Some("MY.QUEUE"), None, None, None)?;

println!("{:?}", session.last_command_payload());    // the JSON sent to MQ
println!("{:?}", session.last_response_payload());   // the parsed JSON response
println!("{}", session.last_http_status());           // HTTP status code
println!("{}", session.last_response_text());         // raw response body
```
