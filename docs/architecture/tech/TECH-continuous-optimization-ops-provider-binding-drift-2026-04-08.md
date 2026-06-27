> Migrated from `docs/review/continuous-optimization-ops-provider-binding-drift-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 持续优化：ops provider binding drift - 2026-04-08

## 1. 当前 step / 波次

- 当前 step：`Step 07`
- 当前波次：`07-C2 / CP07-3` 补充波次
- 本轮主题：把 provider binding 从“ops 可见快照”推进到“ops 可见 drift”

## 2. 本轮为什么做

`07-C` 已完成：

- `GET /backend/v3/api/ops/provider_bindings`
- `providerBindings`
- `control-plane -> OpsRuntime -> ops-service` 的镜像快照链路

但仍缺：

- ops 对租户相对全局 provider baseline 的差异识别
- diagnostics 对 provider drift 的统一展示

这意味着运维面能看到快照，却还看不见偏移。

## 3. 本轮实际完成

- `ops-service` 新增：
  - `ProviderBindingDriftItemView`
  - `ProviderBindingDriftView`
- `OpsRuntime` 新增：
  - `provider_binding_drift_view()`
- `ops-service` 新增 `GET /backend/v3/api/ops/provider_bindings/drift`
- `diagnostic bundle` 新增 `providerBindingDrift`
- drift 基线固定为全局 snapshot（`tenantId = null`）
- drift 语义冻结为：
  - `plugin_changed`
  - `selection_source_changed`
  - `plugin_and_selection_source_changed`

## 4. 改动文件

- `services/ops-service/src/lib.rs`
- `services/ops-service/tests/ops_runtime_test.rs`
- `services/ops-service/tests/http_smoke_test.rs`
- `services/ops-service/tests/public_auth_test.rs`
- `services/control-plane-api/tests/governance_loop_test.rs`
- `docs/step/07-C2-控制面provider绑定漂移视图闭环-2026-04-08.md`
- `docs/架构/09E-实施计划-ops-provider-binding漂移补充-2026-04-08.md`
- `docs/架构/150E-ops-provider-binding漂移检测与运维视图设计-2026-04-08.md`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md`
- `docs/架构/09-实施计划.md`
- `services/sdkwork-im-cloud-gateway/tests/provider_plugin_docs_test.rs`

## 5. 验证

- `cargo test -p ops-service --offline --test ops_runtime_test -- --nocapture`
- `cargo test -p ops-service --offline --test http_smoke_test -- --nocapture`
- `cargo test -p ops-service --offline --test public_auth_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`

## 6. 当前还差什么

- drift 仍只是只读视图，不是 provider policy 写治理
- audit actor、配置版本和回滚快照仍未接入
- drift 结果还没有进入独立告警与回滚决策

## 7. 下一轮做什么

1. 补 provider policy 写接口、审计 actor 和配置版本闭环
2. 再把 drift 接到告警与回滚编排

