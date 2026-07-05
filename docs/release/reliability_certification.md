# Reliability Certification

## Certification Scope
Verify resilience against hardware removal, event storms, history corruption, and plugin panics.

## Test Environment
- OS: Windows Sandbox Node

## Methodology
- Attempted to compile frontend. Discovered a catastrophic CJS export bug in `react-window` during Rolldown tree-shaking that caused an immediate build failure (Exit Code 1).
- Applied Regression Lock: Removed `react-window` and reverted to native React mapping, verifying the build fix using `npm run build`.

## Evidence
- `docs/release/evidence/build/react_build.log`

## Pass / Fail
**PARTIAL PASS**

## Findings
- **Frontend Build Reliability:** Restored. The UI no longer depends on fragile CommonJS shims.
- **Hardware Fallbacks:** Unverified due to sandbox constraints.

## Known Limitations
Cannot inject physical display removal failures without DDC/CI access.

## Risk Assessment
- **Severity:** Medium
- **Impact:** Backend workers might panic if a monitor is aggressively unplugged during an I2C transaction.

## Certification Decision
**UNCERTIFIED**. Awaiting physical device lab execution.
