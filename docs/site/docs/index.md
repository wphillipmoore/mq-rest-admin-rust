# mq-rest-admin

## Overview

**mq-rest-admin** provides a Rust-friendly interface to IBM MQ queue manager
administration via the `runCommandJSON` REST endpoint. It translates between
Rust `snake_case` attribute names and native MQSC parameter names, wraps
every MQSC command as a typed method, and handles authentication, CSRF tokens,
and error propagation.

## Key features

- **~148 command methods** covering all MQSC verbs and qualifiers
- **Bidirectional attribute mapping** between developer-friendly names and MQSC parameters
- **Idempotent ensure methods** for declarative object management
- **Bulk sync operations** for configuration-as-code workflows
- **Minimal dependencies** — reqwest, serde, thiserror, base64
- **Transport abstraction** for easy testing with mock transports
- **Builder pattern** for ergonomic session construction

## Installation

Add to your `Cargo.toml`:

```bash
cargo add mq-rest-admin
```

## Status

This project is in **beta**. The API surface, mapping tables, and return
shapes are stable but may evolve.

## License

GNU General Public License v3.0
