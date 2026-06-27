> Migrated from `docs/架构/150V-sdk-release-catalog-schema设计-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150V - SDK release catalog schema 设计

## 设计目标

`sdk-release-catalog.json` 必须拥有和 release evidence index 一样清晰的 schema contract，避免不同 bundle 自行扩写字段、状态和值域。

## schema 落点

固定路径：

- `artifacts/releases/schemas/sdk-release-catalog.schema.json`

对应 catalog 必须声明：

- `$schema = ../schemas/sdk-release-catalog.schema.json`

## 顶层 contract

当前最小顶层字段：

- `$schema`
- `version`
- `bundleId`
- `wave`
- `artifact`
- `state`
- `updatedAt`
- `sdkArtifacts`

## 状态 contract

### 1. `state`

当前最小枚举：

- `template_only_pending_generation`
- `generated_pending_publication`
- `published`

### 2. `sdkArtifacts[*].generationStatus`

当前最小枚举：

- `template_only_pending_generation`
- `generated`

### 3. `sdkArtifacts[*].releaseStatus`

当前最小枚举：

- `not_published`
- `published`

## 为什么先冻结 schema

- 先冻结 schema，才能确保后续真实生成链接入时不会改变 bundle 目录真源
- 先有 schema，后续才能安全扩展：
  - 版本号
  - artifact 路径
  - 发包时间
  - 发布仓库元数据

## 文档落点

以下 release 面必须保持一致：

- `artifacts/releases/README.md`
- `artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`

## 非目标

- 不引入 JSON schema 自动校验 CLI
- 不接入真实包管理仓库
- 不在当前轮次扩写 SDK 真实版本与 checksum 规则

