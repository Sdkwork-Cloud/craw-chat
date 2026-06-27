> Migrated from `docs/架构/43-实时Checkpoint不变量归一化标准-2026-04-05.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 43-实时Checkpoint不变量归一化标准-2026-04-05

## 1. 背景

实时 runtime 的 checkpoint 必须长期满足：

- `0 <= ackedThroughSeq <= latestRealtimeSeq`
- `0 <= trimmedThroughSeq <= ackedThroughSeq`

如果 runtime 在以下入口直接接受非法值：

- 从 durable checkpoint store 恢复
- 从 route migration / state handoff 恢复客户端路由状态

就会把不合法的 ack/trim/latest 关系带入内存与持久化层，导致窗口状态失真。

## 2. 标准

### 2.1 runtime 恢复 checkpoint 时必须先归一化

`ensure_client_route_state(...)` 从 checkpoint store 读取到记录后，必须先做不变量收口：

- `ackedThroughSeq = min(ackedThroughSeq, latestRealtimeSeq)`
- `trimmedThroughSeq = min(trimmedThroughSeq, ackedThroughSeq)`

### 2.2 归一化后的状态必须反写 durable store

如果从 store 读到的 checkpoint 已经违反不变量：

- runtime 不仅要在内存中纠正
- 还必须把纠正后的值重新写回 checkpoint store

这样可以避免错误 checkpoint 在后续重启、迁移、扩容中持续传播。

### 2.3 state handoff 恢复时必须做同样归一化

`restore_client_route_state(snapshot)` 在写入 runtime 内存与 checkpoint store 前，必须先归一化：

- `latestRealtimeSeq`
- `ackedThroughSeq`
- `trimmedThroughSeq`

当 snapshot 中存在事件窗口时，`latestRealtimeSeq` 还必须至少不小于事件窗口里的最大 `realtimeSeq`。

### 2.4 runtime 不得写出非法 checkpoint

无论非法值来源于：

- 损坏的 store 记录
- 错误的跨节点 handoff 输入
- 未来组件替换后的非标准适配器

runtime 都不得把这些非法状态继续写回 durable store。

## 3. 落地要求

- `session-gateway` 必须有测试覆盖：
  - 非法 checkpoint 恢复时被自动纠正
  - 非法 device snapshot 恢复时被自动纠正
- 测试不仅验证内存态，还要验证 checkpoint store 中的 durable 值已被纠正

## 4. 与既有标准的关系

- 细化 [24-实时确认点持久化与恢复标准](./24-实时确认点持久化与恢复标准.md) 的 checkpoint 不变量约束
- 与 [19-实时事件窗口确认与裁剪标准](./19-实时事件窗口确认与裁剪标准.md) 的 ack/trim 单调语义保持一致

