# Getting Started

## Prerequisites
- Rust (latest stable)
- Node.js (v20+)
- pnpm

## Building Locally
1. Clone the repository.
2. Run `pnpm install` in the root.
3. Run `pnpm tauri dev` to start the development server.

## Running Tests
- Rust: `cargo test` inside `apps/desktop/src-tauri`
- Linting: `cargo clippy -- -D warnings` and `cargo fmt --check`
