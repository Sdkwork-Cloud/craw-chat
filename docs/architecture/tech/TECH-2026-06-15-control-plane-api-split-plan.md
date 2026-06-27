> Migrated from `docs/superpowers/plans/2026-06-15-control-plane-api-split-plan.md` on 2026-06-24.
> Owner: SDKWork maintainers

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 control-plane-api 拆分为社交服务和治理服务，解决 16K 行单文件问题

**Architecture:** 
- 创建新的 `social-service`，承载所有社交相关功能（好友、关系、屏蔽、单聊、外部连接、共享频道同步）
- 将 control-plane-api 重命名为 `governance-service`，仅保留协议治理和提供者管理功能
- 更新所有依赖和配置

**Tech Stack:** Rust, axum, tokio

---

## 当前状态分析

### control-plane-api 路由分析

**社交相关路由**（应迁移到 social-service）：
```
/backend/v3/api/control/social/friend_requests
/backend/v3/api/control/social/friendships
/backend/v3/api/control/social/user_blocks
/backend/v3/api/control/social/direct_chats
/backend/v3/api/control/social/external_connections
/backend/v3/api/control/social/external_member_links
/backend/v3/api/control/social/shared_channel_policies
/backend/v3/api/control/social/runtime/*
```

**治理相关路由**（保留在 governance-service）：
```
/backend/v3/api/control/protocol_registry
/backend/v3/api/control/protocol_governance
/backend/v3/api/control/provider_registry
/backend/v3/api/control/provider_bindings
/backend/v3/api/control/provider_policies
/backend/v3/api/control/nodes/*
```

### 代码量统计
- 总行数：16,264
- 社交相关：3,811 (23.4%)
- 治理相关：~2,000 (12.3%)
- 共享工具：~1,500 (9.2%)
- OpenAPI 定义：~2,000 (12.3%)
- 其他：~7,000 (42.8%)

---

## 文件结构

### 新建文件

```
services/social-service/
├── Cargo.toml
├── src/
│   ├── lib.rs                    # 主入口，社交运行时
│   ├── friendship.rs             # 好友请求/关系
│   ├── block.rs                  # 用户屏蔽
│   ├── direct_chat.rs            # 单聊会话
│   ├── external.rs               # 外部连接/成员链接
│   ├── shared_channel.rs         # 共享频道同步
│   ├── runtime.rs                # 社交运行时状态管理
│   ├── http.rs                   # HTTP 路由和处理器
│   └── openapi.rs                # OpenAPI 文档
└── specs/
    └── component.spec.json
```

### 修改文件

```
services/control-plane-api/       # 重命名为 governance-service
├── Cargo.toml                    # 更新依赖
├── src/
│   └── lib.rs                    # 精简为仅治理功能
└── specs/
    └── component.spec.json
```

### 更新依赖的文件

```
services/local-minimal-node/src/node/social.rs  # 更新导入
Cargo.toml                                       # 更新 workspace 成员
```

---

## 任务分解

### Task 1: 创建 social-service 基础结构

**Files:**
- Create: `services/social-service/Cargo.toml`
- Create: `services/social-service/src/lib.rs`
- Create: `services/social-service/specs/component.spec.json`

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "social-service"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
im-domain-core = { path = "../../crates/im-domain-core" }
im-domain-events = { path = "../../crates/im-domain-events" }
im-platform-contracts = { path = "../../crates/im-platform-contracts" }
im-time = { path = "../../crates/im-time" }
sdkwork-im-api-registry = { path = "../../sdks/sdkwork-im-sdk/sdkwork-im-api-registry" }
sdkwork-im-openapi = { path = "../../sdks/sdkwork-im-sdk/sdkwork-im-openapi" }
im-app-context = { path = "../../crates/im-app-context" }
sha2 = "0.10"
```

- [ ] **Step 2: 创建基础 lib.rs**

```rust
//! Social Service - 好友关系、用户屏蔽、单聊、外部连接、共享频道同步

mod friendship;
mod block;
mod direct_chat;
mod external;
mod shared_channel;
mod runtime;
mod http;
mod openapi;

pub use runtime::SocialRuntime;
pub use http::{build_app, build_public_app};
```

- [ ] **Step 3: 创建 component.spec.json**

- [ ] **Step 4: 更新 Cargo.toml workspace 成员**

```toml
[workspace]
members = [
    # ... existing members ...
    "services/social-service",
]
```

- [ ] **Step 5: 验证编译**

```bash
cargo check -p social-service
```

---

### Task 2: 迁移社交运行时状态

**Files:**
- Create: `services/social-service/src/runtime.rs`
- Modify: `services/control-plane-api/src/lib.rs`

- [ ] **Step 1: 从 control-plane-api 提取 SocialControlState**

从 control-plane-api 中提取以下类型和函数：
- `SocialControlState`
- `SocialControlRuntime`
- `SocialCommittedEvent`
- 状态持久化逻辑（内存和文件）

- [ ] **Step 2: 创建 social-service/src/runtime.rs**

包含：
- `SocialRuntime` 结构体
- 状态加载/保存逻辑
- 事件提交逻辑
- 内存和文件存储后端

- [ ] **Step 3: 验证编译**

```bash
cargo check -p social-service
```

---

### Task 3: 迁移好友功能

**Files:**
- Create: `services/social-service/src/friendship.rs`
- Modify: `services/social-service/src/runtime.rs`

- [ ] **Step 1: 提取好友相关函数**

从 control-plane-api 提取：
- `submit_friend_request`
- `accept_friend_request`
- `decline_friend_request`
- `cancel_friend_request`
- `activate_friendship`
- `remove_friendship`
- `friend_request_snapshot`
- `friendship_snapshot`
- `list_friend_requests`

- [ ] **Step 2: 创建 friendship.rs**

包含：
- `FriendshipService` trait 或 impl
- 所有好友相关的业务逻辑
- 事件发布逻辑

- [ ] **Step 3: 验证编译和测试**

```bash
cargo test -p social-service
```

---

### Task 4: 迁移屏蔽功能

**Files:**
- Create: `services/social-service/src/block.rs`

- [ ] **Step 1: 提取屏蔽相关函数**

从 control-plane-api 提取：
- `block_user`
- `user_block_snapshot`

- [ ] **Step 2: 创建 block.rs**

- [ ] **Step 3: 验证编译和测试**

---

### Task 5: 迁移单聊功能

**Files:**
- Create: `services/social-service/src/direct_chat.rs`

- [ ] **Step 1: 提取单聊相关函数**

从 control-plane-api 提取：
- `bind_direct_chat`
- `direct_chat_snapshot`

- [ ] **Step 2: 创建 direct_chat.rs**

- [ ] **Step 3: 验证编译和测试**

---

### Task 6: 迁移外部连接功能

**Files:**
- Create: `services/social-service/src/external.rs`

- [ ] **Step 1: 提取外部连接相关函数**

从 control-plane-api 提取：
- `establish_external_connection`
- `external_connection_snapshot`
- `bind_external_member_link`
- `external_member_link_snapshot`

- [ ] **Step 2: 创建 external.rs**

- [ ] **Step 3: 验证编译和测试**

---

### Task 7: 迁移共享频道同步功能

**Files:**
- Create: `services/social-service/src/shared_channel.rs`

- [ ] **Step 1: 提取共享频道相关函数**

从 control-plane-api 提取：
- `apply_shared_channel_policy`
- `shared_channel_policy_snapshot`
- 所有 dead letter 和 sync 相关函数
- 共享频道同步触发器逻辑

- [ ] **Step 2: 创建 shared_channel.rs**

- [ ] **Step 3: 验证编译和测试**

---

### Task 8: 创建社交 HTTP 路由

**Files:**
- Create: `services/social-service/src/http.rs`
- Create: `services/social-service/src/openapi.rs`

- [ ] **Step 1: 创建 HTTP 路由**

```rust
pub fn build_app(state: SocialRuntime) -> Router {
    Router::new()
        .route("/backend/v3/api/social/friend_requests", get(list_friend_requests).post(submit_friend_request))
        .route("/backend/v3/api/social/friend_requests/{request_id}", get(friend_request_snapshot))
        // ... 其他社交路由
        .with_state(state)
}
```

- [ ] **Step 2: 创建 OpenAPI 文档**

从 control-plane-api 提取社交相关的 OpenAPI 定义

- [ ] **Step 3: 验证编译和测试**

---

### Task 9: 精简 control-plane-api

**Files:**
- Modify: `services/control-plane-api/src/lib.rs`
- Modify: `services/control-plane-api/Cargo.toml`

- [ ] **Step 1: 删除已迁移的社交代码**

删除：
- 所有社交相关的函数
- 社交运行时状态
- 社交 HTTP 路由
- 社交 OpenAPI 定义

- [ ] **Step 2: 更新依赖**

如果 social-service 作为依赖被 control-plane-api 使用，更新 Cargo.toml

- [ ] **Step 3: 验证编译**

```bash
cargo check -p control-plane-api
```

---

### Task 10: 更新 local-minimal-node

**Files:**
- Modify: `services/local-minimal-node/src/node/social.rs`
- Modify: `services/local-minimal-node/Cargo.toml`

- [ ] **Step 1: 更新导入**

将 social 相关导入从 control-plane-api 改为 social-service

- [ ] **Step 2: 更新依赖**

```toml
[dependencies]
social-service = { path = "../social-service" }
```

- [ ] **Step 3: 验证编译**

```bash
cargo check -p local-minimal-node
```

---

### Task 11: 重命名 control-plane-api 为 governance-service

**Files:**
- Rename: `services/control-plane-api/` → `services/governance-service/`
- Modify: `services/governance-service/Cargo.toml` (更新 package name)
- Modify: `Cargo.toml` (更新 workspace 成员)
- Modify: 所有引用 control-plane-api 的文件

- [ ] **Step 1: 重命名目录**

```bash
mv services/control-plane-api services/governance-service
```

- [ ] **Step 2: 更新 Cargo.toml**

```toml
[package]
name = "governance-service"
```

- [ ] **Step 3: 更新 workspace 成员**

- [ ] **Step 4: 更新所有导入**

搜索并替换所有 `control_plane_api` 引用为 `governance_service`

- [ ] **Step 5: 验证编译**

```bash
cargo check -p governance-service
cargo check -p local-minimal-node
```

---

### Task 12: 运行测试验证

- [ ] **Step 1: 运行 social-service 测试**

```bash
cargo test -p social-service
```

- [ ] **Step 2: 运行 governance-service 测试**

```bash
cargo test -p governance-service
```

- [ ] **Step 3: 运行 local-minimal-node 测试**

```bash
cargo test -p local-minimal-node
```

- [ ] **Step 4: 运行 workspace 测试**

```bash
cargo test --workspace
```

- [ ] **Step 5: 运行 clippy 检查**

```bash
cargo clippy --workspace --tests -- -D warnings
```

---

## 预期结果

### 拆分后的 services 目录

```
services/
├── audit-service/           # 审计服务（不变）
├── automation-service/      # 自动化服务（不变）
├── governance-service/      # 治理服务（原 control-plane-api）
├── conversation-runtime/    # 会话运行时（不变）
├── local-minimal-node/      # 本地统一节点（不变）
├── media-service/           # 媒体服务（不变）
├── notification-service/    # 通知服务（不变）
├── ops-service/             # 运维服务（不变）
├── projection-service/      # 投影服务（不变）
├── session-gateway/         # 会话网关（不变）
├── social-service/          # 社交服务（新建）
├── streaming-service/       # 流式服务（不变）
└── web-gateway/             # Web 网关（不变）
```

### 代码量对比

| 服务 | 拆分前 | 拆分后 |
|------|--------|--------|
| control-plane-api | 16,264 | - |
| governance-service | - | ~4,000 |
| social-service | - | ~5,000 |

### API 路径变更

**旧路径**：
```
/backend/v3/api/control/social/friend_requests
/backend/v3/api/control/protocol_registry
```

**新路径**：
```
/backend/v3/api/social/friend_requests      # social-service
/backend/v3/api/control/protocol_registry   # governance-service
```

---

## 风险和注意事项

1. **API 兼容性**：社交 API 路径从 `/control/social/` 改为 `/social/`，需要更新前端
2. **状态迁移**：社交状态文件（social-state.json）需要迁移或保持兼容
3. **测试覆盖**：确保所有社交功能的测试都迁移到 social-service
4. **依赖关系**：检查是否有其他服务依赖 control-plane-api 的社交功能

---

## 回滚方案

如果出现问题：
1. 恢复 control-plane-api 原始代码
2. 删除 social-service
3. 恢复所有导入和依赖

