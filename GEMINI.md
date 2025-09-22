# Gemini Context: IdleDash Project

This file provides context for the Gemini AI assistant to pick up work on the `idledash` project.

## Project Overview

`idledash` is a terminal user interface (TUI) dashboard written in Rust. It provides a multi-pane view of system information, time, disk usage, and the status of version control repositories.

## Current Status

The initial implementation is complete. The following features are implemented:
- Time pane (local, UTC, west coast)
- System resource pane (CPU, memory)
- Disk usage pane
- Version control pane (Git only)
- Configuration via `config.toml` for VCS paths

Mercurial support is planned but not yet implemented.

## Project Structure

- `src/main.rs`: Main application entry point, TUI setup, and main loop.
- `src/app.rs`: `AppState` struct, holding the application's state.
- `src/ui.rs`: Main UI layout rendering.
- `src/config.rs`: Configuration loading (`config.toml`).
- `src/time.rs`: Time pane rendering logic.
- `src/system.rs`: System resource pane rendering logic.
- `src/disks.rs`: Disk usage pane rendering logic.
- `src/vcs.rs`: Version control scanning and rendering logic.
- `Cargo.toml`: Project dependencies.
- `config.toml`: Configuration file.
