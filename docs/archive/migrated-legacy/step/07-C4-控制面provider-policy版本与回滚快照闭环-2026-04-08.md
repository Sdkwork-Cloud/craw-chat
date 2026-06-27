# Step 07-C4: 控制面 provider-policy 版本与回滚快照闭环

## 闭环定位

- 当前 step：`Step 07`
- 当前波次：`07-C4 / CP07-4B`
- 目标：在 `07-C3` 的 provider policy 写接口基础上，补齐版本历史、回滚快照、ops 全量刷新和审计闭环。

## 本轮落地

- `RuntimeProviderRegistry` 输出 `ProviderPolicyHistory`，每次写入生成新版本快照。
- 新增 `GET /backend/v3/api/control/provider-policies`，用于查询 provider policy 版本历史。
- 新增 `POST /backend/v3/api/control/provider-policies/rollback`，按 `targetVersion` 执行回滚。
- 回滚快照新增 `rollbackFromVersion` 字段，保留“从哪个版本恢复”的审计语义。
- control-plane 在回滚后调用 `replace_provider_binding_snapshots`，全量替换 ops 中的 provider binding 快照，避免租户 override 被删除后残留脏数据。
- 新增审计动作 `control.provider_policy_rolled_back`。

## 对外契约

- 历史接口：`GET /backend/v3/api/control/provider-policies`
- 回滚接口：`POST /backend/v3/api/control/provider-policies/rollback`
- 历史响应关键字段：
  - `currentVersion`
  - `items`
  - `rollbackFromVersion`
- 回滚请求关键字段：
  - `targetVersion`

## 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`

## 文档回写

- `docs/step/07-C4-控制面provider-policy版本与回滚快照闭环-2026-04-08.md`
- `docs/架构/09G-实施计划-provider-policy版本与回滚补充-2026-04-08.md`
- `docs/架构/150G-control-plane-provider-policy版本与回滚设计-2026-04-08.md`
- `docs/review/continuous-optimization-control-plane-provider-policy-version-and-rollback-2026-04-08.md`

## 下一轮动作

- 若继续留在 `Step 07`，优先补 provider policy 差异查询或变更预览。
- 若 `Step 07` 基线满足验收，则按总 step 流程进入下一闭环。
