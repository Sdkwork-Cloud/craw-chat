> Migrated from `docs/架构/150U-sdk-release-catalog设计-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150U - SDK release catalog 设计

## 设计目标

release bundle 不能只说明“有哪些 SDK 目录”，还必须提供一份 machine-readable catalog，让 bundle、README 与后续发布审计共享同一份 SDK 目录真源。

## 目录 contract

bundle 内固定新增：

- `artifacts/releases/<bundle-id>/sdk-release-catalog.json`

当前最小顶层字段：

- `version`
- `bundleId`
- `wave`
- `artifact`
- `state`
- `updatedAt`
- `sdkArtifacts`

## sdkArtifacts 最小模型

每个 SDK 入口至少包含：

- `id`
- `audience`
- `language`
- `package`
- `readmePath`
- `generationStatus`
- `releaseStatus`

## 当前冻结的最小 SDK 集

- `app-typescript`
- `app-flutter`
- `admin-typescript`
- `admin-flutter`

这四项对应当前 workspace 中已经公开存在的 app/admin 与 TypeScript/Flutter 目录入口。

## 状态语义

### 1. `state`

- bundle 级整体状态
- 当前固定为 `template_only_pending_generation`
- 表示：
  - 目录 contract 已冻结
  - 真实生成链尚未补齐

### 2. `generationStatus`

- 单个 SDK 入口当前是否已有真实生成产物
- 当前固定为 `template_only_pending_generation`

### 3. `releaseStatus`

- 单个 SDK 入口当前是否已对外发布
- 当前固定为 `not_published`

## 为什么要放在 bundle 里

- SDK release 状态属于交付物审计的一部分，而不仅是 workspace README 的注释
- 放在 `artifacts/releases/<bundle-id>/` 内，才能与同 bundle 的：
  - `bundle-manifest.md`
  - `post-release-evidence-index`
  - review 证据
  保持同一归档语义

## 文档落点

以下公开面必须保持一致：

- `sdks/README.md`
- `artifacts/releases/README.md`
- `artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`

## 非目标

- 不引入真实 SDK 生成流水线
- 不引入真实 SDK 包仓库发布流程
- 不在当前轮次新增版本求解、tag 规则或 checksum 规范

