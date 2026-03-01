# Examples

The `examples/` directory contains standalone programs demonstrating common
MQ administration tasks. Each example is a complete `fn main()` binary that
can be run with `cargo run --example <name>`.

## Prerequisites

Start the local MQ development environment before running examples:

```bash
./scripts/dev/mq_start.sh
./scripts/dev/mq_seed.sh
```

This starts two queue managers (`QM1` on port 9483, `QM2` on port 9484) on a
shared Docker network. See [local MQ container](development/local-mq-container.md) for details.

## Environment variables

All examples read connection details from environment variables with sensible
defaults:

| Variable | Default | Description |
| -------- | ------- | ----------- |
| `MQ_REST_BASE_URL` | `https://localhost:9483/ibmmq/rest/v2` | QM1 REST endpoint |
| `MQ_QMGR_NAME` | `QM1` | Queue manager name |
| `MQ_ADMIN_USER` | `mqadmin` | Admin username |
| `MQ_ADMIN_PASSWORD` | `mqadmin` | Admin password |
| `MQ_REST_BASE_URL_QM2` | `https://localhost:9484/ibmmq/rest/v2` | QM2 REST endpoint (multi-QM examples) |
| `DEPTH_THRESHOLD_PCT` | `80` | Warning threshold for queue depth monitor |

## Running examples

```bash
cargo run --example health_check
cargo run --example queue_depth_monitor
cargo run --example channel_status
cargo run --example queue_status
cargo run --example dlq_inspector
cargo run --example provision_environment
```

## health_check

Connects to one or more queue managers and checks QMGR status, command server
availability, and listener state. Produces a pass/fail summary for each queue
manager.

Set `MQ_REST_BASE_URL_QM2` to also check QM2.

```bash
cargo run --example health_check
```

## queue_depth_monitor

Displays local queues with their current depth, flags queues approaching
capacity, and sorts by depth percentage descending. Configure the warning
threshold with `DEPTH_THRESHOLD_PCT`.

```bash
cargo run --example queue_depth_monitor
```

## channel_status

Displays channel definitions alongside live channel status. Identifies channels
that are defined but not running and shows connection details.

```bash
cargo run --example channel_status
```

## queue_status

Demonstrates `DISPLAY QSTATUS TYPE(HANDLE)` and `DISPLAY CONN TYPE(HANDLE)`
queries, showing how `mq-rest-admin` transparently flattens the nested `objects`
response structure into uniform flat `HashMap`s.

```bash
cargo run --example queue_status
```

## dlq_inspector

Checks the dead letter queue configuration for a queue manager, reports its
depth and capacity, and suggests actions when messages are present.

```bash
cargo run --example dlq_inspector
```

## provision_environment

Defines a complete set of queues, channels, and remote queue definitions across
two queue managers, then verifies connectivity. Includes teardown to remove all
provisioned objects. Requires both QM1 and QM2 to be running.

```bash
cargo run --example provision_environment
```
