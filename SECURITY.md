# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | ✅ Active support  |
| < 0.1   | ❌ Not supported   |

## Why Security Matters for PixelSense

PixelSense interacts with monitor hardware through DDC/CI commands and reads ambient sensor data. A security vulnerability could potentially:

- Manipulate physical display hardware
- Access local configuration files
- Exploit the Tauri IPC boundary

We take every report seriously.

## Reporting a Vulnerability

**Do not open a public issue for security vulnerabilities.**

Instead, please report vulnerabilities privately:

1. **GitHub Security Advisories** (preferred): Use the [Security Advisories](https://github.com/1Bharat007/pixelSense/security/advisories/new) feature to submit a private report
2. **Email**: Contact the maintainers at the email listed in the repository

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Affected versions
- Potential impact
- Suggested fix (if any)

### Response Timeline

| Action | Timeframe |
|:-------|:----------|
| Acknowledgment | Within 48 hours |
| Initial assessment | Within 5 business days |
| Fix development | Based on severity |
| Public disclosure | After fix is released |

## Security Design Principles

PixelSense follows these security principles by design:

- **Zero network access** — The application makes no outbound connections
- **No telemetry** — No data collection of any kind
- **Local-only storage** — Configuration stored in local `.json` files
- **Memory-only analysis** — Screen content is analyzed in memory and never persisted
- **Minimal permissions** — Only hardware APIs required for brightness control are accessed
- **Tauri IPC hardening** — Frontend-to-backend communication is restricted to defined command endpoints

## Dependency Management

- Automated dependency updates via [Dependabot](.github/dependabot.yml)
- Rust dependencies audited with `cargo audit`
- npm dependencies monitored for known vulnerabilities
