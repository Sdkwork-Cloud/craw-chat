# 150AY Open Chat Test CMD Validation-Message Special-Char Contract Design

## Problem

The Windows `open-chat-test.cmd` path accepted the scripted-validation command line but did not preserve `--validation-message` literally when the value contained `!`.
That created a hidden operator contract split:

- the PowerShell scripted-validation flow itself could handle the message
- the Windows `.cmd` entry altered the value before it reached the script

## Decision

- Keep the current `open-chat-test.ps1` argument surface and scripted-validation semantics.
- Remove `open-chat-test.cmd` from `_cmd-forward-powershell.cmd` for this loop.
- Define the Windows scripted-validation wrapper boundary as literal passthrough for user-supplied message content.

## Rationale

- This is the minimum change that restores operator input fidelity without broadening the generic forwarder scope.
- It keeps the fix on the wrapper that owns the broken operator contract.
- It preserves the earlier GNU-style named flag closure while removing one shared source of argument mutation.

## Boundary

- This design only covers `open-chat-test.cmd` preserving `--validation-message` content on Windows.
- It does not claim that `_cmd-forward-powershell.cmd` is generally safe for all special-character inputs in unrelated scripts.
