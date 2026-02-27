# AI-assisted engineering

--8<-- "ai-engineering.md"

## Rust-specific quality standards

**Comprehensive testing**: All public APIs are covered by unit tests.
Coverage is enforced as a CI gate.

**Strict linting**: Clippy runs with default warnings denied. All code
must pass `cargo clippy -- -D warnings`.

**Consistent formatting**: `cargo fmt` enforces standard Rust formatting
across the entire source tree.

**Dependency auditing**: `cargo-deny` checks for known vulnerabilities,
license compliance, and duplicate dependencies.

**Validation pipeline**: `scripts/dev/test.sh`, `scripts/dev/lint.sh`, and
`scripts/dev/audit.sh` run the same checks as CI inside Docker containers.
