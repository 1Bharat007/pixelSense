# Release Engineering & Developer Experience

## CI/CD Readiness
- The 5 monolithic GitHub Action workflows (CI, Build, Release, Docs, Benchmarks) engineered in Milestone 6.6 are stable.
- The `v0.1.0-alpha` tag is prepared.

## Developer Onboarding
- **Frictionless Setup:** A new developer only requires Node.js 20+ and Rust 1.75+. Cloning and running `npm run tauri dev` instantly boots the React+Tauri environment.
- **Architecture Understanding:** The exact boundary between Tokio Mutex state and React Context/Zustand is clearly documented in `CONTRIBUTING.md`.
