# Windows Compatibility Certification

## Certification Scope
Verify PixelSense resilience across multiple hardware permutations (High DPI, 125%/150% scaling, Multi-monitor hot-plugging, HDR/SDR, Sleep/Hibernate resumption, Intel/AMD/Nvidia GPUs).

## Test Environment
- OS: Windows Sandbox

## Methodology
Attempt to run the compiled application under varying virtual display conditions.

## Evidence
- `docs/release/evidence/compatibility/matrix.log`

## Pass / Fail
**FAIL (Not Verified)**

## Hardware Validation Matrix

| Windows Version | CPU | GPU | Display Type | Resolution | Scaling | HDR/SDR | Monitor Count | Ambient Sensor | DDC/CI | Result | Notes |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Windows 11 | Virtual | Virtual | Virtual | 1080p | 100% | SDR | 1 | N/A | N/A | Not Verified | Build failed due to missing Rust |
| Windows 10 | AMD | Nvidia | External | 4K | 150% | HDR | 2 | Available | Available | Not Verified | Unavailable Hardware |
| Windows 11 | Intel | Intel | Laptop | 1440p | 125% | SDR | 1 | Available | N/A | Not Verified | Unavailable Hardware |

## Known Limitations
We lack access to physical monitors with DDC/CI hardware and integrated ambient light sensors on this test node.

## Risk Assessment
- **Severity:** High
- **Impact:** We cannot guarantee the multi-monitor DPI scaling behavior.

## Certification Decision
**UNCERTIFIED**. Awaiting physical device lab testing. Added to the Risk Register.
