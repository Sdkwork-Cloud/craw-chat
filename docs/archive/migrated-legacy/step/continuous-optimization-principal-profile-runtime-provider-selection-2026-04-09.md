# 持续优化 Step：用户模块默认运行时提供商选择

## 本轮目标

- 让默认运行时入口真正支持 `principal-profile-upstream-context / principal-profile-external-catalog` 二选一，而不是只在测试里手工注入 external。

## 执行步骤

1. 补失败测试，要求默认 `build_default_app_with_runtime_dir(...)` 在 external 配置下输出 `principal-profile-external-catalog` 元数据。
2. 跑红灯，确认失败原因是默认入口仍绑定 local provider。
3. 在 `services/sdkwork-im-cloud-gateway/src/node/principal_profile.rs` 实现：
   - provider 选择配置解析
   - external 用户目录加载
   - external 主链路元数据映射
4. 回跑新测试与既有 `principal_profile_provider` 主链路测试。
5. 回写 review / 架构文档，记录偏差、修复和下一步。

## 配置面

- `SDKWORK_IM_PRINCIPAL_PROFILE_PROVIDER=local|external`
- `SDKWORK_IM_PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH=<json>`
- `SDKWORK_IM_PRINCIPAL_PROFILE_EXTERNAL_SYSTEM=<name>`

## 结果

- 默认入口已支持 external 用户模块插件选择
- external 用户目录可进入 conversation / member / sender 主链路
- 文档此前“只在 stub 中成立”的能力，已补成真实运行时能力

## 下一轮

- 继续补 `retired-lifecycle-install.cmd` / `retired-lifecycle-deploy.cmd` 的 help discoverability
- 再推进真实 RTC provider adapter
