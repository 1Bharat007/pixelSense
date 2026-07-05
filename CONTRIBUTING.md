# Contributing to PixelSense

We love your input! We want to make contributing to PixelSense as easy and transparent as possible, whether it's:
- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features

## Development Setup
PixelSense requires Node.js 20+ and Rust 1.75+.
```bash
# Clone the repository
git clone https://github.com/pixelSense/pixelSense.git

# Install dependencies
npm install

# Run the Tauri development server
npm run tauri dev
```

## Architecture
PixelSense uses a React + Tailwind CSS frontend and a Rust Tauri backend.
All backend state is managed via Tokio Mutexes and exposed to React via Tauri IPC.

## Pull Request Process
1. Ensure your code strictly follows the "Implementation First Policy" and does not introduce speculative features.
2. Update the README.md with details of changes to the interface.
3. You may merge the Pull Request in once you have the sign-off of at least one other developer.
