# Continuous Optimization: Chat CLI Provided-Token Claims Boundary

## Context

- Current loop: continue after Step 12 closure, only close a real remaining gap.
- Surface: `tools/chat-cli`
- Contract: `sdkwork-im-cli token [--token-only]` when `source = providedBearerToken`

## Gap

- The CLI returned `claims` even when the caller supplied an external bearer token through `--bearer-token`.
- Those `claims` were synthesized from local CLI parameters, not decoded from the provided token.
- That made the output look like the CLI had inspected or verified external token claims when it had not.

## Decision

- Preserve the token command envelope:
  - `source`
  - `authorization`
  - `token`
  - `claims`
- Keep `claims` populated only for locally generated tokens.
- When `source = providedBearerToken`, return `claims = null`.
- Keep external-token output focused on material the CLI actually knows:
  - canonical `authorization`
  - bare `token`

## Changed Files

- `tools/chat-cli/src/lib.rs`
- `tools/chat-cli/tests/chat_cli_contract_test.rs`
- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- `docs/step/continuous-optimization-chat-cli-provided-token-claims-boundary-2026-04-09.md`
- `docs/架构/09AU-chat-cli-provided-token-claims-boundary-implementation-plan-2026-04-09.md`
- `docs/架构/150AU-chat-cli-provided-token-claims-boundary-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_chat_cli_token_command_does_not_synthesize_claims_for_provided_bearer_tokens -- --exact --nocapture
```

- Failed before the patch because `providedBearerToken` still returned local synthesized claims.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_chat_cli_token_command_does_not_synthesize_claims_for_provided_bearer_tokens -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- This loop only fixed the provided-token claims boundary on the token command.
- Step 12 remains in continuous-optimization mode and should continue from the next smallest honest CLI, wrapper, or operator gap.
