# Continuous Optimization: Chat CLI Token-Only Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `sdkwork-im-cli token --token-only` was parsed but not actually narrowing the returned auth material.
- This was a small, real, user-facing contract gap on the CLI operator surface.

## Closure Target

1. Add a regression test that proves:
   - default `token` keeps `authorization = Bearer <token>`
   - `token --token-only` returns a bare token
   - `source` distinguishes `generatedBearerToken` and `providedBearerToken`
2. Patch the CLI with the minimum behavioral change.
3. Backwrite the Step 12 operator docs and validation index.

## Actual Delivery

- Added `test_chat_cli_token_command_freezes_header_and_token_only_contract`
- Fixed `tools/chat-cli/src/lib.rs` so `--token-only` returns the bare token in `authorization`
- Updated:
  - `docs/部署/CLI聊天验证与兼容矩阵.md`
  - `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- Backwrote review and architecture docs for this micro-loop

## Verification

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_chat_cli_token_command_freezes_header_and_token_only_contract -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture
cargo fmt --all --check
```

## Next Round

- Continue from Step 12 public operator/SDK seams.
- Prefer the next smallest honest behavior or discoverability gap, not another broad README expansion.
