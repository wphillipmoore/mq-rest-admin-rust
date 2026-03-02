# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## [1.2.2] - 2026-03-02

### Bug fixes

- move crates.io publish before SBOM generation and bump to 1.2.2 (#50)

## [1.2.1] - 2026-03-02

### Bug fixes

- extract default credential to constant to satisfy CodeQL (#20)
- LTPA cookie extraction uses prefix matching for suffixed cookie names (#31)
- create dist directory before SBOM generation (#47)

### CI

- add workflow to auto-add issues to GitHub Project
- add concurrency group to ci-push workflow (#44)

### Documentation

- add cross-repo documentation links to docs site (#33)

### Features

- initial template repository setup for Rust
- port Python examples to idiomatic Rust (#10)
- refactor examples to dual-mode with importable functions, tests, and coverage (#22)
- auto-generate all MQSC command methods from mapping-data.json (#28)
- add SyncConfig construction validation (#43)

### Styling

- rename abbreviated variables to descriptive PBP names (#12)

### Testing

- add 570 tests achieving 100% line coverage (#14)
- add 35 integration tests against live MQ queue managers (#18)
- enforce 100% region coverage (#24)
- add session state populated after command integration test (#40)

### Revert

- remove premature add-to-project workflow
