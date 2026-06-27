> Migrated from `docs/架构/150AX-chat-window-cmd-gnu-flag-contract-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150AX Chat Window CMD GNU-Flag Contract Design

## Problem

The Windows `chat-window.cmd` path accepted the command line but did not honor the GNU-style named flags used by the Bash operator path.
That created a hidden contract split:

- `.sh` accepted `--base-url`, `--tenant-id`, `--conversation-id`, `--user-id`, `--session-id`, `--device-id`, `--label`, and `--message-prefix`
- `.cmd` exited on usage when the required GNU-style launch flags were supplied

## Decision

- Keep the Windows `.cmd` entry as-is for this loop.
- Expand `chat-window.ps1` so the PowerShell script itself accepts GNU-style aliases for the hyphenated operator parameters.
- Define interactive launch as a cross-shell contract shared by:
  - `chat-window.ps1`
  - `chat-window.sh`
  - `chat-window.cmd`

## Rationale

- This is the minimum change that restores Windows operator parity without broadening the generic forwarder.
- It keeps the contract close to the script that owns the launch defaults and `chat-cli` argument assembly.
- It avoids forcing every caller to learn separate PowerShell-only flag names when the repo already documents GNU-style operator flags.

## Boundary

- This design only covers the `chat-window` Windows interactive-launch argument contract.
- It does not redefine generic PowerShell forwarding for unrelated operational scripts.

