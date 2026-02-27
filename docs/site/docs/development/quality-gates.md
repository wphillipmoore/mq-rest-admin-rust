# Quality gates

--8<-- "development/quality-gates.md"

## Rust-specific validation

The Rust validation pipeline runs via containerized scripts:

```bash
scripts/dev/test.sh       # Unit tests
scripts/dev/lint.sh        # Clippy + fmt checks
scripts/dev/audit.sh       # cargo-deny vulnerability scanning
```

This executes:

1. **cargo clippy** — Lint with all warnings denied (`-D warnings`)
2. **cargo fmt --check** — Formatting verification
3. **cargo test** — Unit tests
4. **cargo-deny check** — Dependency vulnerability scanning, license
   compliance, and duplicate dependency detection

The CI matrix tests against the minimum supported Rust version (1.92).
