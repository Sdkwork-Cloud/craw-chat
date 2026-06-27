> Migrated from `docs/架构/125-local-chat-window-launcher-standard-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 125 本地多窗口聊天验证标准（2026-04-06）

## 目标

在不引入额外后端测试接口的前提下，为 `sdkwork-im` 提供可直接落地的本地聊天验证闭环：

- 启动本地服务
- 创建测试会话
- 加入第二个成员
- 自动打开两个独立终端窗口
- 每个窗口内可以直接输入消息并接收实时消息

## 标准约束

### 1. 仍然只使用现有服务接口

- 不新增仅用于本地联调的后端 API
- 聊天窗口通过现有 HTTP + WebSocket 能力完成收发
- 认证继续使用签名 Bearer Token 或显式 Bearer Token

### 2. 交互式聊天命令标准

新增 `chat-session` 子命令，能力如下：

- 建立 WebSocket 连接
- 自动执行 `subscriptions.sync`
- 自动对 `event.window` 执行 `events.ack`
- 终端输入一行即发送一条消息
- `/quit` 或 `/exit` 退出
- `/help` 输出帮助

### 3. 默认地址解析标准

本地脚本与 CLI 在未显式传入 `baseUrl` 时，必须优先读取 Topology v2 运行时配置：

- `.runtime/standalone.split-services.development/config/standalone.split-services.development.env`
- 键：`SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL`
- 回退：`SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND`
- 默认回退：`http://127.0.0.1:18079`

并将其中的 bind 地址转换为本地访问地址：

- `0.0.0.0`、`::`、`[::]` 统一映射为 `127.0.0.1`

避免脚本默认端口与当前本地配置不一致。

### 4. 发送端标签标准

由于当前公开实时事件与 timeline 投影中不包含完整 sender 结构，聊天验证脚本默认通过 `messagePrefix` 将发送端标签拼接到消息摘要中，例如：

- `[owner] hello`
- `[guest] hi`

这样可以在多窗口人工验证时直接识别消息来源。

### 5. `/bin` 统一入口标准

新增以下入口：

- `bin/chat-window.ps1`
- `bin/chat-window.cmd`
- `bin/chat-window.sh`
- `bin/chat-window`
- `bin/open-chat-test.ps1`
- `bin/open-chat-test.cmd`
- `bin/open-chat-test.sh`
- `bin/open-chat-test`

其中：

- `chat-window` 负责单个交互式聊天窗口
- `open-chat-test` 负责启动服务、建会话、加成员、开两个窗口

## 验证标准

### Rust 侧

- `cargo fmt --all`
- `cargo test -p sdkwork-im-cli --offline`

### 脚本侧

- Windows `-Help` 正常输出
- `open-chat-test.ps1` 可实际拉起本地服务并打开两个窗口

## 当前实现说明

本次落地已经实现：

- `chat-session` 交互命令
- 本地配置端口自动解析
- Windows 自动开窗验证闭环
- Linux/macOS 终端探测型最佳努力脚本

## 后续增强

- 在实时事件与 timeline 中补充 sender 结构
- 在 Linux/macOS 补真实环境下的脚本回归验证
- 提供可选的多会话批量开窗和预置测试消息能力

