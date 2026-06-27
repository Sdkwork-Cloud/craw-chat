> Migrated from `docs/step/06-B-对象存储插件与媒体运行时闭环-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 06-B: 对象存储插件与媒体运行时闭环

## 本轮目标

- 把 `ObjectStorageProvider` 从“只有契约和矩阵”推进到“真实 adapter + 运行时选型 + HTTP surface”。
- 保持与 RTC provider 体系一致的抽象风格：
  - provider registry 负责选择
  - runtime 负责绑定
  - service 只依赖 provider-agnostic contract

## 已完成

- 新增 `adapters/object-storage-s3`
  - 统一承载 `阿里云 / 腾讯云 / 火山引擎 / AWS / Google / Microsoft` 六种对象存储插件。
  - 统一走 `S3` 或 `S3 gateway` 兼容能力。
- `StaticProviderRegistry` 增加 `deployment_profile` 选择层。
- `media-service` 默认通过 deployment profile 选择 `object-storage-volcengine`。
- `complete_upload(...)` 不再信任请求体里的 `storageProvider` 字符串，而是走运行时 provider 绑定。
- 新增 HTTP surface
  - `GET /im/v3/api/media/{mediaAssetId}/download_url`
  - `GET /backend/v3/api/media/provider_health`
- `sdkwork-im-server` 已镜像以上 surface。

## 架构标准

- provider 选择顺序固定为：
  1. tenant override
  2. deployment_profile
  3. global default
- 业务层禁止直接拼接 provider 私有 URL。
- 媒体资源完成上传后，下载地址必须由 `ObjectStorageProvider` 统一签发。
- 新增 provider 能力时，必须同时补齐：
  - adapter contract test
  - standalone service test
  - standalone.split-services.development assembled test
  - `docs/review / docs/step / docs/架构` 回写

## 当前结论

- RTC provider 已完成 `session / credential / callback / health / artifact` 基线。
- 对象存储已完成 `adapter + runtime + health + signed download url` 基线。
- 下一轮优先把 RTC recording artifact 回流到统一 `ObjectStorageProvider`。

