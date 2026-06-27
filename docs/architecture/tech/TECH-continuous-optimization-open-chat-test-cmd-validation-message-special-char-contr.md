> Migrated from `docs/review/continuous-optimization-open-chat-test-cmd-validation-message-special-char-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Open Chat Test CMD Validation-Message Special-Char Contract

## Context

- Current loop: continue after Step 12 closure and only close the next real Windows operator gap.
- Surface: `bin/open-chat-test.cmd -> bin/open-chat-test.ps1`
- Contract: the Windows `.cmd` scripted-validation path must preserve `--validation-message` byte-for-byte, including `!`.

## Gap

- `open-chat-test.cmd` still forwarded through `_cmd-forward-powershell.cmd`.
- That forwarder used delayed expansion while reconstructing the PowerShell argument string.
- Calling the Windows wrapper with `--validation-message "hello ... !"` silently dropped the trailing `!` before the value reached `open-chat-test.ps1`.
- The visible failure mode was subtle but real:
  - the command still exited successfully
  - the JSON summary still returned
  - but `validationMessage` no longer matched the operator input

## Decision

- Keep the existing `open-chat-test.ps1` scripted-validation behavior unchanged for this loop.
- Stop routing `open-chat-test.cmd` through `_cmd-forward-powershell.cmd`.
- Let `open-chat-test.cmd` invoke `open-chat-test.ps1` directly so Windows scripted-validation inputs remain literal.

## Changed Files

- `bin/open-chat-test.cmd`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- `docs/review/continuous-optimization-open-chat-test-cmd-validation-message-special-char-contract-2026-04-09.md`
- `docs/step/continuous-optimization-open-chat-test-cmd-validation-message-special-char-contract-2026-04-09.md`
- `docs/架构/09AY-open-chat-test-cmd-validation-message-special-char-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150AY-open-chat-test-cmd-validation-message-special-char-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_open_chat_test_cmd_wrapper_preserves_exclamation_mark_in_validation_message -- --exact --nocapture
```

- Failed before the patch because the returned JSON summary contained `hello from open-chat-test.cmd scripted validation` instead of `hello from open-chat-test.cmd scripted validation!`.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_open_chat_test_cmd_wrapper_preserves_exclamation_mark_in_validation_message -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_open_chat_test_cmd_wrapper_accepts_gnu_style_named_flags_for_scripted_validation -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- This loop only closes the Windows `open-chat-test.cmd` special-character preservation boundary for scripted validation.
- `_cmd-forward-powershell.cmd` remains a shared helper for other scripts and should not be considered globally closed by this loop.

