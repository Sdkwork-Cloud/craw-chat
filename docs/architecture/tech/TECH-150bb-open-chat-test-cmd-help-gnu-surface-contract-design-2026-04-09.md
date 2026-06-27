> Migrated from `docs/架构/150BB-open-chat-test-cmd-help-gnu-surface-contract-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150BB Open Chat Test CMD Help GNU-Surface Contract Design

## Problem

The Windows `open-chat-test.cmd` wrapper accepted GNU-style named flags at runtime, but its help output only advertised PowerShell-style parameter names.
That created a hidden operator split:

- the wrapper contract was GNU-style
- the documented local help contract was PowerShell-style

## Decision

- Keep the current runtime argument parsing and launch flow.
- Expand `open-chat-test.ps1` help output so Windows operators see an explicit `.cmd` usage line with GNU-style named flags.
- Define `.cmd --help` as part of the wrapper contract, not as an incidental PowerShell echo.

## Rationale

- This is the minimum change that restores help discoverability without touching the working scripted-validation path.
- It keeps the contract close to the script that owns the help text.
- It prevents operators from learning a parameter surface that differs from the Windows wrapper they actually invoke.

## Boundary

- This design only covers `open-chat-test.cmd --help` discoverability on Windows.
- It does not redefine help output conventions for unrelated wrappers.

