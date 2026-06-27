# Step 07-C3: 控制面provider policy写接口与审计闭环

## 当前 step / 波次 / 是否闭环

- 当前 step：`Step 07 - 控制面与协议治理落地`
- 当前波次：`07-C3 / CP07-4` 补充波次
- 本轮状态：本波次已闭环；`Step 07` 整体仍未闭环

## 本轮为什么做

- `07-A / 07-C / 07-C2` 已经让 provider binding 的求值、ops 消费和 drift 视图可见。
- 但控制面还没有最小 provider policy 写接口，provider 治理仍停留在“只能看，不能管”。
- `Step 07` 要求 control-plane 与 audit 形成治理闭环，因此下一步必须补最小写入口和审计锚点。

## 实际完成项

- `im-platform-contracts` 新增 `RuntimeProviderRegistry`，把：
  - 插件矩阵 / 全局默认
  - `deployment_profile`
  - `tenant_override`
  拆成可写的运行时 provider policy owner。
- `control-plane-api` 新增 `POST /backend/v3/api/control/provider_bindings`
- 写接口最小请求固定为：
  - `tenantId`
  - `domain`
  - `pluginId`
- 当前语义固定为：
  - `tenantId = null`：更新 `deployment_profile`
  - `tenantId != null`：更新 `tenant_override`
- 写入后返回同一份 `effectiveBindings` 视图，不另起第二套返回模型。
- 写入后会继续把更新后的 binding snapshot 镜像到 `OpsRuntime`。
- 新增最小审计动作：
  - `control.provider_deployment_profile_updated`
  - `control.provider_tenant_override_updated`
- 公开权限边界保持不变：
  - `GET /backend/v3/api/control/provider_bindings` 仍要求 `control.read`
  - `POST /backend/v3/api/control/provider_bindings` 要求 `control.write`

## 改动文件

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
- `docs/review/continuous-optimization-control-plane-provider-policy-write-and-audit-2026-04-08.md`
- `services/sdkwork-im-cloud-gateway/tests/provider_plugin_docs_test.rs`

## 验证结果

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`

## 文档回写状态

- `docs/step`：已回写本文件
- `docs/架构`：已回写 `09F / 150F / 142 / 09-实施计划`
- `docs/review`：已回写 `continuous-optimization-control-plane-provider-policy-write-and-audit-2026-04-08.md`

## 剩余差距

- 当前 provider policy 仍是运行时内存态，不是持久化真源。
- 配置版本、回滚快照和删除/清理接口还没有落地。
- drift 结果还没有接入独立告警和自动回滚编排。

## 下一轮动作

1. 补 provider policy 配置版本、回滚快照和清理接口。
2. 再把 drift 结果接入告警与回滚决策。
