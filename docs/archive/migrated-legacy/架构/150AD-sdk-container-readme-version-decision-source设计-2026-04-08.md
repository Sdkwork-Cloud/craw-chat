# 150AD - SDK container README version decision source 设计

## 设计目标

如果 root 和 leaf 都表达了 `versionDecisionSourcePath`，但容器 README 不表达，导航链中间层仍然会重新丢失这条治理语义。因此容器 README 也必须公开该占位字段。

## 当前最小公开字段

每个容器 README 至少公开：

- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

## 设计原则

- 容器 README 不发明新的决议来源
- `sdk-release-catalog.json` 仍然是唯一 machine-readable 真源
- 容器 README 的职责是沿导航链继续公开已有占位语义

## 为什么必须补容器层

- 当前 SDK 导航链是：
  - root README
  - app/admin 容器 README
  - 语言叶子 README
- 如果中间层没有 `versionDecisionSourcePath`，导航链就不闭环

## 非目标

- 不补真实 freeze evidence path
- 不补真实版本号
- 不新增新的 catalog 产物
