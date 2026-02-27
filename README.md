# mqrest-rust

Rust wrapper for the IBM MQ administrative REST API.

`mq-rest-admin` provides typed Rust functions for every MQSC command
exposed by the IBM MQ 9.4 `runCommandJSON` REST endpoint. Attribute names are
automatically translated between Rust idioms and native MQSC parameter
names.

## Table of Contents

- [Installation](#installation)
- [Quick start](#quick-start)
- [Documentation](#documentation)
- [Development](#development)
- [License](#license)

## Installation

```bash
cargo add mq-rest-admin
```

Requires Rust 1.92+.

## Quick start

```rust
use mq_rest_admin::MqRestSession;

let session = MqRestSession::builder("https://localhost:9443/ibmmq/rest/v2")
    .basic_auth("mqadmin", "mqadmin")
    .build()?;

let queues = session.display_queue(Some("DEV.*"), None, None)?;
for queue in &queues {
    println!("{:?}", queue);
}
```

## Documentation

Full documentation: <https://wphillipmoore.github.io/mq-rest-admin-rust/>

## Development

```bash
cargo build
```

## License

GPL-3.0-or-later. See `LICENSE`.
