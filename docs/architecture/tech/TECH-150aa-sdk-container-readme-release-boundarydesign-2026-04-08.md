> Migrated from `docs/架构/150AA-sdk-container-readme-release-boundary设计-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150AA - SDK container README release boundary 设计

## 设计目标

SDK 发布边界如果只在 bundle、workspace 根入口和叶子 README 表达，而容器 README 不表达，消费者在进入 app/admin 中间层时仍会失去对 bundle 发布真源的感知。因此容器 README 也必须公开同一套 release catalog 边界。

## 当前最小公开字段

每个容器 README 至少公开：

- `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
- `generationStatus = template_only_pending_generation`
- `releaseStatus = not_published`

## 设计原则

- 容器 README 不拥有自己的发布真源
- bundle 级 `sdk-release-catalog.json` 仍然是唯一真源
- 容器 README 的职责是沿导航链继续公开发布边界，而不是重新定义 SDK 生成或发布状态

## 为什么要补容器层

- 当前 SDK 导航链是三层：
  - workspace 根入口
  - app/admin 容器入口
  - 具体语言叶子入口
- 若中间层没有 release catalog 边界，读者进入 app/admin README 时仍会看到一层发布语义断点

## 非目标

- 不新增真实 semver
- 不新增真实 publish timestamp
- 不引入新的 machine-readable 产物

