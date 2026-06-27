# Continuous Optimization: Chat CLI Lowercase Bearer Normalization Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `sdkwork-im-cli token` already distinguished default header output and `--token-only` bare token output.
- But externally provided lowercase bearer input still leaked `bearer ` into `authorization`, `token`, and `--token-only`.
- This was the next smallest real CLI contract gap after the token-only fix.

## Closure Target

1. Add a regression test that proves provided lowercase bearer input is normalized in both default and `--token-only` paths.
2. Patch the CLI with one shared bearer-prefix rule.
3. Backwrite the Step 12 operator guide and validation index.

## Actual Delivery

- Added `test_chat_cli_token_command_normalizes_lowercase_bearer_prefix`
- Added `strip_bearer_prefix` so header normalization and token extraction use one case-insensitive bearer-prefix rule
- Fixed `tools/chat-cli/src/lib.rs` so:
  - default output canonicalizes `authorization = Bearer <token>`
  - `token = <token>` remains bare token material
  - `--token-only` keeps returning bare token material
- Updated:
  - `docs/部署/CLI聊天验证与兼容矩阵.md`
  - `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- Backwrote review and architecture docs for this micro-loop

## Verification

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_chat_cli_token_command_normalizes_lowercase_bearer_prefix -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Next Round

- Continue from Step 12 public CLI and operator seams.
- Prefer the next smallest honest contract or discoverability gap, not broad documentation expansion.
