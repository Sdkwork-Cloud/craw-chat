> Migrated from `docs/review/2026-04-06-local-chat-cli-multi-terminal-validation-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 本地聊天 CLI 多终端验证 Review Cycle

## 本轮交付

1. 新增工作区包 `tools/chat-cli`。
2. 新增跨平台命令行入口：
   - `bin/chat-cli.ps1`
   - `bin/chat-cli.cmd`
   - `bin/chat-cli.sh`
   - `bin/chat-cli`
3. 保留兼容入口：
   - `bin/chat-cli-local.ps1`
   - `bin/chat-cli-local.cmd`
   - `bin/chat-cli-local.sh`
4. CLI 支持：
   - 解析本地身份参数
   - 自动生成签名 Bearer Token
   - HTTP 命令执行
   - WebSocket 实时订阅与自动 ack
5. 新增 e2e 测试，覆盖双身份建会话、加成员、发消息、拉时间线、实时观察。

## 关键检查项

### 1. 是否新增了服务端协议

结论：没有。CLI 完全复用现有 `/im/v3/api/chat/conversations*` 和 `/im/v3/api/realtime/ws`。

### 2. 是否能用于多终端人工验证

结论：可以。

验证路径：

1. 终端 A 使用 `1` 创建会话并加成员。
2. 终端 B 使用 `2` 执行 `watch`。
3. 终端 A 执行 `send-message`。
4. 终端 B 接收 `event.window` 推送。
5. 终端 B 执行 `timeline` 可读到同一条消息。

### 3. 认证是否与公开服务一致

结论：一致。

CLI 默认优先顺序：

1. `--bearer-token`
2. `--public-bearer-secret`
3. 环境变量 `SDKWORK_IM_PUBLIC_BEARER_HS256_SECRET`
4. `.runtime/standalone.split-services.development/config/standalone.split-services.development.env`

### 4. 是否满足离线构建要求

结论：满足。

实现中避免新增 `clap`、`reqwest` 等当前锁文件未使用的组件，改为手写参数解析，HTTP 使用 `hyper/hyper-util`，WebSocket 使用已存在的 `tokio-tungstenite`。

## 残余风险

1. `watch --exit-after-events` 当前按 `event.window` 次数退出，如果设备存在历史未确认事件，可能先消费 catchup 窗口。
2. CLI 当前主要面向本地/测试环境，未提供复杂输出格式切换和脚本化批量编排能力。
3. 目前没有把 CLI 集成进现有 `retired-lifecycle-install` 生命周期脚本，只通过独立入口脚本运行。

## 下一步建议

1. 如需更强的对话脚本化能力，可增加 `scenario` 子命令。
2. 如需更稳定的人工观察体验，可给 `watch` 增加 `--only-push`、`--pretty` 等选项。
3. 如需纳入本地部署体验，可在 README 和部署文档里补充典型双终端操作手册。

