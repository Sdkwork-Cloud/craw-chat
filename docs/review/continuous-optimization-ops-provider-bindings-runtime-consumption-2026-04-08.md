# 持续优化：ops provider bindings runtime consumption - 2026-04-08

## 1. 当前 step / 波次

- 当前 step：`Step 07`
- 当前波次：`07-C / CP07-3` 补充波次
- 本轮主题：把控制面 provider binding 求值结果真正接入 `ops-service` 的只读消费链路

## 2. 本轮为什么做

`07-A` 已完成：

- `GET /backend/v3/api/control/provider_bindings`
- `tenant override / deployment_profile / global default` 的控制面求值可见性

但仍缺：

- `runtime / ops` 对同一份结果的消费
- 统一 diagnostics 对 provider 治理结果的展示

这意味着控制面能看见，运维面却仍然盲。

## 3. 本轮实际完成

- `ops-service` 新增：
  - `ProviderBindingItemView`
  - `ProviderBindingSnapshotView`
  - `ProviderBindingsView`
- `OpsRuntime` 新增：
  - `update_provider_binding_snapshot(...)`
  - `provider_bindings_view()`
- `ops-service` 新增 `GET /backend/v3/api/ops/provider_bindings`
- `diagnostic bundle` 新增 `providerBindings`
- `control-plane-api` 在 provider binding 查询后，会把结果镜像进 `OpsRuntime`
- 新增治理装配入口：
  - `build_app_with_cluster_provider_registry_and_governance_sinks(...)`

## 4. 改动文件

- `services/ops-service/src/lib.rs`
- `services/ops-service/tests/ops_runtime_test.rs`
- `services/ops-service/tests/http_smoke_test.rs`
- `services/ops-service/tests/public_auth_test.rs`
- `services/control-plane-api/src/lib.rs`
- `services/control-plane-api/tests/governance_loop_test.rs`
- `docs/step/07-C-控制面provider绑定ops消费闭环-2026-04-08.md`
- `docs/架构/09D-实施计划-ops-provider-binding消费补充-2026-04-08.md`
- `docs/架构/150D-ops-provider-binding运行态消费与漂移视图设计-2026-04-08.md`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md`
- `docs/架构/09-实施计划.md`
- `services/sdkwork-im-cloud-gateway/tests/provider_plugin_docs_test.rs`

## 5. 验证

- `cargo test -p ops-service --offline --test ops_runtime_test -- --nocapture`
- `cargo test -p ops-service --offline --test http_smoke_test -- --nocapture`
- `cargo test -p ops-service --offline --test public_auth_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`

## 6. 当前还差什么

- 仍然只是快照镜像，不是 drift 计算
- provider policy 写接口、审计和版本回滚还没接入
- hot-path runtime 还没有直接消费 provider binding 治理结果

## 7. 下一轮做什么

1. 从“镜像快照”推进到“provider binding drift 视图”
2. 再补 provider policy 写接口与审计闭环
