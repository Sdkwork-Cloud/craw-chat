> Migrated from `docs/review/step-05-cp05-2-projection-auth-context-entrypoints-架构兑现-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 CP05-2 Projection Auth Context Entrypoints 架构兑现 - 2026-04-07

## 1. 对应架构能力

- `docs/架构/09-实施计划.md`
  - `Wave B / Step 05 / CP05-2`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
  - Step 05 projection/read-model authority owner 收口
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
  - projection/read-model query timing authority owner
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
  - projection/read-model auth-context boundary
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
  - projection-service crate/interface seam

## 2. 本轮已兑现

- `projection-service` 现在自己持有 projection/read-model auth-context access boundary。
- `projection-service/http` 不再直接维护 device/query authority 校验。
- `sdkwork-im-server` 的 projection/session consumer 已切到 projection-service 的 auth-context seam。
- `ProjectionAccessError` 成为 projection-service authority owner 下沉后的统一错误出口。

## 3. 本轮未兑现

- `CP05-2` 还未整体闭环。
- downstream Step 05 authority consumer 仍未全部切到统一 owner。
- 已知剩余热点：
  - `services/sdkwork-im-cloud-gateway/src/node/effects.rs`

## 4. 是否偏离架构

- 无新增架构偏离。
- 本轮延续了前四个 `CP05-2` 增量的同一策略：
  - 不做大一统重构
  - 只把下一块真实 authority owner 收口到正确 owner

## 5. 回写决议

- 需要按 `97` 回写以下架构文档：
  - `docs/架构/09-实施计划.md`
  - `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
  - `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
  - `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
  - `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`

## 6. 证据

- 代码
  - `services/projection-service/src/access.rs`
  - `services/projection-service/src/http.rs`
  - `services/sdkwork-im-cloud-gateway/src/node/projection.rs`
  - `services/sdkwork-im-cloud-gateway/src/node/session.rs`
  - `services/sdkwork-im-cloud-gateway/src/node.rs`
- 测试
  - `services/projection-service/tests/lib_structure_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- fresh verification
  - `cargo test -p projection-service --offline`
  - `cargo test -p sdkwork-im-cloud-gateway --offline`
  - `rustfmt --edition 2024 --check ...`

