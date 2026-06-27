> Migrated from `docs/架构/150AC-sdk-leaf-readme-version-decision-source设计-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150AC - SDK leaf README version decision source 设计

## 设计目标

当 bundle 级 release catalog 已经拥有 `versionDecisionSourcePath`，叶子 README 仍然不表达该字段时，具体语言消费者会在最靠近使用的位置失去这条治理语义。因此叶子 README 也必须公开同一占位字段。

## 当前最小公开字段

每个叶子 README 至少公开：

- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

## 设计原则

- 叶子 README 不发明自己的决议来源
- `sdk-release-catalog.json` 仍然是唯一 machine-readable 真源
- 叶子 README 的职责是沿导航链继续公开已有占位语义

## 为什么先补叶子层

- 叶子 README 是具体语言消费者最直接的入口
- 如果这里只能看到“没有版本号”，却看不到“决议来源尚未分配”，说明导航链仍未闭环

## 非目标

- 不补真实 freeze doc path
- 不补真实生成产物
- 不新增额外 schema

