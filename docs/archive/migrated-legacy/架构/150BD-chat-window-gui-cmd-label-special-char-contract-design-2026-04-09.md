# 150BD Chat Window GUI CMD Label Special-Character Contract Design

## Problem

The Windows `chat-window-gui.cmd` wrapper still relied on `_cmd-forward-powershell.cmd`, which enabled delayed expansion and stripped `!` from label values before the GUI script received them.
That created a runtime contract split:

- direct PowerShell GUI launches preserved literal label values
- the Windows `.cmd` wrapper silently changed those same values

## Decision

- Keep the existing GUI script behavior and diagnostics format.
- Bypass `_cmd-forward-powershell.cmd` for `chat-window-gui.cmd` and invoke `chat-window-gui.ps1` directly through PowerShell.
- Define literal label preservation as part of the visible Windows GUI wrapper contract.

## Rationale

- This is the minimum change that fixes the runtime boundary without redesigning the generic forwarder.
- It matches the already-proven pattern used for `chat-window.cmd` and `open-chat-test.cmd`.
- It prevents visible GUI session identity labels from being silently rewritten at the wrapper boundary.

## Boundary

- This design only covers `chat-window-gui.cmd` literal preservation for label values on Windows.
- It does not redefine the generic `_cmd-forward-powershell.cmd` strategy for unrelated wrappers.
