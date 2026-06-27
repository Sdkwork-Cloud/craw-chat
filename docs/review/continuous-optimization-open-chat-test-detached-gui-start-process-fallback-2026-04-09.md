# Continuous Optimization - open-chat-test detached GUI Start-Process fallback - 2026-04-09

## Problem

`bin/open-chat-test.ps1` default GUI mode created the conversation, but default popup launch could fail under restricted Windows automation hosts when `Win32_Process.Create` was unavailable and the script fell straight to the VBS launcher. In this environment the fallback path produced no new GUI diagnostics logs, while direct `Start-Process powershell.exe ... chat-window-gui.ps1` succeeded.

## Root Cause

The detached launcher skipped the only locally verified stable mid-tier path:

1. `Win32_Process.Create`
2. direct `Start-Process powershell.exe -PassThru`
3. VBS `wscript.exe` last resort

Without step 2, the popup path depended on a weaker fallback that was not reliable in the current automation host.

## Fix

- add a `Start-Process -FilePath "powershell.exe" -ArgumentList $ArgumentList -WorkingDirectory $workingDirectory -PassThru -WindowStyle Normal` fallback inside `Start-DetachedPowerShellWindow`
- keep `Win32_Process.Create` as first choice
- keep `wscript.exe` as final fallback
- tighten the Windows contract test so the script must preserve this three-tier launcher chain

## Verification

- `cargo test -p sdkwork-im-cloud-gateway --offline test_open_chat_test_ps1_uses_detached_gui_launcher_for_default_windows_mode -- --exact --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_open_chat_test_powershell_scripted_validation_emits_json_summary -- --exact --nocapture`
- `cargo fmt --all --check`
- same-session runtime proof:
  - `pnpm dev:server`
  - `bin/open-chat-test.ps1 -SkipStart -ConversationId c_popup_20260409122030`
  - output included `ownerWindowPid: 3840` and `guestWindowPid: 74040`
  - GUI logs:
    - `20260409121931-owner-c_popup_20260409122030.log`
    - `20260409121931-guest-c_popup_20260409122030.log`
  - both logs recorded `script start` and `form shown`

## Boundary

The current Codex shell host reaps detached child processes after the command exits, so cross-command popup persistence cannot be used as a truth source here. Same-session GUI logs were used as the reliable proof.

## Next

- add a non-visual regression harness for detached GUI launch if we need stronger automation proof than script-contract plus same-session diagnostics
- if needed, persist GUI success markers beyond `form shown`, for example a first successful timeline refresh stamp
