# Repository Standards

## Table of Contents

- [Pre-flight checklist](#pre-flight-checklist)
- [Local validation](#local-validation)
- [Tooling requirement](#tooling-requirement)
- [Merge strategy override](#merge-strategy-override)
- [Approved domain abbreviations](#approved-domain-abbreviations)
- [Accepted naming deviations](#accepted-naming-deviations)

## Pre-flight checklist

- Before modifying any files, check the current branch with `git status -sb`.
- If on `develop`, create a short-lived `feature/*` branch or ask for explicit approval to proceed on `develop`.
- If approval is granted to work on `develop`, call it out in the response and proceed only for that user-approved scope.
- Enable repository git hooks before committing: `git config core.hooksPath ../standard-tooling/scripts/lib/git-hooks`.

## Local validation

Canonical local validation command:

```bash
validate-local-rust
```

## Tooling requirement

Required for daily workflow:

- Rust 1.92+ (via rust-tooling.toml), cargo-deny, cargo-llvm-cov
- `markdownlint` (required for docs validation and PR pre-submission)

Required for integration testing:

- Docker (for local MQ container environment)

## Merge strategy override

- Feature, bugfix, and chore PRs targeting `develop` use squash merges (`--squash`).
- Release PRs targeting `main` use regular merges (`--merge`) to preserve shared
  ancestry between `main` and `develop`.
- Auto-merge commands:
  - Feature PRs: `gh pr merge --auto --squash --delete-branch`
  - Release PRs: `gh pr merge --auto --merge --delete-branch`

## Commit and PR scripts

AI agents **must** use the `st-commit` and `st-submit-pr` scripts for commits
and PR submission. Do not construct commit messages or PR bodies manually.

### Committing

```bash
st-commit \
  --type TYPE --message MESSAGE --agent AGENT \
  [--scope SCOPE] [--body BODY]
```

- `--type` (required): one of
  `feat|fix|docs|style|refactor|test|chore|ci|build`
- `--message` (required): commit description
- `--agent` (required): `claude` or `codex`
- `--scope` (optional): conventional commit scope
- `--body` (optional): detailed commit body

The script resolves the correct `Co-Authored-By` identity from
`standard-tooling.toml` and the git hooks validate the result.

### Submitting PRs

```bash
st-submit-pr \
  --issue NUMBER --summary TEXT \
  [--linkage KEYWORD] [--title TEXT] \
  [--notes TEXT] [--dry-run]
```

- `--issue` (required): GitHub issue number (just the number)
- `--summary` (required): one-line PR summary
- `--linkage` (optional, default: `Fixes`):
  `Fixes|Closes|Resolves|Ref`
- `--title` (optional): PR title (default: most recent commit
  subject)
- `--notes` (optional): additional notes
- `--dry-run` (optional): print generated PR without executing

The script detects the target branch and merge strategy
automatically.

## Approved domain abbreviations

Domain-specific abbreviations that are well-established in the IBM MQ ecosystem
and may be used in identifiers without expansion:

- `qmgr` — queue manager (established MQSC domain term)

## Accepted naming deviations

None yet.
