# Contributing to PixelSense

Thank you for considering contributing to PixelSense! This document covers the essentials to get you started quickly.

For the complete developer handbook — including architecture rules, coding standards, and detailed PR workflow — see [CONTRIBUTOR_GUIDE.md](CONTRIBUTOR_GUIDE.md).

---

## Quick Setup

```bash
# Prerequisites: Node.js 20+, Rust 1.75+, Visual Studio Build Tools 2022

git clone https://github.com/1Bharat007/pixelSense.git
cd pixelSense
npm install
npm run tauri dev
```

---

## Ways to Contribute

### 🐛 Found a Bug?

1. Search [existing issues](https://github.com/1Bharat007/pixelSense/issues) to avoid duplicates
2. Open a [Bug Report](https://github.com/1Bharat007/pixelSense/issues/new?template=bug_report.md) with reproduction steps
3. Include your Windows version, display configuration, and any relevant logs

### 💡 Have an Idea?

1. Open a [Feature Request](https://github.com/1Bharat007/pixelSense/issues/new?template=feature_request.md)
2. For architectural changes, start a Discussion first — we'll help shape the approach before you invest time coding

### 📝 Improving Documentation?

Documentation PRs are always welcome. No issue required — just submit the PR.

### 🔧 Submitting Code?

1. **Open an issue first** for non-trivial changes
2. **Fork and branch** from `master` with a descriptive name: `feat/ambient-sensor-windows`, `fix/transition-edge-case`
3. **Follow the coding standards** outlined in [CONTRIBUTOR_GUIDE.md](CONTRIBUTOR_GUIDE.md)
4. **Test your changes** locally before submitting
5. **Fill in the PR template** completely

---

## Coding Standards (Summary)

### Rust
- All fallible operations return `Result<T, E>` — no `unwrap()` or `expect()` in production code
- Run `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo test` before committing
- Follow [Conventional Commits](https://www.conventionalcommits.org/) for commit messages

### TypeScript / React
- No business logic in components — use `services/` and `hooks/`
- TypeScript strict mode — no untyped `any` without documented justification
- ARIA labels on all interactive elements

---

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(ambient): add Windows native sensor provider
fix(transition): correct rate limiter edge case
docs(readme): update quick start instructions
test(comfort): add profile matching edge cases
```

---

## What We're Looking For

- Windows DDC/CI brightness improvements
- macOS and Linux platform providers
- Test coverage improvements
- Accessibility improvements
- Documentation and translations

## What We're Not Looking For (Currently)

- Cloud or networking features
- AI/ML integrations
- Color calibration or blue light filtering
- Any change that persists image or pixel data to disk

---

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold these standards.

---

## Questions?

Open a [Discussion](https://github.com/1Bharat007/pixelSense/discussions) before investing time in a PR. We're happy to help shape the right approach.
