> Migrated from `docs/架构/09AU-chat-cli-provided-token-claims-boundary-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09AU Chat CLI Provided-Token Claims Boundary Implementation Plan

## Goal

Make `sdkwork-im-cli token` stop synthesizing claim payloads for externally supplied bearer tokens while keeping the output envelope stable.

## Implementation

1. Freeze provided-token claim exposure in `tools/chat-cli/tests/chat_cli_contract_test.rs`
2. Keep generated-token behavior unchanged
3. Return `claims = null` when `source = providedBearerToken`
4. Backwrite the Step 12 operator guide and validation index

## Non-Goals

- No JWT decoding feature
- No token verification feature
- No HTTP authorization-path change
- No wrapper behavior change

