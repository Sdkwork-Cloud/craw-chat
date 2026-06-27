> Migrated from `docs/step/07-A-控制面provider绑定求值可见性闭环-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 07-A: 控制面provider绑定求值可见性闭环

## 当前 step / 波次 / 是否闭环

- 当前 step：`Step 07 - 控制面与协议治理落地`
- 当前波次：`07-A / CP07-1` 补充波次
- 本轮状态：本波次已闭环；`Step 07` 整体仍未闭环

## 本轮为什么做

- `GET /backend/v3/api/control/provider-registry` 只能返回静态 provider 矩阵。
- `tenant override / deployment_profile / global default` 的实际求值结果还没有控制面只读视图。
- `Step 06-C` 已经把 RTC 录制播放面收口到对象存储 provider runtime，下一步必须让控制面能看见同一套 binding 结果。

## 实际完成项

- 保留 `GET /backend/v3/api/control/provider-registry` 作为静态矩阵快照，不破坏既有契约。
- 新增 `GET /backend/v3/api/control/provider_bindings`：
  - 支持可选查询参数 `tenantId`
  - 返回 `interfaceVersion / tenantId / effectiveBindings / precedence`
- `control-plane-api` 新增 `build_app_with_cluster_and_provider_registry(...)`，允许在测试或装配层注入自定义 `ProviderRegistry`。
- 控制面现在可明确区分：
  1. `tenant_override`
  2. `deployment_profile`
  3. `global_default`
- 已用真实测试冻结两类关键求值结果：
  - RTC 在默认情况下仍显示 `rtc-volcengine`
  - 对象存储可显示 `deployment_profile = object-storage-volcengine`
  - 指定租户后可显示 `rtc-aliyun / object-storage-aws` 的 `tenant_override`

## 改动文件

- `services/control-plane-api/src/lib.rs`
- `services/control-plane-api/tests/provider_registry_test.rs`
- `services/control-plane-api/tests/public_auth_test.rs`
- `docs/step/07-A-控制面provider绑定求值可见性闭环-2026-04-08.md`
- `docs/架构/09C-实施计划-控制面provider绑定治理补充-2026-04-08.md`
- `docs/架构/150C-控制面provider绑定求值与租户治理设计-2026-04-08.md`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md`
- `docs/架构/09-实施计划.md`
- `docs/review/continuous-optimization-control-plane-provider-bindings-2026-04-08.md`
- `services/sdkwork-im-cloud-gateway/tests/provider_plugin_docs_test.rs`

## 验证结果

- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`

## 文档回写状态

- `docs/step`：已回写本文件
- `docs/架构`：已回写 `09C / 150C / 142 / 09-实施计划`
- `docs/review`：已回写 `continuous-optimization-control-plane-provider-bindings-2026-04-08.md`

## 剩余差距

- provider registry 仍不是持久化真源。
- 还没有 provider selection policy 的写接口与审计链路。
- runtime / ops / audit 还没有消费 `effective provider bindings` 做漂移检测或运行态展示。

## 下一轮动作

1. 把 `effective provider bindings` 接入 runtime / ops 的只读消费与 drift 视图。
2. 再补 provider policy 的写接口、审计记录和回滚边界。

