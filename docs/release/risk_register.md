# Release Risk Register

| Identifier | Description | Severity | Likelihood | Impact | Mitigation | Owner | Release Decision |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| RSK-001 | Application cannot be compiled on current CI node due to missing Rust/MSVC | Critical | 100% | Blocks all Windows builds and installer generation. | Provision a fully equipped Windows GitHub Actions Runner. | Release Engineering | **NO-GO** |
| RSK-002 | Multi-Monitor DDC/CI scaling cannot be tested without hardware | High | Unknown | Monitor brightness might change on the wrong display. | Test in a physical device lab before publishing RC. | QA | **NO-GO** |
| RSK-003 | 8-hour continuous long-run validation could not execute | High | Unknown | Potential memory leaks remain undiscovered. | Execute long-run test on provisioned node. | Performance Engineering | **NO-GO** |
