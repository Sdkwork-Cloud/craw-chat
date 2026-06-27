# 2026-04-06 本地多窗口聊天验证闭环 Review

## 问题清单

### 1. 原 CLI 缺少单命令交互式收发能力

- 现状：只有 `watch` 和 `send-message`，需要多命令配合
- 风险：无法直接在两个终端窗口里像聊天工具一样手工验证
- 处理：新增 `chat-session`

### 2. 原脚本默认地址与当前本地配置不一致

- 现状：本地运行时配置绑定 `127.0.0.1:18124`
- 风险：若脚本默认写死 `18090`，会误判服务不可用
- 处理：脚本与 CLI 都改为优先读取 `.runtime/standalone.split-services.development/config/standalone.split-services.development.env`

### 3. PowerShell 脚本变量命名冲突

- 现象：`$host` 被当作只读保留变量
- 处理：改名为 `$resolvedHost`

### 4. Windows 子进程管道测试不稳定

- 现象：直接通过子进程 stdin 做交互测试会出现挂起噪音
- 处理：测试改为基于 `execute_interactive_command_with_io` 的双工流验证

## 实施结果

### 已完成

- 新增 `chat-session` 交互式命令
- 新增 `chat-window` 单窗口启动脚本
- 新增 `open-chat-test` 多窗口开窗脚本
- Windows 实际执行 `open-chat-test.ps1` 成功
- 本地服务成功启动并创建测试会话

### 已验证

- `cargo fmt --all`
- `cargo test -p sdkwork-im-cli --offline`
- `bin/chat-window.ps1 -Help`
- `bin/open-chat-test.ps1 -Help`
- `bin/open-chat-test.ps1` 实际执行成功

## 真实执行结果

本次真实执行已完成：

- 启动本地服务：`http://127.0.0.1:18124`
- 创建测试会话：`c_demo_20260406203149`
- 自动打开两个聊天窗口

## 剩余风险

- 当前运行环境下 Bash 服务不可用，无法在本机对 `.sh` 脚本做语法与运行验证
- Linux/macOS 开窗脚本属于最佳努力实现，需要在对应图形环境中补回归
