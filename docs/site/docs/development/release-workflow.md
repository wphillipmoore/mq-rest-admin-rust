# Release workflow

This document describes how mq-rest-admin versions are managed and published
to crates.io.

## Version management

The version is stored in `Cargo.toml` under `[package].version`. It follows
semantic versioning (`MAJOR.MINOR.PATCH`).

After each release, the publish workflow automatically opens a PR to
bump the patch version on `develop`. This default can be overridden at
any time by changing the version to a minor or major bump instead.

## Release flow

1. **Develop** — All feature work merges into `develop`. Ensure the
   version in `Cargo.toml` is set to the desired release version.
2. **Prepare release** — Create a `release/X.Y.Z` branch from `develop`
   and generate the changelog:

   ```bash
   git checkout -b release/X.Y.Z develop
   git-cliff --tag develop-vX.Y.Z -o CHANGELOG.md
   st-commit --type chore --scope release --message "update changelog for vX.Y.Z" --agent claude
   ```

3. **Merge to main** — Open a PR from `release/X.Y.Z` to `main` and
   merge with a regular merge commit (not squash). This preserves shared
   ancestry between `main` and `develop`, avoiding history divergence.
4. **Automatic publish** — The `publish.yml` workflow fires on push to
   `main` and:
   - Extracts the version from `Cargo.toml`
   - Skips if the version is already on crates.io (idempotent)
   - Runs `cargo publish` using `CARGO_REGISTRY_TOKEN`
   - Creates an annotated git tag (`vX.Y.Z`)
   - Creates a `develop-vX.Y.Z` lightweight tag on `develop` for
     git-cliff boundary tracking
   - Creates a GitHub Release with install instructions
   - Opens a PR against `develop` to bump the patch version (e.g.
     `1.2.0` → `1.2.1`)

## Automatic version bump

After each successful publish, the workflow creates a PR to increment
the patch version on `develop`. This keeps the working version ahead of
the last release and ready for the next patch.

If the next release should be a minor or major bump instead, simply
change the version in `Cargo.toml` at any point during the
development cycle — the automated PR is just a default starting point.

The bump PR is skipped if `develop` already has the expected next
version (e.g. if someone bumped it manually first).

## Changelog

The project changelog is maintained in `CHANGELOG.md` using
[git-cliff](https://git-cliff.org/), configured via `cliff.toml` at
the repository root.

git-cliff is a local developer tool (not required in CI). Install it
with Homebrew:

```bash
brew install git-cliff
```

### How it works

git-cliff uses `develop-vX.Y.Z` lightweight tags as version boundary
markers. These tags point to commits on `develop` and are created
automatically by the publish workflow after each release. They allow
git-cliff to determine which commits belong to each version when run
on `develop` or a release branch.

To regenerate the changelog from scratch:

```bash
git-cliff -o CHANGELOG.md
```

To generate the changelog with a new version heading (used during the
release flow):

```bash
git-cliff --tag develop-vX.Y.Z -o CHANGELOG.md
```

### CI validation

The `release-gates` CI job validates that `CHANGELOG.md` contains an
entry matching the version in `Cargo.toml` for PRs targeting
`main`. This ensures the changelog is always updated before a release.

## CI version gates

Pull requests trigger additional version checks:

- **PRs targeting main**: Version must not already exist on crates.io,
  must be greater than the latest published version, and `CHANGELOG.md`
  must contain an entry for the version.
- **PRs targeting develop**: Version must differ from the version on
  `main` (prevents accidental no-op releases).

## crates.io token setup (one-time)

Before the first release, the repository owner must configure the
crates.io API token:

1. Log in at <https://crates.io/> with the GitHub account.
2. Go to Account Settings > API Tokens.
3. Create a scoped token: name `mq-rest-admin-publish`, scopes
   `publish-new` + `publish-update` for crate `mq-rest-admin`.
4. Add the token as repo secret `CARGO_REGISTRY_TOKEN` at
   Settings > Secrets > Actions.

The crate name `mq-rest-admin` will be claimed on first publish.

## Troubleshooting

### Version already exists on crates.io

The publish workflow skips publishing if the version already exists. To
release a new version, bump the version in `Cargo.toml` and go
through the release flow again.

### Tag already exists

The publish workflow skips tag creation if the tag already exists. This
is expected when re-running a failed workflow.

### Publish fails

Check the workflow logs for authentication errors. Ensure the
`CARGO_REGISTRY_TOKEN` secret is configured correctly. The workflow
only triggers on push to `main`.
