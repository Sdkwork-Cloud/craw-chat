# Step 02 Workspace / Crate 重构复盘 - 2026-04-07

## 1. 复盘范围

本复盘覆盖 `docs/step/02-workspace与crate骨架重构.md` 在当前仓库中的实际落地结果。

目标不是重复罗列所有改动，而是明确回答：

- Step 02 是否真的把 workspace / service-impl 边界拉回正确方向
- 后续 Step 03 是否已经具备稳定入口

## 2. 已完成的工程收口

### 2.1 顶层 facade 收敛

以下关键 facade 已处于 Step 02 可接受状态：

| 文件 | 当前行数 | 状态 |
| --- | --- | --- |
| `services/session-gateway/src/lib.rs` | `639` | 通过 |
| `services/conversation-runtime/src/lib.rs` | `3` | 通过 |
| `services/sdkwork-im-cloud-gateway/src/lib.rs` | `3` | 通过 |
| `services/projection-service/src/lib.rs` | `965` | 通过，需重点盯防 |
| `adapters/local-disk/src/lib.rs` | `24` | 通过 |
| `tools/chat-cli/src/lib.rs` | `848` | 通过 |

### 2.2 能力目录化

Step 02 已把核心能力从大文件中下沉为独立模块：

- `session-gateway`
  - `cluster/`
  - `realtime/`
  - `presence.rs`
- `conversation-runtime`
  - `runtime/http.rs`
  - `runtime/policy.rs`
  - `runtime/recovery.rs`
  - `runtime/support.rs`
- `sdkwork-im-server`
  - `node/access.rs`
  - `node/effects.rs`
  - `node/platform.rs`
  - `node/build.rs`
  - `node/runtime_dir.rs`
  - `node/message.rs`
  - `node/membership.rs`
  - `node/projection.rs`
  - `node/media.rs`
  - `node/session.rs`
  - `node/stream.rs`
  - `node/rtc.rs`
- `projection-service`
  - `http.rs`
  - `model.rs`
  - `projection.rs`
- `local-disk`
  - `journal.rs`
  - `shared.rs`
  - `realtime.rs`
  - `state.rs`
  - `ops.rs`
- `chat-cli`
  - `command.rs`

## 3. 本轮最后一个阻塞点与处理

本轮真正的尾项是 `tools/chat-cli/src/lib.rs`。

阻塞前的真实问题：

- `lib.rs` 同时承载命令模型、参数解析、usage/help、本地 env 解析、HTTP / WebSocket / interactive chat。
- 行数超标，且持续违反 Step 02 的 facade 原则。

处理后结果：

- `command.rs` 承接命令模型、解析、配置读取和 usage surface。
- `lib.rs` 只保留运行时执行路径与外部公开 API。
- `chat-cli` 结构测试、E2E、包级测试全部 fresh 通过。

## 4. 复盘结论

### 4.1 Step 02 已兑现的能力

- `services/*` 已回到装配层 / facade 角色。
- 能力按目录和模块聚合，不再被迫继续堆回旧 `lib.rs`。
- 超大生产 Rust 文件已被清零到治理红线之下。

### 4.2 Step 02 尚未承担的能力

- `ccp-*` crate 族
- `contract-*` 职责拆分
- `tenant / sender / actor` 权威字段的协议级冻结

这些属于 `Step 03`，不应再回灌到 Step 02。

### 4.3 需要盯防的残余风险

- `services/projection-service/src/lib.rs` 当前 `965` 行，虽然合规，但已经接近上限。
- Step 03 协议与契约工作不得再落回 `projection-service` 或其他 Step 02 刚收口的 facade 文件。

## 5. 是否允许进入下一步

- 结论：`允许进入 Step 03`
- 前提：严格把 Step 03 的新增逻辑放入新的 `ccp-* / contract-*` 落点，不回流旧服务 facade。
