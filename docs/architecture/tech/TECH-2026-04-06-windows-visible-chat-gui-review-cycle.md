> Migrated from `docs/review/2026-04-06-windows-visible-chat-gui-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-06 Windows 可见聊天窗体 Review

## 现象

用户反馈执行开窗脚本后，没有看到任何新窗口。

## 根因

### 1. 控制台启动路径是 headless 的

在当前自动化宿主中：

- `cmd.exe /k ...`
- `powershell.exe -NoExit ...`
- `Start-Process powershell.exe`

都可能生成无 `MainWindowHandle` 的控制台进程。

这类进程虽然处于当前交互会话，但不会在桌面上显示控制台窗口。

### 2. 早期 GUI 包装脚本存在 PowerShell 5.1 兼容问题

- `ProcessStartInfo.ArgumentList` 在当前环境不可用
- `open-chat-test.ps1` 初版 GUI 启动未加 `-NoProfile`

导致 GUI 窗口脚本会提前退出。

### 3. 本地服务后台托管方式不稳定

`retired-lifecycle-start.ps1` 的既有后台模式会出现：

- 启动后健康检查一度成功
- 随后进程退出

这会影响窗口建立后的实时聊天验证。

## 修复

### 已完成

- 新增 `bin/chat-window-gui.ps1`
- 新增 `bin/chat-window-gui.cmd`
- `open-chat-test.ps1` 默认改为 GUI 窗口模式
- `open-chat-test.ps1` 补齐 `-NoProfile`
- GUI 子进程参数拼接改为兼容 PowerShell 5.1 的 `Arguments` 字符串
- 服务不健康时改为“独立宿主前台托管”模式启动 `sdkwork-im-server.exe`

## 实机验证

### fresh evidence

- 执行 `bin/open-chat-test.ps1 -SkipStart`
- 成功创建会话：`c_demo_20260406210328`
- 系统可枚举到 2 个可见聊天窗口：
  - `sdkwork-im [owner] [c_demo_20260406210328]`
  - `sdkwork-im [guest] [c_demo_20260406210328]`
- 初始化消息成功发送到该会话

## 当前可直接使用

用户现在可以直接在这两个 GUI 聊天窗中进行收发测试。

