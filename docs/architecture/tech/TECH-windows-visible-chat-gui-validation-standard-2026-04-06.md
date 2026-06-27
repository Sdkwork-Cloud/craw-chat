> Migrated from `docs/架构/126-windows-visible-chat-gui-validation-standard-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 126 Windows 可见聊天验证窗体标准（2026-04-06）

## 背景

在当前 Windows 自动化宿主环境中，直接启动 `cmd.exe` 或 `powershell.exe` 形式的控制台聊天窗口，会生成无窗口句柄的 headless 进程：

- 进程存在
- 会话 ID 正确
- 但桌面不可见

因此，Windows 本地人工聊天验证不能再以控制台窗口作为默认承载。

## 标准

### 1. Windows 默认使用 GUI 聊天窗体

`bin/open-chat-test.ps1` 在 Windows 上默认打开 GUI 聊天窗：

- 每个用户一个独立 WinForms 窗口
- 底层仍然复用 `chat-session`
- 通过重定向 stdin/stdout 与 CLI 子进程通信

### 2. 控制台聊天仅作为可选兼容模式

保留控制台启动脚本，但不再作为默认路径：

- `bin/chat-window.ps1`
- `bin/chat-window.cmd`

显式传入 `-UseConsoleWindows` 时才走控制台模式。

### 3. 可见性验证标准

Windows 聊天测试窗口启动成功的判定标准为：

- 进程 `MainWindowHandle` 非 0
- 窗口标题形如 `sdkwork-im [owner] [<conversationId>]`
- 窗口可在当前交互桌面会话显示

### 4. 服务启动标准

`open-chat-test.ps1` 在服务不健康时，不再依赖不稳定的后台直接拉起方式，而是使用独立 PowerShell 宿主前台托管：

- 宿主进程隐藏
- `sdkwork-im-server.exe` 在宿主内以前台方式运行
- 用健康检查轮询确认服务就绪

## 当前入口

- GUI 单窗：`bin/chat-window-gui.ps1`
- GUI 单窗命令包装：`bin/chat-window-gui.cmd`
- 双窗开窗：`bin/open-chat-test.ps1`

## 验证结果

2026-04-06 实机验证结果：

- 会话创建成功
- 两个 GUI 窗口已打开
- 进程窗口标题可被系统枚举
- 初始测试消息已成功投递到当前会话

