# RustGanizer

[![Crates.io Version](https://img.shields.io/crates/v/rustganizer)](https://crates.io/crates/rustganizer)
[![Crates.io License](https://img.shields.io/crates/l/rustganizer)](https://crates.io/crates/rustganizer)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-blue)](https://www.rust-lang.org)
[![CI Status](https://img.shields.io/github/actions/workflow/status/your-username/rustganizer/ci.yml)](https://github.com/your-username/rustganizer/actions)
[![Windows Support](https://img.shields.io/badge/platform-Windows-blue)](https://www.microsoft.com/windows)

![RustGanizer Screenshot](https://github.com/user-attachments/assets/b3dce163-d2cc-405b-a036-7e5de39e644c)

RustGanizer is a terminal-based file manager that automatically organizes your Windows Downloads and Desktop folders using Rust-powered efficiency. Built with Rust's Cursive library for a sleek CLI experience, it categorizes media and documents into proper folders with an intuitive interactive interface.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Architecture](#architecture)
- [Contributing](#contributing)
- [License](#license)

## Overview

RustGanizer scans your `Downloads` and `Desktop` directories, analyzes file types by extension, and moves them into appropriate category folders:

| Category | File Extensions |
|----------|---------------|
| Music | `.mp3`, `.wav`, `.flac`, `.aac`, `.ogg`, `.m4a` |
| Videos | `.mp4`, `.mkv`, `.avi`, `.mov`, `.wmv`, `.flv` |
| Pictures | `.jpg`, `.jpeg`, `.png`, `.gif`, `.bmp`, `.webp`, `.svg` |
| Documents | `.pdf`, `.doc`, `.docx`, `.txt`, `.xls`, `.xlsx` |
| Archives | `.zip`, `.rar`, `.7z` |
| Code | `.rs`, `.py`, `.js`, `.ts`, `.java`, `.c`, `.cpp` |

## Features

- **Automatic Organization**: Categorizes files from Downloads and Desktop into proper folders
- **Interactive CLI**: Terminal-based UI built with Cursive
- **Statistics Tracking**: Displays counts of files and folders moved
- **Multiple File Support**: Handles media, documents, archives, and code files
- **Logging**: Comprehensive logging with configurable levels
- **Error Handling**: Robust error handling with user-friendly messages

## Quick Start

```bash
# Clone and build
git clone https://github.com/your-username/rustganizer.git
cd rustganizer
cargo build --release

# Run
cargo run
```

Enter your Windows username when prompted, then select **Organize** from the menu.

## Installation

### Prerequisites

- **Rust** (1.75+): Install via [rustup](https://rustup.rs/)
- **Cargo**: Included with Rust
- **Windows OS**: Designed for Windows systems

### Build from Source

```bash
git clone https://github.com/your-username/rustganizer.git
cd rustganizer
cargo build --release
```

The compiled binary will be at `target/release/rustganizer.exe`.

## Usage

### Running RustGanizer

```bash
cargo run
```

Or after building:

```bash
./target/release/rustganizer.exe
```

### Interface

1. Enter your Windows username when prompted
2. Select **Organize** to begin organization
3. Review the summary showing files moved
4. Select **Close** to exit

### Troubleshooting

| Error | Solution |
|-------|----------|
| User not found | Enter a valid Windows username |
| Permission denied | Run as Administrator |

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RUSTGANIZER_LOG_LEVEL` | Logging verbosity | `info` |
| `RUSTGANIZER_CONFIG` | Path to config file | `./config.toml` |

### Config File

Create `config.toml` in the project root:

```toml
[logging]
level = "info"
file = "rustganizer.log"

[performance]
batch_size = 100
max_workers = 4

[ui]
language = "en"
```

### File Extensions

Default supported extensions:

- **Music**: `mp3`, `wav`, `flac`, `aac`, `ogg`, `m4a`
- **Videos**: `mp4`, `mkv`, `avi`, `mov`, `wmv`, `flv`
- **Pictures**: `jpg`, `jpeg`, `png`, `gif`, `bmp`, `webp`, `svg`
- **Documents**: `pdf`, `doc`, `docx`, `txt`, `xls`, `xlsx`, `ppt`, `pptx`
- **Archives**: `zip`, `rar`, `7z`, `tar`, `gz`
- **Code**: `rs`, `py`, `js`, `ts`, `java`, `c`, `cpp`, `go`, `rb`

## Architecture

### Module Structure

```
src/
├── main.rs           # Entry point
├── lib.rs           # Library exports
├── config.rs        # Configuration management
├── error.rs        # Error types
├── logging.rs      # Logging system
├── organizer/      # File organization
│   ├── mod.rs
│   ├── analyzer.rs     # File analysis
│   ├── async_analyzer.rs  # Async analysis
│   ├── mover.rs       # File moving
│   └── types.rs       # Data types
├── platform/       # Platform-specific
│   ├── mod.rs
│   └── user.rs     # Windows user utilities
└── ui/             # User interface
    ├── mod.rs
    └── views.rs    # UI views
```

### Key Components

- **Config**: Centralized configuration with validation and persistence
- **Organizer**: File analysis and categorization logic
- **Platform**: Windows-specific utilities
- **UI**: Interactive terminal interface using Cursive

## Contributing

### Development Setup

```bash
git clone https://github.com/your-username/rustganizer.git
cd rustganizer
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Run lints
cargo clippy -- -D warnings
```

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Coding Standards

- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings
- Add tests for new features
- Update documentation for any changes

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Built with Rust by FrankCasanova Technologies