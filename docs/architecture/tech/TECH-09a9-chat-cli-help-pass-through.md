> Migrated from `docs/架构/09AV-chat-cli-cmd-help-pass-through-contract-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09AV Chat CLI CMD Help Pass-Through Contract Implementation Plan

## Goal

Make `bin/chat-cli.cmd --help` preserve the raw CLI help contract instead of rewriting flags through the generic PowerShell forwarder.

## Implementation

1. Freeze the Windows `.cmd` help surface in `tools/chat-cli/tests/chat_cli_e2e_test.rs`
2. Replace the generic normalized-forwarder call in `bin/chat-cli-local.cmd` with direct raw argument pass-through
3. Keep PowerShell startup flags unchanged:
   - `-NoProfile`
   - `-ExecutionPolicy Bypass`
4. Backwrite the Step 12 operator guide and validation index

## Non-Goals

- No `.ps1` wrapper change
- No `.sh` wrapper change
- No CLI parser change
- No new wrapper feature

