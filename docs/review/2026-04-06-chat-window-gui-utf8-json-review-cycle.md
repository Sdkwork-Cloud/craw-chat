# 2026-04-06 GUI 聊天窗 UTF-8 JSON 编码修复复盘

## 现象

GUI 聊天窗持续报错：

- `传入的对象无效，应为“:”或“}”`

报错对象来自 `timeline` JSON，中文消息内容被破坏为乱码，例如：

- `窗口已稳定保持打开，现在可以直接双窗聊天测试`
- 被错误读取成
- `绐楀彛宸茬ǔ瀹氫繚鎸佹墦寮€锛岀幇鍦ㄥ彲浠ョ洿鎺ュ弻绐楄亰澶╂祴璇?`

同时 JSON 中该字段的结尾引号丢失，导致 `ConvertFrom-Json` 失败。

## 根因

问题不在服务端，也不在 `sdkwork-im-cli` 本身。

真实根因是：

- `chat-window-gui.ps1` 在脱离式 Windows PowerShell 宿主中
- 通过 PowerShell 管道捕获 `chat-cli.ps1` 文本输出
- 再把该文本交给 `ConvertFrom-Json`

在这个宿主环境下，外部进程输出被按错误编码解码，UTF-8 中文 JSON 被破坏。

## 关键证据

1. 直接执行：
   - `bin/chat-cli.ps1 ... timeline`
   - 输出中文 JSON 正常。
2. 在脱离式 PowerShell 宿主中，通过旧方式捕获：
   - 同一份 JSON 会变成乱码并缺失结尾引号。
3. 在脱离式 PowerShell 宿主中，改为：
   - 直接调用 `target\debug\sdkwork-im-cli.exe`
   - `ProcessStartInfo.StandardOutputEncoding = UTF8`
   - 输出恢复正常。

## 修复方案

`chat-window-gui.ps1` 的 `Invoke-ChatCliJson` 已改为：

- 直接调用 `sdkwork-im-cli.exe`
- 明确设置：
  - `StandardOutputEncoding = [System.Text.Encoding]::UTF8`
  - `StandardErrorEncoding = [System.Text.Encoding]::UTF8`
- 不再通过 PowerShell 管道捕获 `chat-cli.ps1` 的 JSON 文本

## 验证结果

已通过测试：

- `cargo test -p sdkwork-im-cloud-gateway --offline test_chat_window_gui_ps1_reads_cli_json_via_utf8_process_io -- --exact`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_chat_window_gui_ps1_uses_polling_runtime_instead_of_async_child_stdio_bridge -- --exact`

已通过人工验证：

- 当前会话：`c_demo_20260406214233`
- 当前窗口 PID：
  - owner: `149532`
  - guest: `147588`
- 当前窗口日志显示：
  - owner 已发送 `[owner] nihao`
  - guest 已发送 `[guest] hi`
- 当前 timeline 中已确认存在：
  - `[guest] hi`
  - `编码问题已修复，可以直接双窗聊天测试`
  - `[owner] nihao`

## 结论

GUI 聊天验证窗的 JSON 读取路径必须明确使用 UTF-8 读取 `sdkwork-im-cli.exe` 的标准输出。只要继续通过 PowerShell 管道抓取脚本文本输出，这个编码问题就可能在脱离式宿主中重现。
