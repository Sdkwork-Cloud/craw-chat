> Migrated from `docs/step/continuous-optimization-open-chat-test-detached-gui-start-process-fallback-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization Step - open-chat-test detached GUI Start-Process fallback - 2026-04-09

## Goal

Restore truthful Windows popup validation for `bin/open-chat-test.ps1` default GUI mode.

## Steps

1. rerun `bin/open-chat-test.ps1` without wrapper quoting noise
2. verify the popup path created the conversation but did not emit new GUI logs
3. reproduce `chat-window-gui.ps1` directly and confirm the form could open
4. compare detached launch paths and isolate the gap to `Start-DetachedPowerShellWindow`
5. write a failing Windows contract test requiring a `Start-Process` fallback
6. add the fallback between `Win32_Process.Create` and `wscript.exe`
7. rerun targeted tests and same-session popup verification

## Result

- popup launcher now returns real window pids when the `Start-Process` path is used
- same-session verification produced owner and guest GUI logs with `form shown`
- scripted validation path stayed green

## Evidence

- `test_open_chat_test_ps1_uses_detached_gui_launcher_for_default_windows_mode`
- `test_open_chat_test_powershell_scripted_validation_emits_json_summary`
- popup conversation: `c_popup_20260409122030`

## Next Step

Promote GUI diagnostics from `form shown` to an explicit first-sync success marker when we next touch the desktop validation path.

