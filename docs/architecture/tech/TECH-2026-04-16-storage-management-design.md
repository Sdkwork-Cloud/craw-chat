> Migrated from `docs/superpowers/specs/2026-04-16-storage-management-design.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-16 Storage Management 标准化设计

## 1. 背景

当前 `sdkwork-im` 已经具备通用的 provider / credential / provider registry 能力，但对象存储仍停留在“通用绑定 + S3-compatible 适配器”的阶段。这个形态可以支撑基础上传与下载，但不足以承载一套行业级、可维护、可审计、可扩展的对象存储管理体系。

本次设计的目标不是“再加几个配置项”，而是把对象存储提升为独立的一等公民领域，贯通以下层面：

- Admin 控制台的配置与可视化
- 后端数据库持久化
- provider-specific schema 与凭据管理
- tenant override 与 global fallback
- 上传会话、presigned URL 和 assetId 流程
- SDK 的统一上传与消息媒体接入能力

## 2. 设计目标

1. 支持当前仓库已枚举的全部对象存储 provider。
2. 支持 tenant 级覆盖，且明确 fallback 到 global 配置。
3. 支持直接在 Admin 控制台录入配置并持久化到数据库。
4. 支持不同云 provider 的不同字段名、不同 credential mode、不同校验逻辑。
5. 支持健康检查、连通性验证、上传验证和审计。
6. 支持未来新增 provider 时只扩展 schema 和 driver，不重做控制面框架。
7. 与现有 provider registry 的 precedence 语义保持一致，但对象存储域独立建模。

## 3. 设计原则

- 对象存储配置必须是“一整套配置块”，不做字段级混合继承。
- secret 与非 secret 必须分离存储。
- 任何错误、健康检查、日志和审计都不得回显明文 secret。
- 管理台必须能展示“当前生效配置来自哪里”，而不是只展示静态表单。
- provider-specific schema 允许差异，但控制面和 SDK 对外要保持统一体验。
- 运行时必须支持按 provider 选择 driver，而不能继续把所有 provider 伪装成单一 S3 兼容实现。

## 4. 当前现状参考

以下现状决定了本设计的边界：

- provider registry 已经具备 `tenant_override -> deployment_profile -> global_default` 的 precedence。
- 现有 Admin API 仍以 `/backend/v3/api/admin/*` 风格暴露通用资源。
- 当前对象存储 adapter 主要位于 `adapters/object-storage-s3/src/lib.rs`，本质是 S3-compatible 形态。
- Admin sandbox 目前只有通用 provider / credentials mock，没有独立 storage 域。

这意味着新方案要做的是“补齐对象存储域”，不是推翻现有 provider registry。

## 5. 推荐方案

本设计采用“一等公民 Storage Management 域 + 多驱动运行时”的方案。

核心思路如下：

- Admin 端新增独立 `Storage Management` 模块。
- 后端新增独立 storage API 和 storage 持久化表。
- 配置层采用 `binding + config + secret + audit` 分离模型。
- 运行时采用 provider plugin id 选择 driver 的方式。
- SDK 暴露统一上传能力，屏蔽不同云 provider 的细节差异。

## 6. 产品范围

### 6.1 Admin 控制台

建议新增独立的 `Storage Management` 模块，并从 `System` 页面提供入口。

模块至少包含以下视图：

- Global Config
- Tenant Overrides
- Effective Resolution
- Provider Schemas
- Validation & Health
- Audit Trail

### 6.2 Provider 范围

当前必须完整支持以下对象存储 provider：

- `object-storage-aliyun`
- `object-storage-tencent`
- `object-storage-volcengine`
- `object-storage-aws`
- `object-storage-google`
- `object-storage-microsoft`

设计上必须保持可扩展，以便后续增加 MinIO、Cloudflare R2、Backblaze B2、Wasabi 等 provider 时不需要重做框架。

### 6.3 能力范围

必须具备以下能力：

- 配置查看
- 配置创建与更新
- 租户覆盖与全局回退
- provider-specific 表单和校验
- secret 安全保存
- 连接性验证
- presign 验证
- 上传探测
- 审计记录
- SDK 上传能力联动

## 7. 数据模型

### 7.1 StorageBinding

StorageBinding 表达某个作用域选择哪个 provider。

建议字段：

- `id`
- `scope_type`：`global | tenant`
- `scope_id`
- `provider_plugin_id`
- `enabled`
- `version`
- `updated_by`
- `updated_at`

约束：

- `(scope_type, scope_id)` 唯一
- 一个作用域同一时刻只允许一个生效 binding

说明：

- `StorageBinding` 只负责“选谁”，不负责具体 bucket 和 secret。
- tenant 覆盖必须是整套覆盖，不允许字段级混合继承。

### 7.2 StorageConfig

StorageConfig 保存非敏感配置和 provider-specific 扩展配置。

建议字段：

- `id`
- `scope_type`
- `scope_id`
- `provider_plugin_id`
- `bucket_or_container`
- `region`
- `endpoint`
- `public_base_url`
- `cdn_base_url`
- `upload_prefix`
- `download_prefix`
- `default_acl`
- `default_storage_class`
- `multipart_enabled`
- `multipart_min_part_size`
- `presign_expires_seconds`
- `path_style`
- `schema_version`
- `provider_config_json`
- `updated_by`
- `updated_at`

说明：

- 主流字段必须结构化，不允许所有 provider 配置都扔进 JSON。
- `provider_config_json` 只作为扩展字段和后向兼容兜底。

### 7.3 StorageSecret

StorageSecret 保存敏感凭据，必须独立表、独立加密。

建议字段：

- `id`
- `scope_type`
- `scope_id`
- `provider_plugin_id`
- `credential_mode`
- `encrypted_secret_payload`
- `secret_fingerprint`
- `encryption_key_id`
- `rotated_at`
- `updated_by`
- `updated_at`

说明：

- `encrypted_secret_payload` 必须是整体密文，不是字段级明文。
- `secret_fingerprint` 用于变更检测，不得可逆。
- 控制台读取时只返回摘要，不返回 secret 原文。

### 7.4 StorageAuditLog

StorageAuditLog 用于记录所有配置变化。

建议字段：

- `id`
- `actor_id`
- `scope_type`
- `scope_id`
- `provider_plugin_id`
- `change_type`
- `before_summary`
- `after_summary`
- `created_at`

建议记录内容：

- 创建 / 更新 / 删除 / 激活 / 轮换 secret / 验证失败 / 验证成功
- 只存摘要，不存明文 secret

## 8. Provider Schema 策略

### 8.1 共有字段

对象存储的通用字段建议统一为：

- `bucket` 或 `container`
- `region`
- `endpoint`
- `publicBaseUrl`
- `cdnBaseUrl`
- `uploadPrefix`
- `downloadPrefix`
- `defaultAcl`
- `defaultStorageClass`
- `multipartEnabled`
- `multipartMinPartSize`
- `presignExpiresSeconds`
- `pathStyle`

### 8.2 Provider-specific 字段

provider-specific 字段必须按 provider 注册 schema，不允许强行压扁成单一模型。

建议支持的 credential mode：

- AWS S3
  - static access key
  - session token
  - role assumption
- 阿里云 OSS
  - accessKeyId / accessKeySecret
  - STS token
- 腾讯云 COS
  - secretId / secretKey
  - session token
- 火山引擎 TOS
  - accessKeyId / secretAccessKey
  - session token
- Google Cloud Storage
  - interoperability key
  - service account JSON
- Microsoft Azure Blob
  - account key
  - SAS token
  - service principal

### 8.3 schema 注册方式

每个 provider 需要具备以下元数据：

- `provider_plugin_id`
- `display_name`
- `domain`
- `supported_credential_modes`
- `required_fields`
- `optional_fields`
- `validation_rules`
- `driver_kind`
- `capabilities`

这样 Admin 控制台可以动态渲染表单，后端也能统一做校验和解析。

## 9. 作用域与生效规则

### 9.1 precedence

对象存储的实际生效顺序沿用现有 registry 的语义：

1. tenant override
2. global default

### 9.2 生效原则

- tenant 配置只有在完整且验证通过后才参与生效。
- 不允许“租户 bucket + 全局 secret”这种混搭。
- 不允许字段级 merge。
- tenant 没有显式配置时，直接 fallback 到 global。

### 9.3 effective config

查询 `effective config` 时必须返回：

- `source`：`tenant | global`
- `providerPluginId`
- `bindingId`
- `configSummary`
- `secretState`
- `validationState`

这个接口是 Admin 控制台的核心可视化能力。

## 10. Admin API

建议新增独立 storage 域 API，而不是继续复用通用 provider / credential 接口。

### 10.1 查询接口

- `GET /backend/v3/api/admin/storage/providers`
- `GET /backend/v3/api/admin/storage/config`
- `GET /backend/v3/api/admin/storage/config/tenants/{tenantId}`
- `GET /backend/v3/api/admin/storage/effective/tenants/{tenantId}`
- `GET /backend/v3/api/admin/storage/health`
- `GET /backend/v3/api/admin/storage/health/tenants/{tenantId}`
- `GET /backend/v3/api/admin/storage/audit`

### 10.2 写入接口

- `POST /backend/v3/api/admin/storage/config`
- `POST /backend/v3/api/admin/storage/config/tenants/{tenantId}`
- `DELETE /backend/v3/api/admin/storage/config/tenants/{tenantId}`
- `POST /backend/v3/api/admin/storage/validate`
- `POST /backend/v3/api/admin/storage/validate/tenants/{tenantId}`
- `POST /backend/v3/api/admin/storage/presign`
- `POST /backend/v3/api/admin/storage/test-upload`
- `POST /backend/v3/api/admin/storage/rotate-secret`
- `POST /backend/v3/api/admin/storage/rotate-secret/tenants/{tenantId}`

### 10.3 请求模型原则

写入接口应该接受完整配置块，而不是零碎 patch。

原因：

- 审计更清晰
- 验证更一致
- 回滚更简单
- 更适合 tenant override 整套替换

### 10.4 响应模型原则

响应中只返回摘要，不返回 secret 原文。

建议摘要字段：

- `configured`
- `providerPluginId`
- `scopeType`
- `scopeId`
- `credentialMode`
- `fieldsPresent`
- `lastValidatedAt`
- `validationState`
- `healthState`

## 11. Admin 控制台

### 11.1 页面结构

建议在 Admin 左侧导航新增 `Storage Management`，并在 `System` 页面保留入口卡片和当前状态摘要。

### 11.2 页面内容

1. Global Config
2. Tenant Overrides
3. Effective Resolution
4. Provider Schemas
5. Validation & Health
6. Audit Trail

### 11.3 交互原则

- provider 选择后动态渲染表单
- secret 字段默认 mask
- 保存前必须可见校验结果
- 切换 tenant 时必须显示 effective config
- 删除 tenant override 后必须立即回落到 global

### 11.4 UI 文案要求

页面文案必须明确区分：

- `configured` 与 `validated`
- `global` 与 `tenant`
- `effective` 与 `draft`
- `secret present` 与 `secret revealed`

这样可以避免运维误判。

## 12. 运行时与驱动

### 12.1 现有适配器演进

当前 `object-storage-s3` 适配器不能直接代表所有 provider 的最终形态。建议保留 `ObjectStorageProvider` 接口，但内部拆成多驱动：

- `s3_driver`
- `gcs_driver`
- `azure_blob_driver`

### 12.2 driver 选择方式

运行时根据以下信息选择 driver：

- `provider_plugin_id`
- `credential_mode`
- `schema_version`
- `capabilities`

### 12.3 driver 职责

每个 driver 需要支持：

- `put_object`
- `signed_upload_url`
- `signed_download_url`
- `provider_health_snapshot`
- `validate_credentials`
- `probe_bucket`

### 12.4 provider 到 driver 的映射

建议映射如下：

- `object-storage-aliyun` -> `s3_driver`
- `object-storage-tencent` -> `s3_driver`
- `object-storage-volcengine` -> `s3_driver`
- `object-storage-aws` -> `s3_driver`
- `object-storage-google` -> `gcs_driver`
- `object-storage-microsoft` -> `azure_blob_driver`

这个映射不是 API 约束，而是当前实现策略。后续可以继续扩展。

## 13. SDK 上传标准

对象存储管理不是单独存在的，最终要服务于媒体消息、文件消息、生成图片、生成视频、卡片附件等业务场景。

建议 SDK 暴露统一上传能力：

- `client.storage.createUpload(...)`
- `client.storage.uploadFile(...)`
- `client.storage.completeUpload(...)`
- `client.storage.getAsset(...)`

返回结构建议包含：

- `assetId`
- `upload`
  - `method`
  - `url`
  - `headers`
  - `expiresAt`
- `asset`
  - `storageUrl`
  - `cdnUrl`
  - `objectKey`
  - `provider`

这样业务侧可以先上传，再通过 `client.send(client.createImageMessage(...))` 或 `client.send(client.createVideoMessage(...))` 发送消息。

### 13.1 SDK 接入原则

- 客户端不需要理解 provider 差异
- 客户端只处理 `assetId + presigned upload`
- 消息对象始终以最终 message 对象为准
- 上传和消息发送必须解耦

## 14. 安全与审计

- secret 必须加密存储
- secret 读取必须服务端解密，客户端不回显
- health response 不得暴露明文敏感数据
- audit 必须记录操作人、作用域、provider、变更摘要
- validate / test-upload 结果必须明确标注失败阶段，但不能输出 secret
- tenant override 删除后不得泄漏上一版 secret

## 15. 校验标准

保存与激活前至少要通过三类校验：

1. schema 校验
2. credential 校验
3. upload/presign 校验

建议统一健康状态：

- `healthy`
- `degraded`
- `invalid`
- `unknown`

建议统一失败阶段：

- `schema`
- `credential`
- `bucket`
- `presign`
- `readback`

## 16. 迁移与兼容

本设计不要求把通用 provider / credential 模型立刻废弃。

建议策略：

- 通用 provider / credential 继续服务其它域
- storage 域单独走新模型
- sandbox 与 mock 逐步增加 storage 桩数据
- 文档与 SDK 同步升级

如果后续要做旧数据迁移，可以用一次性迁移脚本把存量对象存储配置映射到新表。

## 17. 实施顺序

建议按以下顺序落地：

1. 数据库表与迁移
2. storage 域后端 API
3. Admin sandbox mock
4. Admin 控制台页面
5. driver 化运行时
6. SDK upload 标准
7. 文档与示例同步

## 18. 结论

本方案的核心是：**对象存储作为独立业务域管理，tenant override 只做整套覆盖，secret 与 config 分离，多 provider 通过 schema registry 和 driver registry 支撑，SDK 通过统一 upload 流程屏蔽云厂商差异。**

这套设计可以满足当前 `sdkwork-im` 的控制台管理、运行时上传、SDK 接入和后续多语言扩展要求，也能保持与现有 provider registry 语义一致。

