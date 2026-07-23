# Contributing to PixelSense

Thanks for thinking about contributing! Whether you're fixing a typo, adding a test, or building a new feature — all help is welcome.

This guide will walk you through getting set up and submitting your first pull request.

---

## Setting Up Your Dev Environment

You'll need three things installed before you can build PixelSense:

### 1. Rust (v1.84 or newer)

Rust is the language the backend is written in. Install it from [rustup.rs](https://rustup.rs/). After installing, open a new terminal and run:

```bash
rustc --version
# Should print something like: rustc 1.84.0 (or higher)
```

### 2. Node.js (v18 or newer)

Node.js runs the frontend build tools. Download it from [nodejs.org](https://nodejs.org/). After installing:

```bash
node --version
# Should print something like: v18.x.x (or higher)
```

### 3. Visual Studio Build Tools 2022

This is needed to compile native Windows code. Download the [Build Tools installer](https://visualstudio.microsoft.com/visual-cpp-build-tools/). During setup, check **"Desktop development with C++"** and install it.

### Clone and Run

```bash
git clone https://github.com/1Bharat007/pixelSense.git
cd pixelSense
npm install
npm run tauri dev
```

The first build takes a few minutes (Rust compiles everything from scratch). After that, rebuilds are fast.

---

## Running the Tests

Before submitting a PR, make sure all tests pass:

```bash
cd apps/desktop/src-tauri
cargo test -p app --lib
```

You should see `85 passed; 0 failed`. Two hardware stress tests are `ignored` by default — that's expected and fine.

Also check for compiler warnings:

```bash
cargo clippy -- -D warnings
```

And make sure the code is formatted correctly:

```bash
cargo fmt -- --check
```

---

## Ways to Contribute

### Found a bug?

1. Check [existing issues](https://github.com/1Bharat007/pixelSense/issues) first to see if it's already reported
2. If not, open a new issue with:
   - What you expected to happen
   - What actually happened
   - Steps to reproduce it
   - Your Windows version and display setup (single monitor, multi-monitor, laptop, etc.)

### Want to fix or improve something?

1. Look at the [issues list](https://github.com/1Bharat007/pixelSense/issues) — anything labeled `good first issue` is a small, self-contained task meant for newcomers
2. Comment on the issue to let others know you're working on it
3. Fork the repo, make your changes on a branch, and open a PR

### Want to improve documentation?

Documentation PRs don't need an issue first. Just open the PR.

### Have a feature idea?

Open an issue describing what you'd like and why it would help. For bigger ideas (architectural changes, new subsystems), let's discuss it in the issue before you write code — that way we can agree on the right approach first.

---

## Submitting a Pull Request

1. **Fork** the repo and create a branch from `master`:
   ```bash
   git checkout -b fix/your-descriptive-branch-name
   ```
   Use prefixes like `feat/`, `fix/`, `docs/`, `test/` so the branch name explains what it does.

2. **Make your changes.** Keep the PR focused on one thing — don't mix a bug fix with a refactor.

3. **Test your changes:**
   ```bash
   cargo test -p app --lib
   cargo clippy -- -D warnings
   cargo fmt -- --check
   ```

4. **Write a clear PR description.** Explain what you changed and why. If it's a bug fix, describe what was broken.

5. **Submit the PR.** I'll review it and either merge it or leave feedback.

---

## Code Style

### Rust

- **No `unwrap()` or `expect()` in production code.** Always use `?` or proper error handling. Tests can use `unwrap()` for clarity.
- **Run `cargo fmt` before committing.** This auto-formats everything.
- **Fix all clippy warnings.** We treat warnings as errors.
- Each module should have a single, clear responsibility. If you're not sure where something belongs, ask in the issue.

### TypeScript / React (Frontend)

- Keep components presentational — business logic goes in `services/` or `hooks/`, not inside components.
- Use TypeScript types properly — no `any` without a comment explaining why.
- All interactive elements need ARIA labels for accessibility.

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(ambient): add Windows native sensor provider
fix(transition): correct rate limiter edge case
docs(readme): update installation instructions
test(comfort): add profile matching edge cases
```

The format is: `type(scope): short description`. Common types: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`.

---

## What We're Looking For

- Bug fixes and test coverage improvements
- Windows DDC/CI and WMI brightness handling improvements
- Accessibility improvements in the frontend
- Documentation improvements
- macOS or Linux platform provider implementations (these are currently empty stubs)

## What We're Not Looking For Right Now

- Cloud, networking, or account features
- AI / machine learning integrations
- Color calibration or blue light filtering
- Anything that saves image or pixel data to disk (privacy is a core design rule)

---

## Code of Conduct

Be kind. Be respectful. We follow the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).

---

## Questions?

Open an issue or start a Discussion on GitHub. I'm happy to help you find a good place to start.
