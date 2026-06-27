> Migrated from `docs/review/continuous-optimization-chat-cli-token-only-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Chat CLI Token-Only Contract

## Context

- Current loop: continue after Step 12 closure, only close a real remaining gap.
- Surface: `tools/chat-cli`
- Contract: `sdkwork-im-cli token --token-only`

## Gap

- The CLI already parsed `--token-only`, but the returned `authorization` field still kept `Bearer `.
- That made the flag behaviorally ineffective and left the operator contract ambiguous.
- Step 12 operator docs also did not freeze the `generatedBearerToken / providedBearerToken` source semantics or the `--token-only` bare-token boundary.

## Decision

- Keep the token JSON envelope stable.
- Default `token` output remains:
  - `authorization = Bearer <token>`
  - `token = <token>`
- `token --token-only` now narrows `authorization` to the bare token value.
- `token` continues to expose:
  - `source = generatedBearerToken` when the CLI signs from `--public-bearer-secret`
  - `source = providedBearerToken` when `--bearer-token` is supplied

## Changed Files

- `tools/chat-cli/src/lib.rs`
- `tools/chat-cli/tests/chat_cli_contract_test.rs`
- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- `docs/step/continuous-optimization-chat-cli-token-only-contract-2026-04-09.md`
- `docs/架构/09AS-chat-cli-token-only-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150AS-chat-cli-token-only-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_chat_cli_token_command_freezes_header_and_token_only_contract -- --exact --nocapture
```

- Failed before the patch because `--token-only` still returned `Bearer <token>`.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- This loop only fixed the token contract seam.
- Other Step 12 surfaces remain in continuous-optimization mode and should still prefer the next smallest real gap instead of bulk doc expansion.

