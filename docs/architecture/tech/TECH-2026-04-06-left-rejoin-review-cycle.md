> Migrated from `docs/review/2026-04-06-left-rejoin-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Left Rejoin Review Cycle（2026-04-06）

## 1. 当前阶段

- `leave` 标准已经冻结，`owner transfer` 标准已经冻结。
- 当前会话成员生命周期中剩余的核心缺口，是 `left -> rejoin` 没有统一标准。
- 这个缺口已经影响到成员身份、read cursor、一致性投影和历史审计的正确性。

## 2. 问题列表

### 2.1 高风险：重入复用同一 `memberId`，会导致 runtime/projection 读游标漂移

问题表现：

- 成员 `left` 后再次通过 `add_member` 加入，之前仍复用 `cm_{conversationId}_{principalId}`。
- `conversation-runtime` 的 `upsert_read_cursor(...)` 会按同一个 `memberId` 覆盖游标。
- `projection-service` 的 `apply_member_joined(...)` 对同一个 `memberId` 只做 `or_insert`，不会重置游标。

风险：

- 新一轮 membership episode 可能继承旧 episode 的已读状态。
- runtime 视图和 projection 视图会对同一成员的 read cursor 产生不同结果。
- 商业部署下会直接影响多端同步、inbox 未读数和审计可解释性。

### 2.2 高风险：保留历史成员记录后，stale `memberId` 可能误伤当前 active member

问题表现：

- 若系统开始保留历史 episode，而 `remove_member(...)` 仍然接受旧 `memberId`，
- 则管理者可能通过旧成员记录误删当前 principal 的 active 映射。

风险：

- 旧身份可以影响当前 active member。
- 生命周期边界被打穿，历史数据和当前治理动作混在一起。

## 3. 根因分析

- 旧实现把 `memberId` 设计成 `conversationId + principalId` 的固定函数，没有 membership episode 概念。
- `add_member(...)` 只阻止“当前仍 active 的 principal 重复加入”，不会为历史 episode 分配新身份。
- `conversation-runtime` 与 `projection-service` 对相同 `memberId` 的读游标处理策略不同。
- `remove_member(...)` 之前没有验证目标 `memberId` 是否仍然是该 principal 的当前 active episode。

## 4. 本轮冻结标准

### 4.1 Membership Identity 标准

- `left` 或 `removed` 之后再次通过 `add_member` 加入，必须创建新的 membership episode。
- 新 episode 必须生成新的 `memberId`。
- 第一轮 episode 保持现有基础格式，后续 episode 在此基础上追加 episode 后缀。
- 历史 episode 的 `state / joinedAt / removedAt` 保留，不覆盖、不复活。

### 4.2 Read Cursor 标准

- read cursor 归属于 membership episode，而不是仅归属于 principal。
- 新 episode 的初始 read cursor 必须从 `readSeq = 0` 开始。
- `GET /read-cursor` 和 `POST /read-cursor` 只作用于当前 active episode。
- 历史 episode 的 cursor 保留为历史数据，不得复用到新 episode。

### 4.3 治理动作标准

- `remove_member(...)` 只能作用于当前 active member。
- stale `memberId` 必须被拒绝，不能再修改当前 active principal 的成员关系。

## 5. 本轮实现

### 5.1 领域层

- `services/conversation-runtime/src/lib.rs`
  - `add_member(...)` 改为按 `next_member_episode(...)` 生成新的 episode 身份。
  - 新增 `member_episode_id(...)`，对后续 episode 生成新的 `memberId`。
  - 新增 `ensure_current_active_member_target(...)`，拒绝 stale `memberId`。

### 5.2 覆盖测试

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_left_member_rejoin_creates_new_membership_episode`
  - `test_stale_member_id_cannot_remove_rejoined_active_member`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_left_member_rejoin_gets_new_member_identity_and_fresh_cursor`

## 6. 验证结果

已执行并通过：

- `cargo fmt --all`
- `cargo test -p conversation-runtime --test conversation_flow_test --offline`
- `cargo test -p conversation-runtime --offline`
- `cargo test -p sdkwork-im-cloud-gateway --test access_control_e2e_test --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test --workspace --offline`

## 7. 当前剩余风险

1. promote / demote 仍未标准化，目前只支持 owner handoff。
2. 特殊会话类型的成员生命周期仍未冻结。
3. 历史 membership episode 当前仅被保留，还没有独立的审计查询视图。

## 8. 下一轮计划

1. 评估是否需要独立的 `promote / demote` 标准。
2. 继续 review `agent_dialog / agent_handoff / system_channel` 的成员生命周期。
3. 评估是否需要为 membership episode 增加只读查询视图，但不改变当前写路径。

