# 124 本地聊天 CLI 多终端验证标准

## 1. 目标

为 `sdkwork-im` 提供一个可直接连接运行中服务的命令行验证工具，使开发者可以在多个终端窗口中模拟不同用户，验证标准 IM 对话与实时事件链路，而不需要手工拼接 HTTP 请求或 JWT。

## 2. 适用范围

本标准适用于：

1. `sdkwork-im-server` 本地验证
2. 面向公开入口的服务联调
3. 多身份、多客户端路由、多会话的人工回归和冒烟检查

本标准不引入新的服务端 API，也不替代正式 SDK。

## 3. 标准要求

### 3.1 命令集

CLI 至少必须支持以下命令：

1. `health`
2. `token`
3. `create-conversation`
4. `add-member`
5. `members`
6. `send-message`
7. `timeline`
8. `watch`

### 3.2 认证要求

1. CLI 必须优先兼容公开服务使用的签名 Bearer Token。
2. CLI 必须支持根据 `tenant/user/session/device` 自动生成本地 Bearer Token。
3. CLI 必须支持显式传入现成 token。
4. secret 解析优先级必须允许从运行时配置文件自动发现，减少手工输入。

### 3.3 多终端可用性

CLI 必须允许不同终端通过以下身份参数自由切换：

1. `tenantId`
2. `userId`
3. `actorKind`
4. `sessionId`
5. `clientRouteId`

默认值可以存在，但不得阻止显式覆盖。

### 3.4 输出标准

1. HTTP 命令默认输出 JSON。
2. WebSocket 监听必须输出逐帧 JSON，便于人工观察和日志采集。
3. 非 2xx HTTP 响应必须带出状态码和原始错误体。

### 3.5 实时监听标准

1. `watch` 必须复用 `/im/v3/api/realtime/ws`。
2. `watch` 必须自动发送 `subscriptions.sync`。
3. 收到 `event.window` 后应自动 ack，避免窗口无限堆积。
4. 必须支持按事件数量退出和按空闲超时退出，便于自动化测试与脚本执行。

## 4. 推荐使用方式

### 4.1 终端 A 创建会话并加成员

```powershell
powershell -ExecutionPolicy Bypass -File bin/chat-cli.ps1 -- `
  --user-id 1 `
  --device-id d_owner `
  --session-id s_owner `
  create-conversation --conversation-id c_demo --conversation-type group

powershell -ExecutionPolicy Bypass -File bin/chat-cli.ps1 -- `
  --user-id 1 `
  --device-id d_owner `
  --session-id s_owner `
  add-member --conversation-id c_demo --principal-id 2 --principal-kind user --role member
```

### 4.2 终端 B 监听实时事件

```powershell
powershell -ExecutionPolicy Bypass -File bin/chat-cli.ps1 -- `
  --user-id 2 `
  --device-id d_guest `
  --session-id s_guest `
  watch --conversation-id c_demo --event-type message.posted
```

### 4.3 终端 A 发送消息

```powershell
powershell -ExecutionPolicy Bypass -File bin/chat-cli.ps1 -- `
  --user-id 1 `
  --device-id d_owner `
  --session-id s_owner `
  send-message --conversation-id c_demo --summary "hello" --text "hello"
```

### 4.4 终端 B 查看时间线

```powershell
powershell -ExecutionPolicy Bypass -File bin/chat-cli.ps1 -- `
  --user-id 2 `
  --device-id d_guest `
  --session-id s_guest `
  timeline --conversation-id c_demo
```

### 4.5 不同操作系统推荐入口

1. Windows PowerShell: `powershell -ExecutionPolicy Bypass -File bin/chat-cli.ps1 -- <args>`
2. Windows CMD: `bin\chat-cli.cmd <args>`
3. Linux/macOS Bash: `bash bin/chat-cli.sh <args>`
4. Linux/macOS 直接命令: `./bin/chat-cli <args>`

## 5. 实现约束

1. CLI 不得依赖 Trusted Headers 才能工作。
2. CLI 不得要求开发者手工构造 JWT payload 和签名。
3. CLI 不得绕过服务端现有授权和成员校验逻辑。
4. CLI 必须通过端到端测试验证“双身份 + 实时事件”闭环。
