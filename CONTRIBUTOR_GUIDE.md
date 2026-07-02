# PixelSense — Contributor Guide

Thank you for your interest in contributing to PixelSense. This guide covers everything you need to understand the architecture, coding standards, and workflow before submitting your first pull request.

---

## Before You Start

1. **Read the [Product Vision](PRODUCT_VISION.md)** to understand what PixelSense is trying to achieve.
2. **Read the [Architecture document](ARCHITECTURE.md)** to understand the overall design.
3. **Read the [Development Status](DEVELOPMENT_STATUS.md)** to know what is implemented, what is mocked, and what is planned.
4. **Set up your local environment** by following the [Getting Started Guide](docs/development/getting_started.md).

---

## Core Architecture Rules

These rules are **non-negotiable** in all contributions:

### 1. Single Responsibility
Every Rust module must have a clearly defined responsibility. Before writing code, ask: "What does this module NOT do?" Document that explicitly.

### 2. No Panic Policy
Production code paths must never `unwrap()` or `expect()` on `Option` or `Result`. Use proper error propagation. Tests may use these for clarity.

### 3. Dependency Inversion
Orchestrators (e.g., `AdaptiveBrightnessService`) must depend on **traits**, not concrete types. Concrete implementations live in provider modules or factories.

### 4. No Duplicated Orchestration
If a module already owns a responsibility, do not replicate it elsewhere. For example, `VisualComfortEngine` calculates recommendations. `AdaptiveBrightnessService` executes them. Neither does the other's job.

### 5. Privacy by Design
- No image data may ever be saved to disk, even temporarily.
- No network requests from any module.
- Luminance and ambient analysis must be performed entirely in memory.

### 6. Graceful Degradation
If a sensor or hardware API is unavailable, the system must continue with reduced capability. It must never crash, freeze, or show an error the user cannot recover from.

---

## Rust Standards

```rust
// ✅ Correct: Use Result for all fallible operations
fn get_brightness(&self) -> Result<u8, BrightnessError> { ... }

// ❌ Incorrect: Never use unwrap in production paths
let brightness = manager.get_brightness().unwrap();

// ✅ Correct: Propagate errors
let brightness = manager.get_brightness()?;
```

- Run `cargo fmt` before every commit.
- Run `cargo clippy -- -D warnings` and resolve all warnings.
- Run `cargo test` to verify all tests pass.

---

## TypeScript / React Standards

- Components must be purely presentational. No business logic inside React components.
- All Tauri `invoke()` calls belong in service files (`services/`) or hooks (`hooks/`), never inline in components.
- Use TypeScript strict mode. No `any` types without documented justification.
- ARIA labels are required on all interactive elements.

---

## Pull Request Workflow

1. **Open an issue first** for any non-trivial change. Describe the problem and your proposed approach.
2. **For architectural changes**, open a Request For Comments (RFC) issue and wait for discussion before writing code.
3. **Create a branch** from `main` with a clear name: `feat/ambient-sensor-windows`, `fix/rate-limiter-edge-case`, `docs/update-faq`.
4. **Keep PRs focused.** One PR per concern. Do not mix feature additions with refactors.
5. **Write or update tests** for any changed behavior.
6. **Update documentation** if your change affects architecture, configuration, or the feature list.
7. **Fill in the PR template** completely before requesting review.

---

## Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(ambient): add Windows native sensor provider
fix(rate-limiter): correct timestamp comparison logic
docs(architecture): update visual comfort engine diagram
refactor(transition): extract step calculation into pure function
test(comfort): add missing profile matching edge case
```

---

## What We Are Actively Looking For

- Windows DDC/CI brightness API improvements
- macOS platform provider implementations
- Linux platform provider implementations
- Test coverage improvements
- Documentation improvements
- Accessibility improvements in the React frontend

---

## What We Are Not Looking For (At This Time)

- Cloud or networking features of any kind
- AI or machine learning integrations
- Color calibration features
- Blue light filter features
- Any change that stores image or pixel data to disk

---

## Questions?

Open a Discussion on GitHub before opening a PR. We are happy to discuss the right approach before you invest time writing code.
