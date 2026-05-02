# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**Standards reference**: <https://github.com/wphillipmoore/standards-and-conventions>
— active standards documentation lives in the standard-tooling repository under `docs/`.
Repository profile: `standard-tooling.toml`.

## Auto-memory policy

**Do NOT use MEMORY.md.** Never write to MEMORY.md or any file under the
memory directory. All behavioral rules, conventions, and workflow instructions
belong in managed, version-controlled documentation (CLAUDE.md, AGENTS.md,
skills, or docs/). If you want to persist something, tell the human what you
would save and let them decide where it belongs.

## Parallel AI agent development

This repository supports running multiple Claude Code agents in parallel via
git worktrees. The convention keeps parallel agents' working trees isolated
while preserving shared project memory (which Claude Code derives from the
session's starting CWD).

**Canonical spec:**
[`standard-tooling/docs/specs/worktree-convention.md`](https://github.com/wphillipmoore/standard-tooling/blob/develop/docs/specs/worktree-convention.md)
— full rationale, trust model, failure modes, and memory-path implications.
The canonical text lives in `standard-tooling`; this section is the local
on-ramp.

### Structure

```text
~/dev/github/mq-rest-admin-rust/          ← sessions ALWAYS start here
  .git/
  CLAUDE.md, src/, tests/, …              ← main worktree (usually `develop`)
  .worktrees/                             ← container for parallel worktrees
    issue-71-adopt-worktree-convention/   ← worktree on feature/71-...
    …
```

### Rules

1. **Sessions always start at the project root.**
   `cd ~/dev/github/mq-rest-admin-rust && claude` — never from inside
   `.worktrees/<name>/`. This keeps the memory-path slug stable and shared.
2. **Each parallel agent is assigned exactly one worktree.** The session
   prompt names the worktree (see Agent prompt contract below).
   - For Read / Edit / Write tools: use the worktree's absolute path.
   - For Bash commands that touch files: `cd` into the worktree first,
     or use absolute paths.
3. **The main worktree is read-only.** All edits flow through a worktree
   on a feature branch — the logical endpoint of the standing
   "no direct commits to `develop`" policy.
4. **One worktree per issue.** Don't stack in-flight issues. When a
   branch lands, remove the worktree before starting the next.
5. **Naming: `issue-<N>-<short-slug>`.** `<N>` is the GitHub issue
   number; `<short-slug>` is 2–4 kebab-case tokens.

### Agent prompt contract

When launching a parallel-agent session, use this template (fill in the
placeholders):

```text
You are working on issue #<N>: <issue title>.

Your worktree is: /Users/pmoore/dev/github/mq-rest-admin-rust/.worktrees/issue-<N>-<slug>/
Your branch is:   feature/<N>-<slug>

Rules for this session:
- Do all git operations from inside your worktree:
    cd <absolute-worktree-path> && git <command>
- For Read / Edit / Write tools, use the absolute worktree path.
- For Bash commands that touch files, cd into the worktree first
  or use absolute paths.
- Do not edit files at the project root. The main worktree is
  read-only — all changes flow through your worktree on your
  feature branch.
```

All fields are required.

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

### Two-Tier CI Model

Testing is split across two tiers with increasing scope and cost:

**Tier 1 — Local pre-commit (seconds):** Fast smoke tests in a single
container. Enforced via the `.githooks` pre-commit gate on every commit.
No MQ, no matrix.

```bash
./scripts/dev/test.sh        # Tests in dev-rust:1.93
./scripts/dev/lint.sh        # Lint checks in dev-rust:1.93
./scripts/dev/typecheck.sh   # Type checking in dev-rust:1.93
./scripts/dev/audit.sh       # Security audit in dev-rust:1.93
```

**Tier 2 — PR CI (~8-10 min):** Triggers on `pull_request`. Full version
matrix (["1.92", "1.93"]), all integration tests, security scanners
(CodeQL, Trivy, Semgrep), standards compliance, and release gates. Workflow:
`.github/workflows/ci.yml`.

Push-CI was retired once `st-validate-local` reached parity with PR-CI.
See wphillipmoore/standard-actions#176 for the parity audit and rationale.

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
