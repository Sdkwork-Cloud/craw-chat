# 09AT Chat CLI Lowercase Bearer Normalization Contract Implementation Plan

## Goal

Make provided bearer tokens case-insensitive at the CLI boundary so `authorization` keeps canonical header form while `token` and `--token-only` stay bare-token material.

## Implementation

1. Freeze provided lowercase `bearer ` input in `tools/chat-cli/tests/chat_cli_contract_test.rs`
2. Reuse one shared bearer-prefix parser for header normalization and token extraction
3. Keep the output envelope stable:
   - `source`
   - `authorization`
   - `token`
   - `claims`
4. Backwrite the Step 12 operator guide and validation index

## Non-Goals

- No new CLI flags
- No wrapper behavior change
- No SDK contract change
- No HTTP request-path change
