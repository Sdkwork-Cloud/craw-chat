> Migrated from `docs/review/step-05-cp05-4-projection-realtime-fanout-auth-context-owner-架构兑现-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 / CP05-4 projection realtime fanout auth-context owner 架构兑现 - 2026-04-07

## 1. 对应架构能力

- `docs/架构/09-实施计划.md`
  - `Wave B / Step 05 / CP05-4`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
  - realtime side-effect 不应长期停留在 edge 侧重复抓取 tenant scope 再访问 projection fanout owner
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
  - conversation / stream / handoff / membership realtime publish 应先进入 projection-owned auth-context seam，再完成 client route fanout
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
  - auth context 下的 tenant/client route fanout scope capture 应由单一 owner seam 维护
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
  - 该 seam 应映射到 `projection-service::access`，consumer 位于 `sdkwork-im-server/effects.rs`

## 2. 本轮架构兑现

- `projection-service` 新增 `realtime_fanout_targets_from_auth_context(...)`。
- `sdkwork-im-server/effects.rs` 现在通过这条 seam 统一完成 realtime publish helper 的 auth-context capture。
- owner / consumer 边界变为：
  - owner：`services/projection-service/src/access.rs`
  - consumer：`services/sdkwork-im-cloud-gateway/src/node/effects.rs`

## 3. 当前决议

- 认定本轮为 `CP05-4` 的有效架构兑现增量。
- 不认定 `CP05-4`、`Step 05`、`91 / 95 / 97`、`Wave B / 93` 已完成。

## 4. 回写对象

- `docs/架构/09-实施计划.md`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`

