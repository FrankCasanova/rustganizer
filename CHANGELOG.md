# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Async file processing with `tokio` and `futures`
- Parallel processing support with `rayon`
- Comprehensive configuration system with TOML persistence
- Structured logging with `tracing`
- Custom error types with `thiserror`
- CI/CD pipeline with GitHub Actions

### Changed

- Improved modular project structure
- Enhanced file type categorization (archives, code files)
- Better error handling and recovery
- Performance optimization for large directories

### Fixed

- Username lookup on Windows systems
- File permission handling

## [0.5.0] - 2025-01-15

### Added

- Interactive CLI interface with Cursive
- File statistics tracking
- Multi-category support (Music, Videos, Pictures, Documents)
- Logging system

### Changed

- Migrated to Rust 2021 edition
- Improved file categorization logic

### Fixed

- Fixed Downloads/Desktop path resolution
- Fixed file move conflicts

## [0.4.0] - 2024-12-01

### Added

- Basic file organization
- Windows username detection

### Changed

- Initial project structure

---

## Version History

| Version | Date | Description |
|---------|------|-------------|
| 0.5.0 | 2025-01-15 | Major release with async support |
| 0.4.0 | 2024-12-01 | Initial feature release |

## Migration Guides

### 0.4.0 → 0.5.0

Breaking changes:
- Config file format changed to TOML
- API refactored for async support

New dependencies:
- `tokio` for async runtime
- `rayon` for parallel processing
- `tracing` for logging
- `thiserror` for error handling

Code updates required:
- Update imports to use new module paths
- Update configuration loading code
- Update async functions to use `await`