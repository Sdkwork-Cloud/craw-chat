# 150AW Open Chat Test CMD GNU-Flag Contract Design

## Problem

The Windows `open-chat-test.cmd` path accepted the command line but did not honor GNU-style named flags used by the Bash operator path.
That created a hidden contract split:

- `.sh` accepted `--conversation-id`, `--owner-user-id`, `--guest-user-id`, `--skip-start`, `--scripted-validation`, `--validation-message`, and `--json`
- `.cmd` silently ignored several of those flags and fell back to the default interactive flow

## Decision

- Keep the Windows `.cmd` entry as-is for this loop.
- Expand `open-chat-test.ps1` so the PowerShell script itself accepts GNU-style aliases for the hyphenated operator parameters.
- Define scripted validation as a cross-shell contract shared by:
  - `open-chat-test.ps1`
  - `open-chat-test.sh`
  - `open-chat-test.cmd`

## Rationale

- This is the minimum change that restores Windows automation parity without broadening the generic forwarder.
- It keeps the contract close to the script that owns the behavior.
- It avoids forcing every caller to learn separate PowerShell-only flag names when the repo already documents GNU-style operator flags.

## Boundary

- This design only covers the `open-chat-test` Windows scripted-validation argument contract.
- It does not redefine generic PowerShell forwarding for unrelated operational scripts.
