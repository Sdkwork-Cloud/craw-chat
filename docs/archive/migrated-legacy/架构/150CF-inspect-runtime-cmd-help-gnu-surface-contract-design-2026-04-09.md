# Inspect Runtime CMD Help GNU Surface Contract Design

## Decision

- Treat `inspect-runtime-local.cmd --help` as part of the visible Windows operator contract, not as an incidental wrapper detail.

## Contract

- `inspect-runtime-local.cmd --help` must surface:
  - `Usage: cmd /c .\bin\inspect-runtime-local.cmd [--profile <standalone.split-services.development|standalone.split-services.development>] [--runtime-dir <path>] [--json] [--release]`
  - the existing PowerShell usage line

## Rationale

- The repo already documents GNU-style Windows flags for `.cmd` entrypoints.
- A `.cmd` help surface that only shows PowerShell parameter names creates a discoverability split between documentation and the shipped Windows wrapper.
- Adding the `.cmd` usage line in the PowerShell help branch is the smallest possible fix because the wrapper already forwards `--help` correctly.

## Boundary

- This design covers `inspect-runtime-local.cmd --help` only.
- Adjacent runtime-ops wrappers require separate evidence before they are grouped into the same closure.
