# 09CJ - 实施计划 - IM 社交关系 / 空间治理 / 会话分层落地

## 目标

按 `150CJ` 设计，把当前偏 `conversation/member` 中心的 IM 运行时，逐步演进为 `关系域 + 空间域 + 会话域 + 治理域` 协同模型，并且与现有 Rust workspace 真实模块对齐。

## 输入

- `docs/架构/150CJ-im-social-space-conversation-ddd-design-2026-04-09.md`
- `docs/架构/05-数据模型与数据库设计.md`
- `crates/im-domain-core/src/lib.rs`
- `crates/im-domain-events/src/lib.rs`
- `crates/im-platform-contracts/src/lib.rs`
- `services/conversation-runtime/src/runtime.rs`
- `services/conversation-runtime/src/runtime/membership.rs`
- `services/conversation-runtime/src/runtime/policy.rs`
- `services/session-gateway/src/session.rs`
- `services/control-plane-api/src/lib.rs`
- `services/projection-service/src/lib.rs`

## 范围

本轮实施计划包含：

- 在 `im-domain-core` 建立 `social / space / group / governance` 领域骨架
- 明确数据库真值表、事件表、投影表与迁移目录
- 让 `conversation-runtime` 明确退回会话运行态与投影边界
- 让 `projection-service` 承担联系人、群名册、频道订阅等可重建视图
- 让 `control-plane-api` 承担审批、角色、封禁、邀请等治理写接口

本轮不包含：

- 一次性完成所有 HTTP surface
- 一次性完成完整前端交互
- 把所有历史数据在同一批次迁移完毕

## 分阶段任务

### Task 1: `im-domain-core` 新增关系域与空间域统一语言

**Files:**

- Modify: `crates/im-domain-core/src/lib.rs`
- Create: `crates/im-domain-core/src/social.rs`
- Create: `crates/im-domain-core/src/space.rs`
- Create: `crates/im-domain-core/src/group.rs`
- Create: `crates/im-domain-core/src/governance.rs`
- Test: `crates/im-domain-core/tests/model_contract_test.rs`

**Steps:**

- [ ] 先补失败测试，覆盖 `friendship` 无序唯一键、`group_member` 单人单关系、`channel_access_rule` 继承覆盖模型。
- [ ] 在 `im-domain-core` 引入 `user / actor / member / friendship / direct_chat / space / chat_group / chat_channel` 统一语言。
- [ ] 把大聚合拆成小聚合，确保 `chat_group` 不直接持有整张成员集合。
- [ ] 运行 `cargo test -p im-domain-core --offline -- --nocapture`。

### Task 2: `im-domain-events` 与 `im-platform-contracts` 收敛领域事件与仓储契约

**Files:**

- Modify: `crates/im-domain-events/src/lib.rs`
- Create: `crates/im-domain-events/src/social.rs`
- Create: `crates/im-domain-events/src/space.rs`
- Create: `crates/im-domain-events/src/group.rs`
- Create: `crates/im-domain-events/src/governance.rs`
- Modify: `crates/im-platform-contracts/src/lib.rs`
- Create: `crates/im-platform-contracts/src/social.rs`
- Create: `crates/im-platform-contracts/src/space.rs`
- Create: `crates/im-platform-contracts/src/group.rs`
- Create: `crates/im-platform-contracts/src/governance.rs`
- Test: `crates/im-platform-contracts/tests/contracts_smoke_test.rs`

**Steps:**

- [ ] 为好友、成员、邀请、审批、封禁事件定义明确 envelope。
- [ ] 定义真值仓储接口与 projection 查询接口，禁止二者混用。
- [ ] 给 `projection-service` 和 `conversation-runtime` 暴露只读查询契约。
- [ ] 运行 `cargo test -p im-domain-events --offline -- --nocapture`。
- [ ] 运行 `cargo test -p im-platform-contracts --offline -- --nocapture`。

### Task 3: 冻结数据库 schema 与迁移目录

**Files:**

- Create: `deployments/postgres/migrations/README.md`
- Create: `deployments/postgres/migrations/20260409_01_im_social_space_conversation_baseline.sql`
- Modify: `docs/架构/05-数据模型与数据库设计.md`
- Test: `services/sdkwork-im-cloud-gateway/tests/` 下新增或扩展文档契约测试

**Steps:**

- [ ] 在仓库内先固定迁移目录，不让后续 schema 到处分散。
- [ ] 先落基线表，再单独拆索引、分区、回填脚本。
- [ ] 显式标注 `truth table / event table / projection table`。
- [ ] 为 `conversation_member` 标注 `projection only`，避免回退到旧模型。

### Task 4: `conversation-runtime` 退回消息容器与投影消费边界

**Files:**

- Modify: `services/conversation-runtime/src/runtime.rs`
- Modify: `services/conversation-runtime/src/runtime/membership.rs`
- Modify: `services/conversation-runtime/src/runtime/policy.rs`
- Create: `services/conversation-runtime/src/runtime/projection_sync.rs`
- Test: `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- Test: `services/conversation-runtime/tests/conversation_flow_test.rs`

**Steps:**

- [ ] 先补结构测试，防止 `conversation-runtime` 再次持有好友/群成员真值。
- [ ] 让 `conversation.business_type + business_id` 成为唯一桥接入口。
- [ ] `conversation_member` 只能由投影同步或显式场景构建，不再直接充当群成员真值。
- [ ] 运行 `cargo test -p conversation-runtime --offline -- --nocapture`。

### Task 5: `projection-service` 承担联系人、名册、频道订阅投影

**Files:**

- Modify: `services/projection-service/src/lib.rs`
- Modify: `services/projection-service/src/model.rs`
- Modify: `services/projection-service/src/projection.rs`
- Create: `services/projection-service/src/social_projection.rs`
- Create: `services/projection-service/src/group_projection.rs`
- Test: `services/projection-service/tests/projection_snapshot_test.rs`
- Test: `services/projection-service/tests/timeline_projection_test.rs`

**Steps:**

- [ ] 明确投影输入来自 `friendship / group_member / channel_subscription / outbox_event`。
- [ ] 建立 `contact_projection`、群名册摘要、频道订阅状态等读模型。
- [ ] 确保投影删除后可以重建，不产生真值丢失。
- [ ] 运行 `cargo test -p projection-service --offline -- --nocapture`。

### Task 6: `control-plane-api` 接管治理写入口

**Files:**

- Modify: `services/control-plane-api/src/lib.rs`
- Create: `services/control-plane-api/src/social_admin.rs`
- Create: `services/control-plane-api/src/group_admin.rs`
- Create: `services/control-plane-api/src/governance_admin.rs`
- Test: `services/control-plane-api/tests/governance_loop_test.rs`
- Test: `services/control-plane-api/tests/http_smoke_test.rs`

**Steps:**

- [ ] 补好友审批、群审批、邀请、角色分配、封禁/禁言等管理接口。
- [ ] 所有治理写操作必须同时写 `audit_event` 和 `outbox_event`。
- [ ] 对“待审批”“已拒绝”“封禁中”做清晰状态机，禁止继续堆在单一成员表状态里。
- [ ] 运行 `cargo test -p control-plane-api --offline -- --nocapture`。

### Task 7: `session-gateway` 与运行态实时消费投影

**Files:**

- Modify: `services/session-gateway/src/session.rs`
- Modify: `services/session-gateway/src/presence.rs`
- Modify: `services/session-gateway/src/realtime.rs`
- Test: `services/session-gateway/tests/realtime_runtime_test.rs`
- Test: `services/session-gateway/tests/http_smoke_test.rs`

**Steps:**

- [ ] 让实时路由与 fanout 消费 `conversation_member` 投影，而不是自行推导组织关系。
- [ ] 保证 WebSocket / presence / unread 流程仍然只依赖会话运行态。
- [ ] 保持 `session-gateway` 不直接持有好友、群角色真值写模型。
- [ ] 运行 `cargo test -p session-gateway --offline -- --nocapture`。

### Task 8: 文档、Step、Review 回写闭环

**Files:**

- Modify: `docs/架构/README.md`
- Modify: `docs/step/README.md`
- Modify: `docs/prompts/反复执行Step指令.md`
- Create: `docs/review/continuous-optimization-im-social-space-conversation-ddd-2026-04-09.md`

**Steps:**

- [ ] 每闭合一个真实缺口，都要回写 `docs/架构`、`docs/step`、`docs/review`。
- [ ] 若架构边界发生变化，先更新 `150CJ` 再声称实现完成。
- [ ] 若只是局部代码完成，不得把 Step 标记为整体闭环。

## 验证

- `cargo test -p im-domain-core --offline -- --nocapture`
- `cargo test -p im-domain-events --offline -- --nocapture`
- `cargo test -p im-platform-contracts --offline -- --nocapture`
- `cargo test -p conversation-runtime --offline -- --nocapture`
- `cargo test -p projection-service --offline -- --nocapture`
- `cargo test -p control-plane-api --offline -- --nocapture`
- `cargo test -p session-gateway --offline -- --nocapture`

## 退出标准

- `im-domain-core` 已经拥有明确的社交关系、空间治理、群与频道领域模型。
- 数据库 baseline schema 已明确区分真值表、事件表、投影表。
- `conversation-runtime` 不再被当作好友/群治理真值中心。
- `projection-service` 已接手联系人、名册、频道订阅等可重建视图。
- `control-plane-api` 已承接治理写入口。
- Step 与架构文档回写一致，不存在“代码已经改了，但文档仍停留旧模型”的漂移。

## 下一步

在以上基线落地后，再进入更细粒度迭代：

1. 好友申请与私聊自动建链
2. 群审批与邀请链路
3. 频道 ACL 继承与覆盖
4. 超大群分区与消息冷热分层

## 2026-04-09 Addendum

结合 `151CJ` 对标结论，实施优先级再补三点：

1. 在线程模型落地前，不要把“频道内子讨论”继续塞进普通消息回复字段。
2. 在外部协作模型落地前，不要把 shared/external 协作直接混进普通 `group_member`。
3. 在 `conversation-runtime` 完成降责前，不要继续把好友、群治理真值堆进会话运行时。
