> Migrated from `docs/review/step-05-cp05-4-projection-active-principal-auth-context-owner-架构兑现-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 / CP05-4 projection active-principal auth-context owner 架构兑现 - 2026-04-07

## 1. 对应架构能力

- `09 / Step 05 / CP05-4` projection / notification 剩余连接点
- `130` active principal mapping 不应长期停留在 edge 侧 member roster read
- `136` message / membership / handoff / stream 的 conversation recipient 解析应先进入 projection-owned conversation seam
- `139` auth-context 下的 conversation principal scope capture 应由单一 owner 维护
- `147` 该 seam 应映射到 `projection-service::access`

## 2. 本轮架构兑现

- `projection-service::access` 新增 `active_conversation_principal_ids_from_auth_context(...)`
- `sdkwork-im-server/effects.rs` 的 `conversation_member_principal_ids_from_auth_context(...)` 统一消费这条 seam
- conversation-scoped notification / realtime side-effect 不再直接依赖 runtime member roster 解析 recipient principals

## 3. 当前决议

- 认定本轮为 `CP05-4` 的有效架构兑现增量。
- 暂不判定 `CP05-4`、`Step 05`、`91 / 95 / 97`、`Wave B / 93` 完成。

## 4. 回写对象

- `docs/架构/09-实施计划.md`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`

