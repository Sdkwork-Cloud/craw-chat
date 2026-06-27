> Migrated from `docs/step/07-C11-控制面provider-policy历史diff回滚状态闭环-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 07-C11: 控制面 provider-policy 历史 diff 回滚状态闭环
## 当前闭环编号

- 所属 step: `Step 07`
- 当前波次: `07-C11 / CP07-11A`
- 目标: 为 `history / diff / rollback` 成功响应补统一 `status`，让 provider-policy 成功路径都能按显式状态消费。

## 本轮实现

- `services/control-plane-api/src/lib.rs`
  - `GET /backend/v3/api/control/provider-policies` 返回 `status=history`
  - `GET /backend/v3/api/control/provider-policies/diff` 返回 `status=diff`
  - `POST /backend/v3/api/control/provider-policies/rollback` 返回 `status=rolled_back`
- 采用 HTTP 包装层补充 `status`，底层 `ProviderPolicyHistory / ProviderPolicyDiff` contract 不改动。
- 已有 `preview / applied / noop` 状态保持不变。

## 接口冻结

- `GET /backend/v3/api/control/provider-policies`
  - `status=history`
  - `currentVersion`
  - `items`
- `GET /backend/v3/api/control/provider-policies/diff`
  - `status=diff`
  - `fromVersion`
  - `toVersion`
  - `deploymentProfileChanges`
  - `tenantOverrideChanges`
- `POST /backend/v3/api/control/provider-policies/rollback`
  - `status=rolled_back`
  - `currentVersion`
  - `items`

## 验证

- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 文档同步

- `docs/step/07-C11-控制面provider-policy历史diff回滚状态闭环-2026-04-08.md`
- `docs/架构/09N-实施计划-provider-policy历史diff回滚状态补充-2026-04-08.md`
- `docs/架构/150N-control-plane-provider-policy-history-diff-rollback-status设计-2026-04-08.md`
- `docs/review/continuous-optimization-control-plane-provider-policy-history-diff-rollback-status-2026-04-08.md`

## 下一缺口

- 下一轮进入 `07-C12`，优先评估 provider-policy error 路径是否也要补统一结果状态或稳定错误分类，完成 success/error 两侧的消费收口。

