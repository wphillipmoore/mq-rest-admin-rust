# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Auto-memory policy

**Do NOT use MEMORY.md.** Claude Code's auto-memory feature stores behavioral
rules outside of version control, making them invisible to code review,
inconsistent across repos, and unreliable across sessions. All behavioral rules,
conventions, and workflow instructions belong in managed, version-controlled
documentation (CLAUDE.md, AGENTS.md, skills, or docs/).

If you identify a pattern, convention, or rule worth preserving:

1. **Stop.** Do not write to MEMORY.md.
2. **Discuss with the user** what you want to capture and why.
3. **Together, decide** the correct managed location (CLAUDE.md, a skill file,
   standards docs, or a new issue to track the gap).

This policy exists because MEMORY.md is per-directory and per-machine — it
creates divergent agent behavior across the multi-repo environment this project
operates in. Consistency requires all guidance to live in shared, reviewable
documentation.

## Shell command policy

**Do NOT use heredocs** (`<<EOF` / `<<'EOF'`) for multi-line arguments to CLI
tools such as `gh`, `git commit`, or `curl`. Heredocs routinely fail due to
shell escaping issues with apostrophes, backticks, and special characters.
Always write multi-line content to a temporary file and pass it via `--body-file`
or `--file` instead.

## Documentation Strategy

This repository uses two complementary approaches for AI agent guidance:

- **AGENTS.md**: Generic AI agent instructions using include directives to force documentation indexing. Contains canonical standards references, shared skills loading, and user override support.
- **CLAUDE.md** (this file): Claude Code-specific guidance with prescriptive commands, architecture details, and development workflows optimized for `/init`.

<!-- include: docs/standards-and-conventions.md -->
<!-- include: docs/repository-standards.md -->

## Project Overview

`mq-rest-admin` is a Rust wrapper for the IBM MQ administrative REST
API. The project provides typed Rust functions for every MQSC command
exposed by the `runCommandJSON` REST endpoint, with automatic attribute name
translation between Rust idioms and native MQSC parameter names.

**Project name**: mqrest-rust

**Status**: Pre-alpha (initial setup)

**Canonical Standards**: This repository follows standards at <https://github.com/wphillipmoore/standards-and-conventions> (local path: `../standards-and-conventions` if available)

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

## Branching and PR Workflow

- **Protected branches**: `main`, `develop` — no direct commits (enforced by pre-commit hook)
- **Branch naming**: `feature/*`, `bugfix/*`, `hotfix/*`, `chore/*`, or `release/*` only
- **Feature/bugfix PRs** target `develop` with squash merge
- **Release PRs** target `main` with regular merge
- **Pre-flight**: Always check branch with `git status -sb` before modifying files. If on `develop`, create a `feature/*` branch first.

## Commit and PR Scripts

**NEVER use raw `git commit`** — always use `st-commit`.
**NEVER use raw `gh pr create`** — always use `st-submit-pr`.

### Committing

```bash
st-commit --type feat --scope session --message "add new feature" --agent claude
st-commit --type fix --message "correct bug" --agent claude
st-commit --type docs --message "update README" --body "Expanded usage section" --agent claude
```

- `--type` (required): `feat|fix|docs|style|refactor|test|chore|ci|build`
- `--message` (required): commit description
- `--agent` (required): `claude` or `codex` — resolves the correct `Co-Authored-By` identity
- `--scope` (optional): conventional commit scope
- `--body` (optional): detailed commit body

### Submitting PRs

```bash
st-submit-pr --issue 42 --summary "Add new feature"
st-submit-pr --issue 42 --linkage Ref --summary "Update docs"
st-submit-pr --issue 42 --summary "Fix bug" --notes "Tested on macOS and Linux"
```

- `--issue` (required): GitHub issue number (just the number)
- `--summary` (required): one-line PR summary
- `--linkage` (optional, default: `Fixes`): `Fixes|Closes|Resolves|Ref`
- `--title` (optional): PR title (default: most recent commit subject)
- `--notes` (optional): additional notes
- `--dry-run` (optional): print generated PR without executing

## Key References

**Canonical Standards**: <https://github.com/wphillipmoore/standards-and-conventions>

- Local path (preferred): `../standards-and-conventions`
- Load all skills from: `<standards-repo-path>/skills/**/SKILL.md`

**Reference implementation**: `../mq-rest-admin-python` (Python version)

**External Documentation**:

- IBM MQ 9.4 administrative REST API
- MQSC command reference

**User Overrides**: `~/AGENTS.md` (optional, applied if present and readable)
