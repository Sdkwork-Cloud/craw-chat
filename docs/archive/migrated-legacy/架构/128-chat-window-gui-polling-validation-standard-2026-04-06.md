# GUI 聊天验证窗轮询标准 2026-04-06

## 目标

为本地 IM 服务提供稳定的 Windows 可视化聊天验证窗口，用于：

- 双窗聊天验证
- 启动后快速人工检查
- 不依赖移动端或 Web 客户端时的本地联调

## 适用范围

- `bin/chat-window-gui.ps1`
- `bin/open-chat-test.ps1`
- `bin/open-chat-test.cmd`

## 标准要求

### 1. 启动方式

Windows 默认 GUI 测试窗必须采用脱离式启动，不得依赖普通 `Start-Process` 直接挂在当前自动化宿主下。

首选：

- `Invoke-CimMethod -ClassName Win32_Process -MethodName Create`

兜底：

- `wscript.exe`

### 2. 窗口运行模型

GUI 测试窗必须采用轮询式模型，而不是异步子进程桥接模型。

允许：

- `System.Windows.Forms.Timer`
- `chat-cli timeline`
- `chat-cli send-message`

禁止继续作为标准方案：

- `BeginOutputReadLine`
- `DataReceivedEventHandler` 驱动 transcript
- 依赖单个长驻 `chat-session` 子进程承载 UI 数据流

### 3. 消息刷新

窗口必须周期性拉取 timeline，并把消息摘要渲染到 transcript。

推荐语义：

- 刷新间隔约 1 秒
- 按 `messageSeq` 升序展示
- 发送后立即触发一次刷新

### 4. 发送语义

点击 `Send` 或按回车发送时，窗口必须：

- 生成新的 `clientMsgId`
- 调用 `chat-cli send-message`
- 成功后刷新 timeline

### 5. 可诊断性

GUI 测试窗必须写入诊断日志，至少包括：

- script start
- form shown
- message sent
- timeline refresh failure
- form closing
- application run completed

## 选择原因

本标准优先保证“稳定打开、可人工验证、可快速复现”，而不是追求复杂的实时桥接方案。对于本地验证工具，轮询式窗口已经足够完成聊天验证，并且在 Windows PowerShell 环境中显著更稳定。
