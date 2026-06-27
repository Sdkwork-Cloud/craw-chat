# 150BC Chat Window GUI CMD Help GNU-Surface Contract Design

## Problem

The Windows `chat-window-gui.cmd` wrapper accepted the visible GUI launch contract, but its help output only advertised PowerShell-style parameter names.
That created a hidden operator split:

- the wrapper contract was invoked through `.cmd`
- the documented local help contract was PowerShell-style

## Decision

- Keep the current GUI launch and polling flow unchanged.
- Expand `chat-window-gui.ps1` help output so Windows operators see an explicit `.cmd` usage line with GNU-style named flags.
- Define `.cmd --help` as part of the visible GUI wrapper contract, not as an incidental PowerShell echo.

## Rationale

- This is the minimum change that restores help discoverability without touching the GUI execution path.
- It keeps the contract close to the script that owns the help text.
- It prevents operators from learning a parameter surface that differs from the Windows wrapper they actually invoke.

## Boundary

- This design only covers `chat-window-gui.cmd --help` discoverability on Windows.
- It does not redefine help output conventions for unrelated wrappers.
