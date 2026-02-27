# Sync

## The problem with fire-and-forget

All MQSC `START` and `STOP` commands are fire-and-forget — they return
immediately without waiting for the object to reach its target state.
In practice, tooling that provisions infrastructure needs to wait until
a channel is `RUNNING` or a listener is `STOPPED` before proceeding to
the next step. Writing polling loops by hand is error-prone and
clutters business logic with retry mechanics.

## The sync pattern

The `*_sync` and `restart_*` methods wrap the fire-and-forget commands
with a polling loop that issues `DISPLAY *STATUS` until the object
reaches a stable state or the timeout expires.

## SyncOperation

An enum indicating the operation that was performed:

```rust
pub enum SyncOperation {
    Started,    // Object confirmed running
    Stopped,    // Object confirmed stopped
    Restarted,  // Stop-then-start completed
}
```

## SyncConfig

Configuration controlling the polling behaviour:

```rust
pub struct SyncConfig {
    pub timeout_seconds: f64,       // Max seconds before returning error (default: 30.0)
    pub poll_interval_seconds: f64, // Seconds between polls (default: 1.0)
}
```

| Field | Type | Description |
| --- | --- | --- |
| `timeout_seconds` | `f64` | Maximum seconds to wait before returning `MqRestError::Timeout` |
| `poll_interval_seconds` | `f64` | Seconds between `DISPLAY *STATUS` polls |

## SyncResult

Contains the outcome of a sync operation:

```rust
pub struct SyncResult {
    pub operation: SyncOperation,  // What happened
    pub polls: u32,                // Number of status polls issued
    pub elapsed: f64,              // Wall-clock seconds from command to confirmation
}
```

| Field | Type | Description |
| --- | --- | --- |
| `operation` | `SyncOperation` | What happened: `Started`, `Stopped`, or `Restarted` |
| `polls` | `u32` | Number of status polls issued |
| `elapsed` | `f64` | Wall-clock seconds from command to confirmation |

## Method signature pattern

All 9 sync methods follow the same signature pattern:

```rust
pub fn start_channel_sync(
    &mut self,
    name: &str,
    config: Option<&SyncConfig>,
) -> Result<SyncResult>
```

## Basic usage

```rust
use mq_rest_admin::{SyncConfig, SyncOperation};

// Start a channel and wait until it is RUNNING
let result = session.start_channel_sync("TO.PARTNER", None)?;
assert!(matches!(result.operation, SyncOperation::Started));
println!("Channel running after {} poll(s), {:.1}s", result.polls, result.elapsed);

// Stop a listener and wait until it is STOPPED
let result = session.stop_listener_sync("TCP.LISTENER", None)?;
assert!(matches!(result.operation, SyncOperation::Stopped));
```

## Custom timeout and poll interval

Pass a `SyncConfig` to override the defaults:

```rust
use mq_rest_admin::SyncConfig;

// Aggressive polling for fast local development
let fast = SyncConfig {
    timeout_seconds: 10.0,
    poll_interval_seconds: 0.25,
};
let result = session.start_service_sync("MY.SVC", Some(&fast))?;

// Patient polling for remote queue managers
let patient = SyncConfig {
    timeout_seconds: 120.0,
    poll_interval_seconds: 5.0,
};
let result = session.start_channel_sync("REMOTE.CHL", Some(&patient))?;
```

## Restart convenience

The `restart_*` methods perform a synchronous stop followed by a
synchronous start. Each phase gets the full timeout independently —
worst case is 2x the configured timeout.

The returned `SyncResult` reports **total** polls and **total** elapsed
time across both phases:

```rust
let result = session.restart_channel("TO.PARTNER", None)?;
assert!(matches!(result.operation, SyncOperation::Restarted));
println!("Restarted in {:.1}s ({} total polls)", result.elapsed, result.polls);
```

## Timeout handling

When the timeout expires, `MqRestError::Timeout` is returned with
diagnostic context:

```rust
use mq_rest_admin::{MqRestError, SyncConfig};

let config = SyncConfig {
    timeout_seconds: 15.0,
    poll_interval_seconds: 1.0,
};

match session.start_channel_sync("BROKEN.CHL", Some(&config)) {
    Ok(result) => println!("Started in {:.1}s", result.elapsed),
    Err(MqRestError::Timeout { .. }) => {
        eprintln!("Channel did not start within timeout");
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Available methods

| Method | Operation | START/STOP qualifier | Status qualifier |
| --- | --- | --- | --- |
| `start_channel_sync()` | Start | `CHANNEL` | `CHSTATUS` |
| `stop_channel_sync()` | Stop | `CHANNEL` | `CHSTATUS` |
| `restart_channel()` | Restart | `CHANNEL` | `CHSTATUS` |
| `start_listener_sync()` | Start | `LISTENER` | `LSSTATUS` |
| `stop_listener_sync()` | Stop | `LISTENER` | `LSSTATUS` |
| `restart_listener()` | Restart | `LISTENER` | `LSSTATUS` |
| `start_service_sync()` | Start | `SERVICE` | `SVSTATUS` |
| `stop_service_sync()` | Stop | `SERVICE` | `SVSTATUS` |
| `restart_service()` | Restart | `SERVICE` | `SVSTATUS` |

## Status detection

The polling loop checks the `STATUS` attribute in the `DISPLAY *STATUS`
response. The target values are:

- **Start**: `RUNNING`
- **Stop**: `STOPPED`

### Channel stop edge case

When a channel stops, its `CHSTATUS` record may disappear entirely
(the `DISPLAY CHSTATUS` response returns no rows). The channel sync
methods treat an empty status result as successfully stopped. Listener
and service status records are always present, so empty results are not
treated as stopped for those object types.

## Attribute mapping

The sync methods call `mqsc_command()` internally, so they participate
in the same [mapping pipeline](../mapping-pipeline.md) as all other
command methods. The status key is checked using both the mapped
`snake_case` name and the raw MQSC name, so polling works correctly
regardless of whether mapping is enabled or disabled.

## Provisioning example

The sync methods pair naturally with the
[ensure methods](ensure.md) for end-to-end provisioning:

```rust
use mq_rest_admin::SyncConfig;
use std::collections::HashMap;

let config = SyncConfig {
    timeout_seconds: 60.0,
    poll_interval_seconds: 1.0,
};

// Ensure listeners exist for application and admin traffic
let mut params = HashMap::new();
params.insert("transport_type".into(), serde_json::json!("TCP"));
params.insert("port".into(), serde_json::json!(1415));
params.insert("start_mode".into(), serde_json::json!("MQSVC_CONTROL_Q_MGR"));
session.ensure_listener("APP.LISTENER", Some(params.clone()))?;

params.insert("port".into(), serde_json::json!(1416));
session.ensure_listener("ADMIN.LISTENER", Some(params))?;

// Start them synchronously
session.start_listener_sync("APP.LISTENER", Some(&config))?;
session.start_listener_sync("ADMIN.LISTENER", Some(&config))?;

println!("Listeners ready");
```
