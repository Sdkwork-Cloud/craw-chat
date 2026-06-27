> Migrated from `docs/review/continuous-optimization-open-chat-test-cmd-gnu-flag-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Open Chat Test CMD GNU-Flag Contract

## Context

- Current loop: continue after Step 12 closure and only close a real remaining Windows operator gap.
- Surface: `bin/open-chat-test.cmd -> bin/open-chat-test.ps1`
- Contract: GNU-style named flags used by `open-chat-test.sh` must also work from the Windows `.cmd` entry.

## Gap

- `bin/open-chat-test.cmd` still used the generic PowerShell forwarder.
- `bin/open-chat-test.ps1` accepted PowerShell-style parameter names only.
- Calling the Windows wrapper with GNU-style flags such as `--base-url`, `--conversation-id`, `--skip-start`, `--scripted-validation`, and `--validation-message` silently fell back to defaults.
- The visible failure mode was severe:
  - the script ignored the supplied base URL
  - it ignored scripted-validation mode
  - it opened the default interactive window flow instead of returning JSON

## Decision

- Keep the `.cmd` wrapper unchanged for this loop.
- Make `bin/open-chat-test.ps1` explicitly accept GNU-style aliases for the hyphenated operator flags used by `.sh` and Windows automation.
- Treat `.ps1`, `.sh`, and `.cmd` as one operator contract surface for scripted validation.

## Changed Files

- `bin/open-chat-test.ps1`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- `docs/review/continuous-optimization-open-chat-test-cmd-gnu-flag-contract-2026-04-09.md`
- `docs/step/continuous-optimization-open-chat-test-cmd-gnu-flag-contract-2026-04-09.md`
- `docs/架构/09AW-open-chat-test-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150AW-open-chat-test-cmd-gnu-flag-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_open_chat_test_cmd_wrapper_accepts_gnu_style_named_flags_for_scripted_validation -- --exact --nocapture
```

- Failed before the patch because the `.cmd` wrapper path ignored GNU-style flags and fell back to the default interactive flow.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_open_chat_test_cmd_wrapper_accepts_gnu_style_named_flags_for_scripted_validation -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- This loop only closes the Windows `open-chat-test.cmd` GNU-flag boundary.
- Step 12 remains in continuous-optimization mode and should continue from the next smallest honest CLI or operator seam.

