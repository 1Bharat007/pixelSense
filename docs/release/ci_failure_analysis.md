# CI Failure Forensics (Milestone 6.6)

## Phase 0: Root Cause Analysis

### Workflow 1: `frontend-ci.yml` & `rust-ci.yml`
- **Root Cause 1 (Triggers):** The workflows are configured to trigger on `push: branches: [ "main", "develop" ]`. The repository's primary branch is `master`. Consequently, standard pushes bypass CI entirely.
- **Root Cause 2 (Package Manager):** `frontend-ci.yml` explicitly attempts to install and run `pnpm install`, `pnpm lint`, and `pnpm build`. In Milestone 6.5, the repository was standardized onto `npm workspaces`, and the `pnpm` lockfiles were destroyed. This produces a fatal failure if the workflow is invoked.

### Workflow 2: `release.yml`
- **Root Cause 1 (Working Directory):** The `validate` job executes `cargo fmt -- --check`, `cargo clippy`, and `cargo test` directly in the root repository path (`./`). The Rust backend is located inside `apps/desktop/src-tauri`. Running `cargo` in a directory without a `Cargo.toml` immediately fails with: `error: could not find Cargo.toml in . or any parent directory`.
- **Root Cause 2 (Missing Dependencies):** `cargo tauri build` requires system dependencies on Linux (e.g., `libwebkit2gtk`, `build-essential`) which are entirely missing from the runner setup.
- **Root Cause 3 (Invalid Steps):** The pipeline attempts to run `cargo bench` and `mdbook build docs` which are not initialized in this repository.

### Summary Verdict
The entire GitHub Actions suite is fundamentally disconnected from the repository's current structure. The triggers are targeting ghost branches, the package manager is referencing a deleted ecosystem, and the Rust commands are executing in the wrong filesystem context.
