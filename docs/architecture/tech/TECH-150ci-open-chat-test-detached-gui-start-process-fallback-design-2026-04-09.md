> Migrated from `docs/架构/150CI-open-chat-test-detached-gui-start-process-fallback-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150CI open-chat-test detached GUI Start-Process fallback design - 2026-04-09

## Decision

Adopt a three-tier Windows popup launcher chain in `Start-DetachedPowerShellWindow`:

1. `Win32_Process.Create`
2. `Start-Process powershell.exe -PassThru`
3. `wscript.exe` VBS fallback

## Rationale

- tier 1 preserves the original detached-host design goal
- tier 2 is the simplest locally proven path for GUI script startup under restricted automation hosts
- tier 3 remains a compatibility escape hatch

## Non-Goals

- no redesign of GUI polling
- no service lifecycle rewrite
- no new popup-specific configuration surface

## Verification Design

Use layered proof:

- contract test to freeze the launcher order
- scripted validation regression to protect non-GUI behavior
- same-session runtime diagnostics to prove owner/guest windows reach `form shown`

