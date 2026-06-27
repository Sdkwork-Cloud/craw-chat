> Migrated from `docs/review/2026-04-06-chat-window-gui-polling-stability-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-06 GUI 聊天测试窗稳定性复盘

## 现象

用户反馈通过 `bin/open-chat-test.ps1` 打开的聊天窗口会在启动后自动关闭，无法完成双窗聊天验证。

## 复现证据

1. 旧版 `chat-window-gui.ps1` 采用“PowerShell WinForms + 子进程 `chat-session` + 异步 stdout/stderr 事件桥接”的方式。
2. 在当前 Windows 环境中，窗口脚本能走到 `child started pid=...`，但随后父进程会异常退出。
3. 日志中看不到稳定的 `chat-session ready` / `application run completed`，说明崩溃点发生在 GUI 主循环与异步子进程桥接之间。
4. 通过 `Win32_Process.Create` 启动的普通 PowerShell GUI 进程可以稳定存活，说明不是所有脱离进程都会被宿主回收。

## 根因判断

根因不是 IM 服务端不可用，也不是 `chat-cli` 发送能力失效，而是旧版 GUI 测试窗设计依赖了一个不稳定的运行时组合：

- Windows PowerShell 5.1
- WinForms 主循环
- 重定向子进程标准输入/输出
- `BeginOutputReadLine` / `DataReceivedEventHandler`
- 跨线程 UI 更新

这条链路在当前环境中会导致 GUI 窗口异常终止，不适合作为人工聊天验证工具。

## 修复策略

放弃旧的“长驻子进程聊天会话桥接”方案，改为更稳定的轮询式 GUI 方案：

- 窗口刷新使用 `System.Windows.Forms.Timer`
- 定时调用 `chat-cli timeline`
- 发送按钮调用 `chat-cli send-message`
- 默认窗口启动改为脱离式启动：
  - 优先 `Win32_Process.Create`
  - 失败时降级 `wscript.exe`

## 影响文件

- `bin/chat-window-gui.ps1`
- `bin/open-chat-test.ps1`
- `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`

## 新增回归约束

- `open-chat-test.ps1` 必须包含脱离式 GUI 启动能力
- `chat-window-gui.ps1` 必须采用 `Timer + timeline + send-message` 轮询架构
- `chat-window-gui.ps1` 不得再依赖 `BeginOutputReadLine`

## 验证结果

已通过：

- `cargo test -p sdkwork-im-cloud-gateway --offline test_open_chat_test_ps1_uses_detached_gui_launcher_for_default_windows_mode -- --exact`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_chat_window_gui_ps1_uses_polling_runtime_instead_of_async_child_stdio_bridge -- --exact`

人工验证已通过：

- 直接执行 `bin/open-chat-test.ps1 -SkipStart`
- 返回新会话：
  - `c_demo_20260406213321`
- 返回新窗口 PID：
  - owner `72920`
  - guest `158980`
- 命令返回 10 秒后，两个窗口进程仍然存活
- Win32 窗口标题枚举结果：
  - `sdkwork-im [owner] [c_demo_20260406213321]`
  - `sdkwork-im [guest] [c_demo_20260406213321]`

## 当前结论

现在这套 GUI 测试窗已经适合拿来做人工双窗聊天验证。它不是生产客户端，只是稳定的本地验证工具，因此优先选择“可观测、可重复、可持续打开”的轮询架构，而不是继续追求高复杂度的 PowerShell 异步子进程桥接。

