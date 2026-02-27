# Namespace origin

## How the `snake_case` namespace was created

The `snake_case` attribute namespace in `mapping-data.json` was initialized
by parsing IBM MQ 9.4 MQSC and PCF documentation using an automated
extraction pipeline. The pipeline:

1. Downloaded MQSC and PCF command reference pages from IBM documentation
2. Extracted attribute names, types, and value constants
3. Built a mapping between MQSC and PCF attribute names
4. Proposed `snake_case` equivalents for each attribute

The automated output was then reviewed, customized, and rationalized by
hand. Many names were changed, value mappings were corrected, and
qualifier-specific overrides were applied.

## Current source of truth

`mapping-data.json` at the repository root is the sole authoritative
source for all attribute mappings. It is maintained directly — not
generated from external documentation.

The mapping data is shared across all language implementations in the
mq-rest-admin family (Python, Rust, etc.) via the
`mq-rest-admin-common` repository.

## Handling future MQ versions

When IBM releases a new MQ version (for example, 9.5):

1. Compare the previous and new MQSC command reference for new, changed,
   or removed attributes
2. Propose `snake_case` names for new attributes following the
   established naming conventions in `mapping-data.json`
3. Update `mapping-data.json` directly with the new mappings
4. Regenerate downstream artifacts and verify:

   ```bash
   cargo test
   cargo clippy -- -D warnings
   ```

Re-running the archived extraction pipeline is not recommended. The
namespace has diverged significantly from what automation would produce,
and manual maintenance preserves the naming consistency that has been
built up over time.
