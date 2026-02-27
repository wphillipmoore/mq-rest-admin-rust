# Generation scripts

Several artifacts in the repository are generated from the mapping data
in `mapping-data.json`. This page documents how to regenerate them.

## Mapping data

The `mapping-data.json` file at the repository root contains all qualifier
definitions, attribute name mappings, value mappings, and command metadata.
It is maintained directly and is the sole source of truth for attribute
mappings. See [Namespace origin](namespace-origin.md) for the history of
how this namespace was bootstrapped.

The mapping data is embedded into the binary at compile time via
`include_str!()` in `src/mapping_data.rs`, so no runtime file I/O is
needed.

## Command methods

The MQSC command wrapper methods in `src/commands.rs` are generated from
the command definitions in `mapping-data.json`. Each method is a thin
wrapper that calls `mqsc_command()` with the correct verb and qualifier.

## Mapping documentation

The per-qualifier mapping reference pages are maintained in the shared
`mq-rest-admin-common` repository and included via `--8<--` snippet
directives. See [qualifier mapping reference](../mappings/index.md)
for the complete reference.

## Regeneration workflow

When the mapping data changes:

1. Update `mapping-data.json` with the new mappings
2. Regenerate command methods in `src/commands.rs`
3. Run the full test suite to verify:

```bash
cargo test
cargo clippy -- -D warnings
```
