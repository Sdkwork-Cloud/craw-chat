> Migrated from `docs/架构/09CI-open-chat-test-detached-gui-start-process-fallback-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09CI open-chat-test detached GUI Start-Process fallback implementation plan - 2026-04-09

## Scope

Only fix the default Windows GUI launcher in `bin/open-chat-test.ps1`. Do not change conversation creation, scripted validation, or `bin/chat-window-gui.ps1` polling logic.

## Plan

1. freeze the contract with a failing Windows test
2. insert a `Start-Process` fallback after `Win32_Process.Create`
3. preserve the existing VBS last resort
4. verify scripted validation still passes
5. verify same-session popup launch emits owner and guest GUI logs

## Exit Criteria

- default popup mode can return window pids through a stable PowerShell launcher path
- owner and guest GUI processes reach `form shown`
- scripted validation remains green

