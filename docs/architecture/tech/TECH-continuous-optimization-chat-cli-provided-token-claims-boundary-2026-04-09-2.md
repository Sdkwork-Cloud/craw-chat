> Migrated from `docs/step/continuous-optimization-chat-cli-provided-token-claims-boundary-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Chat CLI Provided-Token Claims Boundary

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `sdkwork-im-cli token` already separated generated and provided token sources.
- But `providedBearerToken` still exposed `claims` synthesized from local CLI inputs, which overstated what the CLI knew about an external token.
- This was the next smallest real Step 12 contract gap after token-only and lowercase bearer normalization.

## Closure Target

1. Add a regression test that proves provided bearer tokens never pretend local CLI inputs are decoded external-token claims.
2. Patch the token command with the minimum behavioral change.
3. Backwrite the Step 12 operator guide and validation index.

## Actual Delivery

- Added `test_chat_cli_token_command_does_not_synthesize_claims_for_provided_bearer_tokens`
- Fixed `tools/chat-cli/src/lib.rs` so:
  - generated tokens still expose locally signed `claims`
  - provided tokens return `claims = null`
  - `authorization` and `token` keep the existing canonicalized material boundary
- Updated:
  - `docs/部署/CLI聊天验证与兼容矩阵.md`
  - `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- Backwrote review and architecture docs for this micro-loop

## Verification

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_chat_cli_token_command_does_not_synthesize_claims_for_provided_bearer_tokens -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Next Round

- Continue from Step 12 public CLI and operator seams.
- Prefer the next smallest honest behavior or discoverability gap, not bulk documentation fill.

