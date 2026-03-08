# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

<!-- include: docs/standards-and-conventions.md -->
<!-- include: docs/repository-standards.md -->

## Auto-memory policy

**Do NOT use MEMORY.md.** Never write to MEMORY.md or any file under the
memory directory. All behavioral rules, conventions, and workflow instructions
belong in managed, version-controlled documentation (CLAUDE.md, AGENTS.md,
skills, or docs/). If you want to persist something, tell the human what you
would save and let them decide where it belongs.

## Project Overview

`mq-rest-admin` is a Rust wrapper for the IBM MQ administrative REST
API. The project provides typed Rust functions for every MQSC command
exposed by the `runCommandJSON` REST endpoint, with automatic attribute name
translation between Rust idioms and native MQSC parameter names.

**Project name**: mqrest-rust

**Status**: Pre-alpha (initial setup)

## Development Commands

### Standard Tooling

```bash
cd ../standard-tooling && uv sync                                                # Install standard-tooling
export PATH="../standard-tooling/.venv/bin:../standard-tooling/scripts/bin:$PATH" # Put tools on PATH
git config core.hooksPath ../standard-tooling/scripts/lib/git-hooks               # Enable git hooks
```

### Three-Tier CI Model

Testing is split across three tiers with increasing scope and cost:

**Tier 1 — Local pre-commit (seconds):** Fast smoke tests in a single
container. Run before every commit. No MQ, no matrix.

```bash
./scripts/dev/test.sh        # Tests in dev-rust:1.93
./scripts/dev/lint.sh        # Lint checks in dev-rust:1.93
./scripts/dev/typecheck.sh   # Type checking in dev-rust:1.93
./scripts/dev/audit.sh       # Security audit in dev-rust:1.93
```

**Tier 2 — Push CI (~3-5 min):** Triggers automatically on push to
`feature/**`, `bugfix/**`, `hotfix/**`, `chore/**`. Single language version
(1.93), includes integration tests, no security scanners or
release gates. Workflow: `.github/workflows/ci-push.yml` (calls `ci.yml`).

**Tier 3 — PR CI (~8-10 min):** Triggers on `pull_request`. Full version
matrix (["1.92", "1.93"]), all integration tests, security scanners
(CodeQL, Trivy, Semgrep), standards compliance, and release gates. Workflow:
`.github/workflows/ci.yml`.

### Environment Setup

```bash
rustup show
```

### Docker-First Testing

All tests can run inside containers — Docker is the only host prerequisite.
The `dev-rust:1.93` image is built from
`../standard-tooling/docker/rust/`.

```bash
# Build the dev image (one-time, from standard-tooling)
cd ../standard-tooling && docker/build.sh

# Run tests in container
./scripts/dev/test.sh

# Run lint checks in container
./scripts/dev/lint.sh

# Run security audit in container
./scripts/dev/audit.sh
```

Environment overrides:

- `DOCKER_DEV_IMAGE` — override the container image (default: `dev-rust:1.93`)
- `DOCKER_TEST_CMD` — override the test command

### Validation

```bash
validate-local-rust
```

### Testing

```bash
cargo test
```

### Local MQ Container

The MQ development environment is owned by the
[mq-rest-admin-dev-environment](https://github.com/wphillipmoore/mq-rest-admin-dev-environment)
repository. Clone it as a sibling directory before running lifecycle
scripts:

```bash
# Prerequisite (one-time)
git clone https://github.com/wphillipmoore/mq-rest-admin-dev-environment.git ../mq-rest-admin-dev-environment

# Start the containerized MQ queue managers
./scripts/dev/mq_start.sh

# Seed deterministic test objects (DEV.* prefix)
./scripts/dev/mq_seed.sh

# Verify REST-based MQSC responses
./scripts/dev/mq_verify.sh

# Stop the queue managers
./scripts/dev/mq_stop.sh

# Reset to clean state (removes data volumes)
./scripts/dev/mq_reset.sh
```

Container details:

- Queue managers: `QM1` and `QM2`
- QM1 ports: `1454` (MQ listener), `9483` (mqweb console + REST API)
- QM2 ports: `1455` (MQ listener), `9484` (mqweb console + REST API)
- Admin credentials: `mqadmin` / `mqadmin`
- Read-only credentials: `mqreader` / `mqreader`
- QM1 REST base URL: `https://localhost:9483/ibmmq/rest/v2`
- QM2 REST base URL: `https://localhost:9484/ibmmq/rest/v2`
- Object prefix: `DEV.*`

Port assignments are explicit in each `scripts/dev/mq_*.sh` script via
`QM1_REST_PORT`, `QM2_REST_PORT`, `QM1_MQ_PORT`, and `QM2_MQ_PORT` exports.
Rust uses offset ports (9483/9484, 1454/1455) to avoid conflicts with other
language repos. See the
[port allocation table](https://github.com/wphillipmoore/mq-rest-admin-common)
in mq-rest-admin-common for the full cross-language map.

## Architecture

The project follows the Python reference implementation's structure, adapted
for Rust idioms:

- `src/lib.rs` — Public API re-exports
- `src/session.rs` — `MqRestSession` struct, `MqRestTransport` trait, HTTP
  transport via `reqwest::blocking`
- `src/commands.rs` — MQSC command methods (display_*, define_*, alter_*, etc.)
- `src/mapping.rs` — Request/response attribute name translation
- `src/mapping_data.rs` — Loads mapping-data.json at compile time
- `src/auth.rs` — BasicAuth, LtpaAuth, CertificateAuth
- `src/error.rs` — Error types via thiserror
- `src/ensure.rs` — Idempotent ensure_* methods
- `src/sync_ops.rs` — Synchronous polling wrappers
- `mapping-data.json` — Shared mapping data (same as Python)

## Key References

**Reference implementation**: `../mq-rest-admin-python` (Python version)

**External Documentation**:

- IBM MQ 9.4 administrative REST API
- MQSC command reference
