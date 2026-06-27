# 150Y - SDK leaf README version placeholder 设计

## 设计目标

具体语言 README 不仅要公开“未生成 / 未发布”，还要公开“版本尚未冻结”，否则消费方仍可能把 README 占位误读成已有计划版本。

## 当前最小公开字段

每个叶子 README 至少应公开：

- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`

## 设计原则

- 叶子 README 不生成自己的版本真源
- bundle catalog 仍然是唯一版本冻结真源
- 叶子 README 只负责把该占位状态对外公开

## 为什么必须补这一层

- 具体消费者最容易直接打开叶子目录
- 若只在根 README 和 bundle 公开版本占位，具体语言入口仍可能被误读为“版本只是没写出来”
- 显式写出 `plannedVersion = null` 才能避免隐式猜测

## 非目标

- 不在叶子 README 中新增真实 semver
- 不在叶子 README 中新增 freeze 时间或 changelog
- 不让叶子 README 脱离 bundle catalog 自行维护版本状态
