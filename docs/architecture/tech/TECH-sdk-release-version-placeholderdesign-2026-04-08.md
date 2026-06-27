> Migrated from `docs/架构/150X-sdk-release-version-placeholder设计-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150X - SDK release version placeholder 设计

## 设计目标

在真实 SDK 版本冻结之前，bundle 级 release catalog 必须显式表达“版本未分配”，而不是让消费方从目录名、README 或发布时间自行猜版本号。

## 当前最小 contract

每个 `sdkArtifacts[*]` 至少新增：

- `plannedVersion`
- `versionStatus`

## 当前模板态语义

### 1. `plannedVersion = null`

- 表示当前没有已经冻结的目标版本号
- 这是显式占位，不是字段遗漏

### 2. `versionStatus = version_unassigned_pending_freeze`

- 表示版本号仍待后续 freeze 决议
- 它与 `generationStatus / releaseStatus` 分离：
  - 版本可未冻结
  - 生成可未开始
  - 发布可未发生

## 为什么不能省略

- 省略字段会让不同 bundle 用不同方式表达“还没有版本”
- 显式占位后，后续才能安全推进到：
  - `plannedVersion = 0.x.y`
  - `versionStatus = version_frozen`

## 文档落点

以下公开入口必须同步：

- `sdks/README.md`
- `artifacts/releases/README.md`
- `artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`

## 非目标

- 不在当前轮次引入真实 semver 决议
- 不把 placeholder 扩写成完整 release note
- 不接入真实 SDK 生成器或包仓库

