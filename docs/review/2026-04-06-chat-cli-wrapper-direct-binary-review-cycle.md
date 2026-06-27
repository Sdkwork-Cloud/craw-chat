# 2026-04-06 chat-cli 包装链直连二进制修复复盘

## 背景

Windows GUI 聊天窗 `bin/chat-window-gui.ps1` 通过 `bin/chat-cli.ps1` 拉起 `chat-session` 子进程。用户反馈“窗口能打开，但聊不了”。

## 问题现象

- GUI 窗口可打开，但实际聊天体验不稳定。
- `bin/chat-cli.ps1` 的交互会话会把 Cargo 输出混入会话输出：
  - `Finished 'dev' profile ...`
  - `Running 'target\\debug\\sdkwork-im-cli.exe ...'`
- 双窗同时打开时，每个窗口都可能触发一次 `cargo run`，引入文件锁竞争和额外启动噪音。

## 根因

`bin/chat-cli-local.ps1` 与 `bin/chat-cli-local.sh` 使用 `cargo run -p sdkwork-im-cli -- ...` 作为运行时入口，而不是直接执行已构建好的 `sdkwork-im-cli` 二进制。

这会带来两个直接问题：

1. 交互会话输出被 Cargo 启动日志污染，GUI transcript 不是纯净聊天流。
2. 多窗口并发打开时会重复进入 Cargo 构建/锁等待路径，放大启动抖动。

## 修复方案

- PowerShell 本地入口改为：
  - 优先执行 `target\\debug\\sdkwork-im-cli.exe`
  - `-Release` 时执行 `target\\release\\sdkwork-im-cli.exe`
  - 仅在二进制缺失时执行 `cargo build -p sdkwork-im-cli`
- Bash 本地入口改为：
  - 优先执行 `target/debug/sdkwork-im-cli`
  - `--release` 时执行 `target/release/sdkwork-im-cli`
  - 仅在二进制缺失时执行 `cargo build -p sdkwork-im-cli`

## 涉及文件

- `bin/chat-cli-local.ps1`
- `bin/chat-cli-local.sh`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`

## 新增验证

- 新增 Windows 集成测试：
  - `test_chat_cli_powershell_entry_wrapper_can_send_interactive_messages`
- 断言点：
  - `bin/chat-cli.ps1` 入口能完成真实交互发消息
  - timeline 中能看到消息
  - 输出中不得再出现 Cargo 构建/运行噪音

## 验证结果

已通过：

- `cargo test -p sdkwork-im-cli --offline`

已完成人工复核：

- 使用与 GUI 同构的 `ProcessStartInfo + RedirectStandardInput/Output` 链路发送消息成功。
- 真实可见窗口已成功打开：
  - `sdkwork-im [owner] [c_gui_visible_20260406b]`
  - `sdkwork-im [guest] [c_gui_visible_20260406b]`
- 会话 `c_gui_visible_20260406b` 已写入种子消息：
  - `窗口已修复并重新打开，现在可以直接聊天测试`

## 结论

本次修复把聊天窗口的运行入口从“开发态 Cargo 启动器”切换为“部署态二进制入口”。这属于运行时启动链修复，不改变 IM 协议、消息模型和服务端 API，但显著提升了 GUI 交互稳定性和多窗口可用性。
