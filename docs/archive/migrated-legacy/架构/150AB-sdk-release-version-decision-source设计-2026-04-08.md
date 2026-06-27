# 150AB - SDK release version decision source 设计

## 设计目标

如果 release catalog 只表达“当前没有版本号”，却不表达“未来版本 freeze 决议来自哪里”，后续真实治理证据仍然会缺少稳定挂点。因此需要先引入一个显式但允许为空的决议来源字段。

## 当前最小公开字段

每个 SDK artifact 至少公开：

- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

## 设计原则

- `versionDecisionSourcePath` 是未来 freeze evidence 的挂点，不是当前伪造出来的真值
- 当前真实 freeze evidence 尚未产生时，必须保持 `null`
- bundle 级 `sdk-release-catalog.json` 仍然是唯一 machine-readable 真源

## 为什么现在就要补

- 当前 release catalog 已能回答“有没有版本号”
- 但还不能回答“将来版本 freeze 决议证据会挂在哪”
- 先冻结字段名和空值占位，可以避免后续不同 bundle 各自发明字段

## 非目标

- 不补真实 semver
- 不补真实 freeze 时间
- 不补真实 decision doc path
