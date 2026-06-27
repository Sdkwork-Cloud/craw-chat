# repair-runtime-local.cmd Help GNU Surface Contract Design

## Problem

- The repo documents `repair-runtime-local.cmd` as a Windows operator entrypoint.
- Its help output exposed only PowerShell syntax, so `.cmd` users could not discover the Windows flag surface from the command they actually run.

## Decision

- Keep the shared `.cmd` forwarder unchanged.
- Extend `bin/repair-runtime-local.ps1 -Help` to print both:
  - PowerShell usage
  - GNU-style `.cmd` usage

## Rationale

- The wrapper already maps `--help` correctly.
- The missing contract was presentation, not argument forwarding or runtime behavior.
- Printing both usage forms preserves current PowerShell guidance while making the Windows wrapper self-describing.

## Non-Goals

- No change to runtime-dir repair semantics.
- No assumption that adjacent wrappers are already fixed.
