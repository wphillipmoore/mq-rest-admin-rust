# Developer setup

This guide covers everything needed to develop and test mq-rest-admin
locally.

## Prerequisites

| Tool | Version | Purpose |
| --- | --- | --- |
| Rust | 1.92+ (edition 2024) | Compiler and runtime |
| `rustup` | Latest | Rust toolchain management |
| `cargo-deny` | Latest | Dependency auditing |
| Docker | Latest | Local MQ containers (integration tests) |
| `markdownlint` | Latest | Docs validation |
| `git-cliff` | Latest | Changelog generation (releases only) |

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install cargo-deny:

```bash
cargo install cargo-deny
```

## Required repositories

mq-rest-admin depends on several sibling repositories:

| Repository | Purpose |
| --- | --- |
| [mq-rest-admin-rust](https://github.com/wphillipmoore/mq-rest-admin-rust) | This project |
| [standards-and-conventions](https://github.com/wphillipmoore/standards-and-conventions) | Canonical project standards (referenced by `AGENTS.md` and git hooks) |
| [standard-tooling](https://github.com/wphillipmoore/standard-tooling) | Development tools, Docker images, and git hooks |
| [mq-rest-admin-dev-environment](https://github.com/wphillipmoore/mq-rest-admin-dev-environment) | Dockerized MQ test infrastructure (local and CI) |

## Recommended directory layout

Clone all repositories as siblings:

```text
~/dev/
├── mq-rest-admin-rust/
├── standards-and-conventions/
├── standard-tooling/
└── mq-rest-admin-dev-environment/
```

```bash
cd ~/dev
git clone https://github.com/wphillipmoore/mq-rest-admin-rust.git
git clone https://github.com/wphillipmoore/standards-and-conventions.git
git clone https://github.com/wphillipmoore/standard-tooling.git
git clone https://github.com/wphillipmoore/mq-rest-admin-dev-environment.git
```

## Initial setup

```bash
cd mq-rest-admin-rust

# Build the project
cargo build

# Run tests
cargo test

# Enable standard tooling and git hooks
cd ../standard-tooling && uv sync
export PATH="../standard-tooling/.venv/bin:../standard-tooling/scripts/bin:$PATH"
cd ../mq-rest-admin-rust
git config core.hooksPath ../standard-tooling/scripts/lib/git-hooks
```

## Running validation

The containerized validation suite matches CI hard gates:

```bash
./scripts/dev/test.sh        # Tests in dev container
./scripts/dev/lint.sh        # Clippy + fmt in dev container
./scripts/dev/audit.sh       # cargo-deny in dev container
```

Or run locally:

```bash
cargo test                      # Unit tests
cargo clippy -- -D warnings     # Lint checks
cargo fmt --check               # Format verification
cargo deny check                # Dependency audit
```

## Running integration tests

Integration tests require running MQ containers. Start the containers,
seed test objects, then run the tests:

```bash
# Start both queue managers
./scripts/dev/mq_start.sh

# Seed deterministic test objects
./scripts/dev/mq_seed.sh

# Run integration tests
MQ_REST_ADMIN_RUN_INTEGRATION=1 cargo test --features integration
```

See [local MQ container](local-mq-container.md) for full container configuration,
credentials, gateway routing, and troubleshooting.

## CI pipeline overview

CI runs on every pull request and enforces the same gates as local
validation. The pipeline includes:

- **Unit tests** on the minimum supported Rust version (1.92)
- **Integration tests** against real MQ queue managers via the shared
  `wphillipmoore/mq-rest-admin-dev-environment/.github/actions/setup-mq` action
- **Standards compliance** (clippy, fmt, markdown lint, commit
  messages, repository profile)
- **Dependency audit** (`cargo-deny`)
- **Security scanning** (CodeQL, Trivy, Semgrep) on PR CI
- **Release gates** (version checks, changelog validation) for PRs
  targeting `main`
