> Migrated from `docs/架构/09G-实施计划-provider-policy版本与回滚补充-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09G 实施计划: provider-policy 版本与回滚补充

## 目标

- 在 `07-C3` 的写接口闭环之上，补齐 `07-C4 / 09G / 150G`。
- 把 provider policy 从“可写”推进到“可追溯、可回滚、可同步 ops”。

## 实施项

1. 在 `RuntimeProviderRegistry` 中维护版本快照序列，输出 `ProviderPolicyHistory`。
2. 暴露 `GET /backend/v3/api/control/provider-policies`，用于查询所有版本。
3. 暴露 `POST /backend/v3/api/control/provider-policies/rollback`，按 `targetVersion` 回滚。
4. 回滚后生成新快照而不是覆写旧快照，并回填 `rollbackFromVersion`。
5. control-plane 在写入和回滚后刷新 ops provider binding 视图；回滚路径必须走 `replace_provider_binding_snapshots`。
6. 写入与回滚都要补齐 audit；新增 `control.provider_policy_rolled_back`。

## 契约冻结

- 历史接口：`GET /backend/v3/api/control/provider-policies`
- 回滚接口：`POST /backend/v3/api/control/provider-policies/rollback`
- 回滚请求：`{"targetVersion":1}`
- 历史字段：`currentVersion`、`items`、`rollbackFromVersion`
- ops 刷新要求：删除租户 override 后不允许保留旧快照

## 验证清单

- provider registry 合约测试覆盖版本递增与回滚恢复。
- control-plane HTTP 测试覆盖历史查询、回滚、read-after-rollback。
- governance loop 测试覆盖 ops 刷新与 `control.provider_policy_rolled_back`。
- public auth 测试覆盖 `control.read / control.write` 权限守卫。

## 交付物

- `docs/step/07-C4-控制面provider-policy版本与回滚快照闭环-2026-04-08.md`
- `docs/架构/09G-实施计划-provider-policy版本与回滚补充-2026-04-08.md`
- `docs/架构/150G-control-plane-provider-policy版本与回滚设计-2026-04-08.md`
- `docs/review/continuous-optimization-control-plane-provider-policy-version-and-rollback-2026-04-08.md`

