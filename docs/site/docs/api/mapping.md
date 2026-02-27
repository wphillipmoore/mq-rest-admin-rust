# Mapping

## Overview

The mapping module provides bidirectional attribute translation between
developer-friendly `snake_case` names and native MQSC parameter names. The
mapper is used internally by `MqRestSession` and is not typically called
directly.

See [Mapping Pipeline](../mapping-pipeline.md) for a conceptual overview of
how mapping works.

## Mapping functions

The module exposes three public functions that perform the actual translation.
These are called internally by `MqRestSession` during command execution:

- **`map_request_attributes()`** — Translates request parameters from
  `snake_case` to MQSC before sending to the REST API. Performs key mapping,
  value mapping, and key-value mapping in sequence.

- **`map_response_attributes()`** — Translates a single response map from
  MQSC to `snake_case` after receiving from the REST API.

- **`map_response_list()`** — Translates a list of response maps (the common
  return type for DISPLAY commands).

The mapper performs three types of translation in each direction:

- **Key mapping**: Attribute name translation (e.g. `current_queue_depth` ↔
  `CURDEPTH`)
- **Value mapping**: Enumerated value translation (e.g. `"yes"` ↔ `"YES"`,
  `"server_connection"` ↔ `"SVRCONN"`)
- **Key-value mapping**: Combined name+value translation for cases where both
  key and value change together (e.g. `channel_type="server_connection"` →
  `CHLTYPE("SVRCONN")`)

## Mapping data

The mapping tables are loaded from a JSON resource embedded at compile time:

```text
mapping-data.json
```

The data is loaded via `include_str!()` and parsed into a `LazyLock<Value>`
static. This means the mapping data is always available without any runtime
file I/O.

The data is organized by qualifier (e.g. `queue`, `channel`, `qmgr`) with
separate maps for request and response directions. Each qualifier contains:

- `request_key_map` — `snake_case` → MQSC key mapping for requests
- `request_value_map` — value translations for request attributes
- `request_key_value_map` — combined key+value translations for requests
- `response_key_map` — MQSC → `snake_case` key mapping for responses
- `response_value_map` — value translations for response attributes

The mapping data was originally bootstrapped from IBM MQ 9.4 documentation and
covers all standard MQSC attributes across 42 qualifiers.

## Diagnostics

### MappingIssue

Tracks mapping problems encountered during translation:

```rust
pub struct MappingIssue {
    pub direction: String,        // "request" or "response"
    pub reason: String,           // e.g. "unknown_key", "unknown_value"
    pub attribute_name: String,   // the attribute that failed mapping
    pub attribute_value: Option<String>,
    pub object_index: Option<usize>,
    pub qualifier: String,
}
```

- Unknown attribute names (not found in key map)
- Unknown attribute values (not found in value map)
- Ambiguous mappings

In strict mode, any `MappingIssue` causes a `MappingError`. In lenient
mode, issues are collected but the unmapped values pass through unchanged.

### MappingError

Returned when attribute mapping fails in strict mode. Contains the list of
`MappingIssue` instances that caused the failure:

```rust
pub struct MappingError {
    pub message: String,
    pub issues: Vec<MappingIssue>,
}
```

```rust
match session.display_queue(
    Some("MY.QUEUE"),
    None,
    Some(vec!["invalid_attribute_name".into()]),
    None,
) {
    Err(MqRestError::Mapping(e)) => {
        // e.issues describes the unmappable attributes
        eprintln!("{}", e.message);
    }
    _ => {}
}
```
