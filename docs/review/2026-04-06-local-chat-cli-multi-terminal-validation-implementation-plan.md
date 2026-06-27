# 本地聊天 CLI 多终端验证实施计划

## 背景

当前 `sdkwork-im-server` 已经具备公开 HTTP/WebSocket 接口，但缺少一个面向开发联调的命令行入口。用户需要在多个终端窗口中分别扮演不同身份，直接对运行中的服务做建会话、加成员、发消息、拉时间线和实时监听验证。

## 目标

1. 新增 Rust CLI 包，复用现有服务 API，不新增协议。
2. 支持基于 `tenant/user/session/device` 自动生成本地可用的签名 Bearer Token。
3. 支持多终端对话验证的最小命令集：
   - `health`
   - `token`
   - `create-conversation`
   - `add-member`
   - `members`
   - `send-message`
   - `timeline`
   - `watch`
4. 提供 Windows/Linux/macOS 统一入口脚本。
5. 用端到端测试冻结“两终端对话 + 实时事件观察”契约。

## 设计约束

1. 不引入新的服务端接口。
2. 默认走 `build_public_app()` 路径使用签名 Bearer，避免 CLI 依赖 Trusted Headers。
3. 在离线构建环境下优先复用锁文件中已存在的依赖。
4. 输出以 JSON 为主，便于人工观察和自动测试。

## 实施步骤

1. 新建 `tools/chat-cli` 工作区包。
2. 先写失败的 CLI e2e 契约测试，覆盖建会话、加成员、发消息、时间线、WebSocket 监听。
3. 实现手写参数解析，避免引入未锁定的命令行库。
4. 实现 HTTP 调用层和 WebSocket 监听层。
5. 实现本地签名 Token 生成和配置文件 secret 自动发现。
6. 新增 `bin/chat-cli.ps1/.cmd/.sh` 和 `bin/chat-cli` 统一跨平台入口，保留 `chat-cli-local*` 兼容包装。
7. 执行 `cargo test -p sdkwork-im-cli --offline`、`cargo fmt --all`，并做一次真实 CLI 烟雾验证。
