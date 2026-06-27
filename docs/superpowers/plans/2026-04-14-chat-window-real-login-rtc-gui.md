# Chat Window Real Login And RTC GUI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add visible real-login controls and RTC signaling controls to the local chat window so operators can test chat and RTC flows against the real backend.

**Architecture:** Extend the existing PowerShell GUI instead of introducing a new tool. Centralize auth and request helpers so the window can acquire a real bearer token, use it for timeline and message operations, and drive RTC endpoints from the same authenticated session.

**Tech Stack:** PowerShell, Windows Forms, Rust `chat-cli`, local-minimal-node, Rust integration tests

---

### Task 1: Lock the operator contract with failing tests

**Files:**
- Modify: `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- Test: `tools/chat-cli/tests/chat_cli_e2e_test.rs`

- [ ] **Step 1: Write the failing test**

Add or extend Windows wrapper tests that assert the operator contract now includes visible login and RTC-related help text or launch options for the GUI entrypoints.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p sdkwork-im-cli test_chat_window_gui_cmd_help_surfaces_gnu_style_named_flags -- --exact`
Expected: FAIL because the help text does not yet describe the expanded operator surface.

- [ ] **Step 3: Write minimal implementation**

Update the relevant script help strings and wrapper-exposed usage text so the new contract is visible.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p sdkwork-im-cli test_chat_window_gui_cmd_help_surfaces_gnu_style_named_flags -- --exact`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add tools/chat-cli/tests/chat_cli_e2e_test.rs bin/chat-window-gui.ps1 bin/chat-window-gui.cmd
git commit -m "test(gui): lock login and rtc launch contract"
```

### Task 2: Add failing regression tests for real-login-driven operator setup

**Files:**
- Modify: `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- Test: `tools/chat-cli/tests/chat_cli_e2e_test.rs`

- [ ] **Step 1: Write the failing test**

Add a regression test that prepares a real-login conversation, launches the GUI in non-eager mode, and verifies diagnostics reflect manual login readiness rather than immediate network failure.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p sdkwork-im-cli test_chat_window_gui_manual_login_launch_is_idle -- --exact`
Expected: FAIL because the current GUI always assumes authenticated chat mode.

- [ ] **Step 3: Write minimal implementation**

Teach the GUI script to distinguish pre-authenticated launch from manual-login launch and to suppress send/refresh until auth is resolved.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p sdkwork-im-cli test_chat_window_gui_manual_login_launch_is_idle -- --exact`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add tools/chat-cli/tests/chat_cli_e2e_test.rs bin/chat-window-gui.ps1
git commit -m "test(gui): cover manual real-login launch flow"
```

### Task 3: Add failing RTC operator workflow test

**Files:**
- Modify: `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- Test: `tools/chat-cli/tests/chat_cli_e2e_test.rs`

- [ ] **Step 1: Write the failing test**

Add a real-login integration test that exercises the intended operator RTC sequence:

- owner login
- guest login
- create conversation
- add member
- create RTC session in `video` mode
- invite
- guest sends `rtc.offer`
- guest accepts
- owner ends
- timeline contains `rtc.invite`, `rtc.offer`, `rtc.accept`, `rtc.end`

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p sdkwork-im-cli test_chat_cli_real_login_rtc_operator_flow -- --exact`
Expected: FAIL because the current helper layer does not expose one cohesive operator workflow for RTC from the window support layer.

- [ ] **Step 3: Write minimal implementation**

Add reusable helper functions that mirror the intended GUI RTC actions and make the test pass using the real backend APIs.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p sdkwork-im-cli test_chat_cli_real_login_rtc_operator_flow -- --exact`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add tools/chat-cli/tests/chat_cli_e2e_test.rs bin/chat-window-gui.ps1 bin/open-chat-test.ps1
git commit -m "test(rtc): cover real-login operator rtc workflow"
```

### Task 4: Refactor PowerShell auth and request helpers

**Files:**
- Modify: `bin/chat-window-gui.ps1`
- Modify: `bin/chat-window.ps1`
- Modify: `bin/open-chat-test.ps1`

- [ ] **Step 1: Write the failing test**

Use the new helper-focused regression from Task 3 as the failing spec for shared auth/request behavior.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p sdkwork-im-cli test_chat_cli_real_login_rtc_operator_flow -- --exact`
Expected: FAIL against current helper structure.

- [ ] **Step 3: Write minimal implementation**

Refactor the PowerShell scripts so they share:

- auth-context resolution
- real-login execution
- guarded authenticated command building
- RTC endpoint invocation

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p sdkwork-im-cli test_chat_cli_real_login_rtc_operator_flow -- --exact`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add bin/chat-window-gui.ps1 bin/chat-window.ps1 bin/open-chat-test.ps1
git commit -m "refactor(gui): share real-login and rtc helper flow"
```

### Task 5: Implement the GUI login panel

**Files:**
- Modify: `bin/chat-window-gui.ps1`
- Test: `tools/chat-cli/tests/chat_cli_e2e_test.rs`

- [ ] **Step 1: Write the failing test**

Extend the GUI launch diagnostics test to assert that the form exposes manual login mode with connection status that does not pretend to be authenticated.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p sdkwork-im-cli test_chat_window_gui_manual_login_launch_is_idle -- --exact`
Expected: FAIL

- [ ] **Step 3: Write minimal implementation**

Add visible fields and buttons for:

- base URL
- tenant
- conversation
- user id
- login
- password
- session id
- client route id
- login action

Gate send/refresh until auth succeeds.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p sdkwork-im-cli test_chat_window_gui_manual_login_launch_is_idle -- --exact`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add bin/chat-window-gui.ps1 tools/chat-cli/tests/chat_cli_e2e_test.rs
git commit -m "feat(gui): add visible real-login controls"
```

### Task 6: Implement the GUI RTC panel

**Files:**
- Modify: `bin/chat-window-gui.ps1`
- Test: `tools/chat-cli/tests/chat_cli_e2e_test.rs`

- [ ] **Step 1: Write the failing test**

Use the RTC operator workflow test as the failing specification for create/invite/signal/accept/end behavior.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p sdkwork-im-cli test_chat_cli_real_login_rtc_operator_flow -- --exact`
Expected: FAIL until the window helper layer can drive the RTC lifecycle consistently.

- [ ] **Step 3: Write minimal implementation**

Add GUI controls for:

- rtc session id
- mode
- signaling stream id
- artifact message id
- signal type
- schema ref
- payload
- create/invite/accept/reject/end/send-signal/fetch-credentials/fetch-recording

Append results to diagnostics and refresh the timeline after durable RTC actions.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p sdkwork-im-cli test_chat_cli_real_login_rtc_operator_flow -- --exact`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add bin/chat-window-gui.ps1 tools/chat-cli/tests/chat_cli_e2e_test.rs
git commit -m "feat(gui): add rtc signaling control panel"
```

### Task 7: Align the launcher flow

**Files:**
- Modify: `bin/open-chat-test.ps1`
- Modify: `bin/chat-window.ps1`
- Test: `tools/chat-cli/tests/chat_cli_e2e_test.rs`

- [ ] **Step 1: Write the failing test**

Add or extend launcher tests to assert that seeded auto-login still works while manual-login windows remain available for interactive validation.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p sdkwork-im-cli test_open_chat_test_cmd_help_surfaces_gnu_style_named_flags -- --exact`
Expected: FAIL if the launcher contract does not expose the updated modes clearly enough.

- [ ] **Step 3: Write minimal implementation**

Update launcher help text and flow so operators can choose manual-login or auto-login windows without breaking existing scripted validation.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p sdkwork-im-cli test_open_chat_test_cmd_help_surfaces_gnu_style_named_flags -- --exact`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add bin/open-chat-test.ps1 bin/chat-window.ps1 tools/chat-cli/tests/chat_cli_e2e_test.rs
git commit -m "feat(tooling): align launcher with gui login and rtc flow"
```

### Task 8: Verify the full commercial-grade local flow

**Files:**
- Modify: `docs/superpowers/specs/2026-04-14-chat-window-real-login-rtc-gui-design.md`
- Modify: `docs/superpowers/plans/2026-04-14-chat-window-real-login-rtc-gui.md`

- [ ] **Step 1: Run focused automated verification**

Run:

```bash
cargo test -p sdkwork-im-cli test_chat_window_gui_cmd_help_surfaces_gnu_style_named_flags -- --exact
cargo test -p sdkwork-im-cli test_chat_window_gui_manual_login_launch_is_idle -- --exact
cargo test -p sdkwork-im-cli test_chat_cli_real_login_rtc_operator_flow -- --exact
```

Expected: PASS

- [ ] **Step 2: Run broader package verification**

Run: `cargo test -p sdkwork-im-cli`
Expected: PASS

- [ ] **Step 3: Run real local service validation**

Run:

```powershell
powershell -ExecutionPolicy Bypass -File bin/start-local.ps1 -ProfileName local-minimal
powershell -ExecutionPolicy Bypass -File bin/open-chat-test.ps1 -SkipStart
```

Expected: two windows open, operators can log in, exchange a message, and drive the RTC signaling lifecycle.

- [ ] **Step 4: Commit**

```bash
git add docs/superpowers/specs/2026-04-14-chat-window-real-login-rtc-gui-design.md docs/superpowers/plans/2026-04-14-chat-window-real-login-rtc-gui.md
git commit -m "docs: capture gui real-login and rtc verification flow"
```
