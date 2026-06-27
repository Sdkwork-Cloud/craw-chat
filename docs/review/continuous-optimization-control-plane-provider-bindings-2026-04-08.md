# 持续优化：control-plane provider bindings - 2026-04-08

## 1. 当前 step / 波次

- 当前 step：`Step 07`
- 当前波次：`07-A / CP07-1` 补充波次
- 本轮主题：把 provider 实际绑定求值结果暴露到控制面，只读可见，不破坏已有静态快照契约

## 2. 本轮为什么做

上一轮已经有：

- `ProviderRegistry`
- `StaticProviderRegistry`
- `GET /backend/v3/api/control/provider-registry`

但仍缺：

- `tenant override` 的控制面可见性
- `deployment_profile` 的控制面可见性
- 面向租户的实际 `effective binding` 查询入口

这会导致 Step 06 已经进入真实运行时的 provider 选择结果，仍然无法被 Step 07 控制面直接查看。

## 3. 本轮实际完成

- 新增 `GET /backend/v3/api/control/provider_bindings`
- 新增 `tenantId` 查询参数，允许查看租户视角下的 provider 求值结果
- 返回体新增：
  - `interfaceVersion`
  - `tenantId`
  - `effectiveBindings`
  - `precedence`
- 保留 `GET /backend/v3/api/control/provider-registry` 作为静态矩阵接口，不兼容式修改旧响应
- 新增 `build_app_with_cluster_and_provider_registry(...)`，把 provider registry 注入能力显式暴露给控制面装配与测试
- 新增红绿测试，冻结：
  - RTC 默认 `rtc-volcengine`
  - 对象存储 `deployment_profile = object-storage-volcengine`
  - 租户覆盖 `rtc-aliyun / object-storage-aws`
- 新增 public auth 回归，确保新接口仍受 `control.read` 约束

## 4. 改动文件

- `services/control-plane-api/src/lib.rs`
- `services/control-plane-api/tests/provider_registry_test.rs`
- `services/control-plane-api/tests/public_auth_test.rs`
- `docs/step/07-A-控制面provider绑定求值可见性闭环-2026-04-08.md`
- `docs/架构/09C-实施计划-控制面provider绑定治理补充-2026-04-08.md`
- `docs/架构/150C-控制面provider绑定求值与租户治理设计-2026-04-08.md`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md`
- `docs/架构/09-实施计划.md`
- `services/sdkwork-im-cloud-gateway/tests/provider_plugin_docs_test.rs`

## 5. 验证

- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`

## 6. 当前还差什么

- provider registry 仍非持久化真源
- 没有 provider policy 写接口
- 没有配置版本、审计 actor 与回滚链路
- runtime / ops / audit 仍未消费 `effective provider bindings`

## 7. 下一轮做什么

1. 把 `effective provider bindings` 接入 runtime / ops 的只读消费链路
2. 增加 provider policy 写接口与审计记录
3. 再把 provider binding 差异纳入 rollout / drift / rollback 视图
