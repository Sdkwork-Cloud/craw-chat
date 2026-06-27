> Migrated from `docs/架构/150BH-principal-profile-runtime-provider-selection-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 用户模块默认运行时提供商选择设计

## 1. 问题

- `PrincipalProfileProvider` 的主链路接入已存在。
- 但默认启动入口此前始终返回 `UpstreamContextPrincipalProfileProvider`。
- 这导`principal-profile-external-catalog` 虽有契约、stub 与主链路测试，却没有真实默认装配路径。

## 2. 设计目标

- 默认仍走统一 `PrincipalProfileProvider` port
- 默认值安全：未配置时仍为 `local`
- external 可真实启用，不依赖测试注。
- 主链路输出与现有 external stub 语义一。
- 配置错误可显式暴露。

## 3. 方案

### 3.1 Provider 选择

- `sdkwork_im_PRINCIPAL_PROFILE_PROVIDER=local|external`
- 缺省`local`
- 非法值直接拒绝，避免静默回退掩盖配置错误

### 3.2 External 目录。

- `sdkwork_im_PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH=<json>`
- `sdkwork_im_PRINCIPAL_PROFILE_EXTERNAL_SYSTEM=<name>`
- catalog 最小结构：

```json
{
  "externalSystem": "corp-idp",
  "users": [
    {
      "tenantId": "100001",
      "userId": "1",
      "displayName": "Demo User",
      "externalPrincipalId": "ext::3",
      "attributes": {
        "source": "external"
      }
    }
  ]
}
```

### 3.3 行为

- `get_user / batch_get_users / search_users` 先查 override，再与 external catalog
- `map_external_principal` 支持 external principal 到平台用户映。
- `create_or_bind / update / disable` 提供最override 能力，保证运行时闭环
- `sender / member / bootstrap member / mutation actor` 继续只消`PrincipalProfileProvider`

## 4. 结果

- external 用户目录首次进入默认运行时入。
- 架构声明从“测试注入成立”推进到“默认入口可配置成立。
- 为后RTC / Object Storage / IoT 的真provider adapter 选择提供一致模。

## 5. 后续

- 与 external catalog 配置补到 operator 文档
- 与 external provider 增加健康检查与配置错误用例
- 继续推进 `rtc-volcengine` 最小真adapter

