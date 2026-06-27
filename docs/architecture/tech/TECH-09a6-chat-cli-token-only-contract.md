> Migrated from `docs/架构/09AS-chat-cli-token-only-contract-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09AS Chat CLI Token-Only Contract Implementation Plan

## Goal

Make `sdkwork-im-cli token --token-only` expose a real bare-token contract instead of returning the same bearer-header shape as the default mode.

## Implementation

1. Freeze the behavior in `tools/chat-cli/tests/chat_cli_contract_test.rs`
2. Keep the default path unchanged:
   - `authorization = Bearer <token>`
   - `token = <token>`
3. Change only the `--token-only` branch so `authorization = <token>`
4. Backwrite the operator guide and validation index

## Non-Goals

- No new output variant
- No wrapper behavior change
- No SDK contract change

