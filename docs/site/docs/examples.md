# Examples

!!! note
    The `examples/` directory is not yet available in the Rust implementation.
    Example programs demonstrating common MQ administration tasks are planned
    for a future release.

## Planned examples

The following examples from the Python reference implementation will be
adapted for Rust:

- **Health check** — Connect to queue managers and verify status
- **Queue depth monitor** — List queues and flag those approaching capacity
- **Channel status report** — Cross-reference definitions with live status
- **Queue status and handles** — Demonstrate nested object flattening
- **Dead letter queue inspector** — Inspect DLQ configuration and depth
- **Environment provisioner** — Bulk provisioning across multiple queue managers

## Prerequisites

When examples are available, they will use the same local Docker environment:

```bash
./scripts/dev/mq_start.sh
./scripts/dev/mq_seed.sh
```

This starts two queue managers (`QM1` on port 9443, `QM2` on port 9444) on a
shared Docker network. See [local MQ container](development/local-mq-container.md) for details.

## Basic usage pattern

Until dedicated examples are available, here is the basic pattern for using
the library:

```rust
use mq_rest_admin::{MqRestSession, Credentials, MqRestError};

fn main() -> Result<(), MqRestError> {
    let session = MqRestSession::builder()
        .rest_base_url("https://localhost:9443/ibmmq/rest/v2")
        .qmgr_name("QM1")
        .credentials(Credentials::Ltpa {
            username: "mqadmin".into(),
            password: "mqadmin".into(),
        })
        .verify_tls(false)
        .build()?;

    // Health check
    if let Some(qmgr) = session.display_qmgr(None, None)? {
        println!("Queue manager: {:?}", qmgr.get("queue_manager_name"));
    }

    // List queues
    let queues = session.display_queue(
        Some("DEV.*"),
        None,
        Some(vec!["current_queue_depth".into(), "max_queue_depth".into()]),
        None,
    )?;

    for queue in &queues {
        println!("{}: depth {:?}",
            queue.get("queue_name").unwrap(),
            queue.get("current_queue_depth"));
    }

    Ok(())
}
```
