# Step 07-C: 控制面provider绑定ops消费闭环

## 当前 step / 波次 / 是否闭环

- 当前 step：`Step 07 - 控制面与协议治理落地`
- 当前波次：`07-C / CP07-3` 补充波次
- 本轮状态：本波次已闭环；`Step 07` 整体仍未闭环

## 本轮为什么做

- `07-A` 已让控制面能计算并返回 `effective provider bindings`。
- 但 `runtime / ops` 仍然看不见这份治理结果，控制面和运维面之间还缺少同一份只读消费链路。
- `Step 07` 的闭环判定明确要求：运行时必须消费治理结果，而不是只有接口层存在。

## 实际完成项

- `ops-service` 新增 `ProviderBindingItemView / ProviderBindingSnapshotView / ProviderBindingsView`。
- `OpsRuntime` 新增 provider binding snapshot 存储与读取能力：
  - `update_provider_binding_snapshot(...)`
  - `provider_bindings_view()`
- `ops-service` 新增只读接口：
  - `GET /backend/v3/api/ops/provider_bindings`
- `ops diagnostics` 现在会同时携带 `providerBindings`，不再只暴露 cluster / lag / projection。
- `control-plane-api` 在返回 `GET /backend/v3/api/control/provider_bindings` 时，会把同一份求值结果镜像写入 `OpsRuntime`。
- 新增带自定义 provider registry 的治理装配入口：
  - `build_app_with_cluster_provider_registry_and_governance_sinks(...)`

## 改动文件

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
- `docs/review/continuous-optimization-ops-provider-bindings-runtime-consumption-2026-04-08.md`
- `services/sdkwork-im-cloud-gateway/tests/provider_plugin_docs_test.rs`

## 验证结果

- `cargo test -p ops-service --offline --test ops_runtime_test -- --nocapture`
- `cargo test -p ops-service --offline --test http_smoke_test -- --nocapture`
- `cargo test -p ops-service --offline --test public_auth_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`

## 文档回写状态

- `docs/step`：已回写本文件
- `docs/架构`：已回写 `09D / 150D / 142 / 09-实施计划`
- `docs/review`：已回写 `continuous-optimization-ops-provider-bindings-runtime-consumption-2026-04-08.md`

## 剩余差距

- 当前只读消费链路落在 `ops-service`，还没有更深入的 runtime hot-path 消费。
- `provider binding drift` 还没有独立差异计算与告警语义。
- provider policy 写接口、审计 actor、配置版本与回滚能力仍未落地。

## 下一轮动作

1. 把 provider binding drift 视图从“快照镜像”推进到“差异检测”。
2. 再补 provider policy 写接口与审计闭环。
