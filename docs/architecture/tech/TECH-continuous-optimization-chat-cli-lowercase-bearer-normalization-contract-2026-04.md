> Migrated from `docs/review/continuous-optimization-chat-cli-lowercase-bearer-normalization-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Chat CLI Lowercase Bearer Normalization Contract

## Context

- Current loop: continue after Step 12 closure, only close a real remaining gap.
- Surface: `tools/chat-cli`
- Contract: `sdkwork-im-cli token [--token-only]` with externally provided bearer material

## Gap

- The previous loop fixed `--token-only`, but provided lowercase bearer input still leaked casing into runtime output.
- `--bearer-token "bearer <token>"` returned `authorization = bearer <token>` instead of the canonical header form.
- The same lowercase prefix also leaked into `token` and `--token-only`, which broke the boundary between header-form material and bare-token material.

## Decision

- Treat bearer prefixes as case-insensitive on input.
- Canonicalize default `authorization` output to `Bearer <token>`.
- Keep `token = <token>` as the bare token value.
- Keep `token --token-only` returning the same bare token in `authorization`.
- Preserve `source = providedBearerToken`.

## Changed Files

- `tools/chat-cli/src/lib.rs`
- `tools/chat-cli/tests/chat_cli_contract_test.rs`
- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- `docs/step/continuous-optimization-chat-cli-lowercase-bearer-normalization-contract-2026-04-09.md`
- `docs/架构/09AT-chat-cli-lowercase-bearer-normalization-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150AT-chat-cli-lowercase-bearer-normalization-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_chat_cli_token_command_normalizes_lowercase_bearer_prefix -- --exact --nocapture
```

- Failed before the patch because `authorization` still stayed `bearer <token>`.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_chat_cli_token_command_normalizes_lowercase_bearer_prefix -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- This loop only fixed the lowercase bearer normalization seam on the token command surface.
- Step 12 remains in continuous-optimization mode and should continue from the next smallest honest CLI, SDK, or operator gap.

