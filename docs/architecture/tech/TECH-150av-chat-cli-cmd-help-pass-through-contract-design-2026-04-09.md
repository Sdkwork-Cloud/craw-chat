> Migrated from `docs/架构/150AV-chat-cli-cmd-help-pass-through-contract-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150AV Chat CLI CMD Help Pass-Through Contract Design

## Problem

The Windows `.cmd` wrapper chain reused a generic PowerShell forwarder that normalized several flags.
For `chat-cli`, this broke the basic help path by turning `--help` into `-Help`, which the CLI parser does not accept.

## Decision

- Treat `bin/chat-cli.cmd` as a raw CLI transport wrapper, not a flag-normalizing adapter.
- Forward user-supplied CLI arguments unchanged into `chat-cli-local.ps1`.
- Keep wrapper responsibilities narrow:
  - start PowerShell without profile noise
  - bypass execution policy
  - preserve exit code

## Rationale

- This is the minimum change that restores the documented Windows entry contract.
- It avoids coupling `chat-cli` to unrelated flag-normalization rules shared by operational scripts.
- It keeps the wrapper predictable for automation and shell users.

## Boundary

- This design only covers `chat-cli` Windows wrapper argument pass-through.
- It does not redefine the generic `_cmd-forward-powershell.cmd` behavior for other scripts.

