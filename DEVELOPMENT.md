# Development Roadmap: Universal Package Lister (Rust)

This document outlines the technical strategy for building a Rust-based CLI tool to manage global binaries from developer ecosystems (NPM, Go, Cargo, Pip, etc.).

## Phase 1: Environment Discovery
The tool must first detect which developer environments are active on the host machine.

*   **PATH Scanning:** Use `which` or `std::process::Command` to check for the existence of `npm`, `cargo`, `go`, and `pip`.
*   **Active Managers:** Only load logic for the tools actually found in the user's environment to avoid errors.

## Phase 2: Inventory & Parsing Logic
Each tool has a unique way of listing globally installed packages. We need to parse their specific outputs:

*   **Cargo:** Parse output from `cargo install --list`.
*   **NPM:** Parse JSON or text from `npm list -g --depth=0`.
*   **Go:** Scan the `$GOPATH/bin` or `~/go/bin` directory for compiled binaries.
*   **Python/Pip:** Parse `pip list` or `pipx list`.

## Phase 3: The Unified CLI (Rust)
A robust architecture using Rust's trait system to handle different ecosystems consistently.

*   **The `DevManager` Trait:**
    ```rust
    trait DevManager {
        fn name(&self) -> String;
        fn list_globals(&self) -> Vec<GlobalBin>;
        fn uninstall(&self, package: &str) -> Result<(), Error>;
    }
    ```
*   **Deduplication:** Identify if the same tool (e.g., `ripgrep` or `prettier`) is installed across multiple managers, which wastes space and causes PATH conflicts.

## Phase 4: Snapshot & Cleanup
Tools to keep the development machine lean and organized.

*   **Snapshot System:** Save a list of all current global binaries to a local file (e.g., `~/.config/upl/snapshot.json`).
*   **Environmental Revert:** A command to compare the current state with a snapshot and suggest uninstalls for anything new.
*   **Mass Cleanup:** A guided wizard to help users uninstall global tools they haven't used in a while.

## Phase 5: Unified Update Engine
Implement the logic to trigger updates across all detected managers.

*   **Sequential Execution:** Run `npm update -g`, `cargo install-update -a`, etc.
*   **Progress Tracking:** Use a crate like `indicatif` to show a progress bar during long-running updates.

## Recommended Rust Crates
*   `clap`: For the CLI interface.
*   `serde` & `serde_json`: For saving snapshots.
*   `which`: To easily locate binaries in the system PATH.
*   `colored`: For color-coded terminal output.
*   `thiserror` & `anyhow`: For clean error handling.
