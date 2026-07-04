# Build Pipeline Fix Documentation

## Pipeline Consolidation
The entire build pipeline infrastructure for PixelSense has been systematically hardened. 

### Final Architecture
1. **GitHub Actions Matrix:** The repository uses three distinct pipeline matrices (`frontend-ci`, `rust-ci`, `release`).
2. **Package Manager Parity:** `npm` is strictly enforced as the sole frontend package manager, reflecting the repository's migration to npm workspaces.
3. **Artifact Uploads:** `actions/upload-artifact@v3` is mapped explicitly to `apps/desktop/src-tauri/target/release/bundle/`, completely mirroring Tauri's internal bundle directory.

### Safety Guarantee
The hotfix prevents the CI engine from spontaneously halting due to YAML mapping errors, missing Linux UI libraries (webkit2gtk), or out-of-context Cargo manifest failures. Future release deployments (`git push origin v*`) are now mathematically robust.
