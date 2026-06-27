# 150AZ Chat Window CMD Message-Prefix Special-Char Contract Design

## Problem

The Windows `chat-window.cmd` path accepted the interactive command line but did not preserve `--message-prefix` literally when the value contained `!`.
That created a hidden operator contract split:

- the PowerShell interactive flow itself could handle the prefix
- the Windows `.cmd` entry altered the value before it reached the script

## Decision

- Keep the current `chat-window.ps1` argument surface and launch semantics.
- Remove `chat-window.cmd` from `_cmd-forward-powershell.cmd` for this loop.
- Define the Windows interactive wrapper boundary as literal passthrough for user-supplied prefix content.

## Rationale

- This is the minimum change that restores operator input fidelity without broadening the generic forwarder scope.
- It keeps the fix on the wrapper that owns the broken operator contract.
- It preserves the earlier GNU-style named flag closure while removing one shared source of argument mutation.

## Boundary

- This design only covers `chat-window.cmd` preserving `--message-prefix` content on Windows.
- It does not claim that `_cmd-forward-powershell.cmd` is generally safe for all special-character inputs in unrelated scripts.
