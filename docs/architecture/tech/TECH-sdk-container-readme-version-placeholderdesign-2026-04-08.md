> Migrated from `docs/架构/150Z-sdk-container-readme-version-placeholder设计-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150Z - SDK container README version placeholder 设计

## 设计目标

SDK 发布边界如果只在根入口和叶子入口表达，而中间层容器 README 不表达，消费者仍会在导航过程中丢失版本冻结语义。因此容器 README 也必须公开同一组版本占位 contract。

## 当前最小公开字段

每个容器 README 至少公开：

- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`

## 设计原则

- 容器 README 不拥有自己的版本真源
- bundle 级 `sdk-release-catalog.json` 仍然是唯一真源
- 容器 README 的职责是把该占位语义沿导航链继续公开，而不是二次定义版本状态

## 为什么要补容器层

- 当前 SDK 导航链是三层：
  - workspace 根入口
  - app/admin 容器入口
  - 具体语言叶子入口
- 若中间层缺失版本占位，读者在进入 app/admin README 时仍会看到一层没有版本语义的断点

## 非目标

- 不新增真实 semver
- 不新增 freeze 时间或 release note
- 不引入新的 machine-readable 产物

