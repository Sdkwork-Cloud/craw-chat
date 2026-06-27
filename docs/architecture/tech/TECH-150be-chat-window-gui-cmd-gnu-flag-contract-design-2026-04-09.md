> Migrated from `docs/架构/150BE-chat-window-gui-cmd-gnu-flag-contract-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150BE Chat Window GUI CMD GNU-Flag Contract Design

## Problem

The Windows `chat-window-gui.cmd` help surface advertised GNU-style named flags, but `chat-window-gui.ps1` still lacked aliases for the hyphenated parameter names required to bind those flags at runtime.
That created a visible operator split:

- the local help contract was GNU-style
- the executable parameter-binding contract was still PowerShell-style for key inputs

## Decision

- Keep the current GUI wrapper and GUI runtime flow unchanged.
- Add GNU-style aliases to the hyphenated GUI parameters in `chat-window-gui.ps1`.
- Define runtime acceptance of the advertised GNU-style named flags as part of the visible Windows GUI wrapper contract.

## Rationale

- This is the minimum change that restores runtime parity with the already-published local help contract.
- It keeps the contract close to the script that owns the parameter binding.
- It avoids another wrapper-layer redesign while still making the advertised `.cmd` surface truthful.

## Boundary

- This design only covers `chat-window-gui.cmd` GNU-style runtime argument acceptance on Windows.
- It does not redefine the generic argument strategy for unrelated wrappers.

