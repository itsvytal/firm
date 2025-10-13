# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-10-13

### Added

- Tree-sitter grammar repo as a root-level submodule.
- A new README which unifies concepts across core, language and CLI.
- A shared workspace example.
- Pretty output support.
- Inline documentation for most features.
- Github CI pipeline for building and releasing binaries.

### Fixed

- Cargo configs for crates in the workspace.
- Broken test referencing the workspace example.

### Changed

- Migrated separate crate repo to a single Rust workspace.
- CLI add action now also outputs the generated entity.
- Refactoring and documentation cleanup.
