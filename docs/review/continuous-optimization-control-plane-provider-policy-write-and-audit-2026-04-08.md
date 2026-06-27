# 持续优化：control-plane provider policy write and audit - 2026-04-08

## 1. 当前 step / 波次

- 当前 step：`Step 07`
- 当前波次：`07-C3 / CP07-4` 补充波次
- 本轮主题：补最小 provider policy 写接口与审计闭环

## 2. 本轮为什么做

当前仓库已经具备：

- `GET /backend/v3/api/control/provider_bindings`
- `GET /backend/v3/api/ops/provider_bindings`
- `GET /backend/v3/api/ops/provider_bindings/drift`

但仍缺：

- control-plane 写 provider policy 的入口
- 写后对 ops / audit 的闭环 side effect

这意味着 provider 治理已经能看见，却还不能由 control-plane 最小可控。

## 3. 本轮实际完成

- `im-platform-contracts` 新增 `RuntimeProviderRegistry`
- `control-plane-api` 新增 `POST /backend/v3/api/control/provider_bindings`
- 写入后仍返回 `effectiveBindings`
- 写入后会把同一份结果镜像到 `OpsRuntime`
- 写入后会记录 audit anchor
- 当前 audit 动作冻结为：
  - `control.provider_deployment_profile_updated`
  - `control.provider_tenant_override_updated`
- 当前校验已冻结：
  - cross-domain plugin id 会被拒绝

## 4. 改动文件

- `crates/im-platform-contracts/src/provider.rs`
- `crates/im-platform-contracts/tests/provider_registry_contract_test.rs`
- `services/control-plane-api/src/lib.rs`
- `services/control-plane-api/tests/provider_registry_test.rs`
- `services/control-plane-api/tests/governance_loop_test.rs`
- `services/control-plane-api/tests/public_auth_test.rs`
- `docs/step/07-C3-控制面provider-policy写接口与审计闭环-2026-04-08.md`
- `docs/架构/09F-实施计划-provider-policy写接口与审计补充-2026-04-08.md`
- `docs/架构/150F-control-plane-provider-policy写接口与审计设计-2026-04-08.md`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md`
- `docs/架构/09-实施计划.md`
- `services/sdkwork-im-cloud-gateway/tests/provider_plugin_docs_test.rs`

## 5. 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`

## 6. 当前还差什么

- provider policy 还不是持久化真源
- 配置版本与回滚快照还没接入
- drift 还没进入告警/回滚编排

## 7. 下一轮做什么

1. 补 provider policy 版本与 rollback snapshot
2. 再补 drift 对告警与回滚的接入
