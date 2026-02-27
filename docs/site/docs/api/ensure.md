# Ensure

## The problem with ALTER

Every `alter_*()` call sends an `ALTER` command to the queue manager,
even when every specified attribute already matches the current state.
MQ updates `ALTDATE` and `ALTTIME` on every `ALTER`, regardless of
whether any values actually changed. This makes `ALTER` unsuitable for
declarative configuration management where idempotency matters — running
the same configuration twice should not corrupt audit timestamps.

## The ensure pattern

The `ensure_*()` methods implement a declarative upsert pattern:

1. **DEFINE** the object when it does not exist.
2. **ALTER** only the attributes that differ from the current state.
3. **Do nothing** when all specified attributes already match,
   preserving `ALTDATE` and `ALTTIME`.

## EnsureAction

An enum indicating the action taken by an ensure method:

```rust
pub enum EnsureAction {
    Created,    // Object did not exist; DEFINE was issued
    Updated,    // Object existed but attributes differed; ALTER was issued
    Unchanged,  // Object already matched the desired state
}
```

## EnsureResult

A struct containing the action taken and the list of attribute names that
triggered the change (if any):

```rust
pub struct EnsureResult {
    pub action: EnsureAction,   // What happened
    pub changed: Vec<String>,   // Attribute names that differed (empty for Created/Unchanged)
}
```

| Field | Type | Description |
| --- | --- | --- |
| `action` | `EnsureAction` | What happened: `Created`, `Updated`, or `Unchanged` |
| `changed` | `Vec<String>` | Attribute names that triggered an ALTER (in the caller's namespace) |

## Method signature pattern

Most methods share the same signature:

```rust
pub fn ensure_qlocal(
    &mut self,
    name: &str,
    request_parameters: Option<HashMap<String, Value>>,
) -> Result<EnsureResult>
```

The queue manager ensure method omits the name parameter:

```rust
pub fn ensure_qmgr(
    &mut self,
    request_parameters: Option<HashMap<String, Value>>,
) -> Result<EnsureResult>
```

`response_parameters` is not exposed — the ensure logic always requests
`["all"]` internally so it can compare the full current state.

## Basic usage

```rust
use mq_rest_admin::{EnsureAction, EnsureResult};
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert("max_queue_depth".into(), serde_json::json!(50000));
params.insert("description".into(), serde_json::json!("Application request queue"));

// First call — queue does not exist yet
let result = session.ensure_qlocal("APP.REQUEST.Q", Some(params.clone()))?;
assert!(matches!(result.action, EnsureAction::Created));

// Second call — same attributes, nothing to change
let result = session.ensure_qlocal("APP.REQUEST.Q", Some(params.clone()))?;
assert!(matches!(result.action, EnsureAction::Unchanged));

// Third call — description changed, only that attribute is altered
params.insert("description".into(), serde_json::json!("Updated request queue"));
let result = session.ensure_qlocal("APP.REQUEST.Q", Some(params))?;
assert!(matches!(result.action, EnsureAction::Updated));
assert!(result.changed.contains(&"description".to_string()));
```

## Comparison logic

The ensure methods compare only the attributes the caller passes in
`request_parameters` against the current state returned by `DISPLAY`.
Attributes not specified by the caller are ignored.

Comparison is:

- **Case-insensitive** — `"ENABLED"` matches `"enabled"`.
- **Type-normalizing** — integer `5000` matches string `"5000"`.
- **Whitespace-trimming** — `" YES "` matches `"YES"`.

An attribute present in `request_parameters` but absent from the
`DISPLAY` response is treated as changed and included in the `ALTER`.

## Selective ALTER

When an update is needed, only the changed attributes are sent in the
`ALTER` command. Attributes that already match are excluded from the
request. This minimizes the scope of each `ALTER` to the strict delta.

## Available methods

Each method targets a specific MQ object type with the correct
MQSC qualifier triple (DISPLAY / DEFINE / ALTER):

| Method | Object type | DISPLAY | DEFINE | ALTER |
| --- | --- | --- | --- | --- |
| `ensure_qmgr()` | Queue manager | `QMGR` | — | `QMGR` |
| `ensure_qlocal()` | Local queue | `QUEUE` | `QLOCAL` | `QLOCAL` |
| `ensure_qremote()` | Remote queue | `QUEUE` | `QREMOTE` | `QREMOTE` |
| `ensure_qalias()` | Alias queue | `QUEUE` | `QALIAS` | `QALIAS` |
| `ensure_qmodel()` | Model queue | `QUEUE` | `QMODEL` | `QMODEL` |
| `ensure_channel()` | Channel | `CHANNEL` | `CHANNEL` | `CHANNEL` |
| `ensure_authinfo()` | Auth info | `AUTHINFO` | `AUTHINFO` | `AUTHINFO` |
| `ensure_listener()` | Listener | `LISTENER` | `LISTENER` | `LISTENER` |
| `ensure_namelist()` | Namelist | `NAMELIST` | `NAMELIST` | `NAMELIST` |
| `ensure_process()` | Process | `PROCESS` | `PROCESS` | `PROCESS` |
| `ensure_service()` | Service | `SERVICE` | `SERVICE` | `SERVICE` |
| `ensure_topic()` | Topic | `TOPIC` | `TOPIC` | `TOPIC` |
| `ensure_sub()` | Subscription | `SUB` | `SUB` | `SUB` |
| `ensure_stgclass()` | Storage class | `STGCLASS` | `STGCLASS` | `STGCLASS` |
| `ensure_comminfo()` | Comm info | `COMMINFO` | `COMMINFO` | `COMMINFO` |
| `ensure_cfstruct()` | CF structure | `CFSTRUCT` | `CFSTRUCT` | `CFSTRUCT` |

### Queue manager (singleton)

`ensure_qmgr()` has no `name` parameter because the queue manager is a
singleton that always exists. It can only return `Updated` or
`Unchanged` (never `Created`).

This makes it ideal for asserting queue manager-level settings such as
statistics, monitoring, events, and logging attributes without
corrupting `ALTDATE`/`ALTTIME` on every run.

## Attribute mapping

The ensure methods participate in the same
[mapping pipeline](../mapping-pipeline.md) as all other command methods.
Pass `snake_case` attribute names in `request_parameters` and the
mapping layer translates them to MQSC names for the DISPLAY, DEFINE,
and ALTER commands automatically.

## Configuration management example

The ensure pattern is designed for scripts that declare desired state:

```rust
use std::collections::HashMap;
use serde_json::json;

fn configure_queue_manager(session: &mut MqRestSession) -> Result<(), MqRestError> {
    // Ensure queue manager attributes are set for production
    let mut qmgr_params = HashMap::new();
    qmgr_params.insert("queue_statistics".into(), json!("on"));
    qmgr_params.insert("channel_statistics".into(), json!("on"));
    qmgr_params.insert("queue_monitoring".into(), json!("medium"));
    qmgr_params.insert("channel_monitoring".into(), json!("medium"));

    let result = session.ensure_qmgr(Some(qmgr_params))?;
    println!("Queue manager: {:?}", result.action);

    let queues = vec![
        ("APP.REQUEST.Q", json!({"max_queue_depth": 50000, "default_persistence": "yes"})),
        ("APP.REPLY.Q", json!({"max_queue_depth": 10000, "default_persistence": "no"})),
        ("APP.DLQ", json!({"max_queue_depth": 100000, "default_persistence": "yes"})),
    ];

    for (name, attrs) in queues {
        let params: HashMap<String, serde_json::Value> =
            serde_json::from_value(attrs)?;
        let result = session.ensure_qlocal(name, Some(params))?;
        println!("{}: {:?}", name, result.action);
    }

    Ok(())
}
```

Running this function repeatedly produces no side effects when the
configuration is already correct. Only genuine changes trigger `ALTER`
commands, keeping `ALTDATE`/`ALTTIME` accurate.
