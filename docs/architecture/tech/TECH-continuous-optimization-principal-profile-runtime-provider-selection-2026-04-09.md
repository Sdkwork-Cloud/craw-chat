> Migrated from `docs/review/continuous-optimization-principal-profile-runtime-provider-selection-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 持续优化评审记录：用户模块默认运行时提供商选择

## 结论

- 发现真实偏差：文档长期宣称 `principal-profile-upstream-context / principal-profile-external-catalog` 双形态已闭环，但默认运行时入口始终硬编码本地实现，`external` 只能在测试里手工注入。
- 本轮已修复：`sdkwork-im-server` 默认装配已支持按配置切换到真实 `principal-profile-external-catalog`。
- 本轮未扩大范围：未进入 RTC / 对象存储 / IoT 的真实云厂商 SDK 适配，这些仍在后续 backlog。

## Bug 详情

### P1 已修复：默认应用无法选用 external 用户模块插件

- 现象：
  - `build_default_app*` 全链路都经由 `build_default_principal_profile_provider()`
  - 该函数原先始终返回 `UpstreamContextPrincipalProfileProvider`
  - 结果是默认运行时即使配置 external 目录，也不会真正走 `principal-profile-external-catalog`
- 风险：
  - 架构文档与实现不一致
  - 外部用户目录集成无法落地到真实启动入口
  - operator 误以为 external 已可用，实际生产行为仍是 local fallback
- 对标要求：
  - 标杆产品的 provider/plugin 不仅要“可注入测试”，更要“默认入口可配置选择、错误可观测、主链路语义一致”

## 修复方案

- 增加默认提供商选择配置：
  - `SDKWORK_IM_PRINCIPAL_PROFILE_PROVIDER=local|external`
  - 默认 `local`
- 增加 external 目录配置：
  - `SDKWORK_IM_PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH=<json>`
  - `SDKWORK_IM_PRINCIPAL_PROFILE_EXTERNAL_SYSTEM=<name>`，默认 `external-directory`
- 新增真实 `ExternalCatalogPrincipalProfileProvider`
  - 从外部目录 JSON 读取用户
  - 统一输出 `displayName / externalSystem / externalPrincipalId / principalProfilePluginId`
  - 支持 `get_user / batch_get_users / search_users / map_external_principal`
  - 对 `create_or_bind / update / disable` 提供最小可运行覆盖，保证主链路和目录绑定不再只停留在 stub

## 验证证据

- 新增回归测试：
  - `services/sdkwork-im-cloud-gateway/tests/principal_profile_provider_runtime_selection_test.rs`
- 通过：
  - `cargo test -p sdkwork-im-cloud-gateway --test principal_profile_provider_runtime_selection_test -- --nocapture`
  - `cargo test -p sdkwork-im-cloud-gateway principal_profile_provider -- --nocapture`

## 剩余问题清单

### P1 待处理：Windows operator 脚本 help 面仍有残缺

- `retired-lifecycle-install.cmd --help` 仍未补 GNU-style `.cmd` usage
- `retired-lifecycle-deploy.cmd --help` 仍未补 GNU-style `.cmd` usage

### P1 待处理：provider 体系仍未进入真实云适配阶段

- RTC 仍缺 `volcengine / aliyun / tencent` 的真实 adapter
- 对象存储仍缺统一 `S3-compatible` 真实实现
- IoT 仍缺 `MQTT / 小智协议` 之外的控制面编排与设备治理深化

## 下一步计划

1. 冻结 `retired-lifecycle-install.cmd --help` 与 `retired-lifecycle-deploy.cmd --help` 的 Windows discoverability 合约。
2. 为 `principal-profile-external-catalog` 增加 operator 示例配置文档与健康检查断言。
3. 进入 `rtc-volcengine` 最小真实 adapter，建立 provider/plugin 从“契约闭环”到“运行闭环”的下一段主路径。

