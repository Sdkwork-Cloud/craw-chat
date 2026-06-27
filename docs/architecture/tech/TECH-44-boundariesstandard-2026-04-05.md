> Migrated from `docs/架构/44-实时窗口恢复排序去重与裁剪边界标准-2026-04-05.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 44-实时窗口恢复排序去重与裁剪边界标准-2026-04-05

## 1. 背景

`restore_client_route_state(snapshot)` 用于 route migration、runtime handoff、节点切换后的客户端路由状态恢复。  
此前标准已经约束了 checkpoint 不变量：

- `0 <= ackedThroughSeq <= latestRealtimeSeq`
- `0 <= trimmedThroughSeq <= ackedThroughSeq`

但如果 handoff 输入中的 `snapshot.events` 本身出现乱序、重复序号，或者仍然携带 `realtimeSeq <= trimmedThroughSeq` 的旧窗口事件，恢复后的 `list_events` 会破坏单调翻页语义，导致客户端重复收取或重新看到已裁剪事件。

## 2. 标准

### 2.1 恢复前必须归一化事件窗口

`restore_client_route_state(snapshot)` 在把 `snapshot.events` 写入 runtime 内存前，必须执行以下归一化：

1. 丢弃所有 `realtimeSeq <= trimmedThroughSeq` 的事件
2. 按 `realtimeSeq` 升序重建窗口
3. 对相同 `realtimeSeq` 的重复事件只保留一个确定性结果

实现可以选择“最后一个覆盖前一个”或“第一个保留后续丢弃”，但必须是稳定且可测试的确定性规则，不能把重复序号原样写回窗口。

### 2.2 `list_events` 必须保持单调分页

当客户端按 `afterSeq` 分页拉取窗口时：

- 返回的 `items.realtimeSeq` 必须严格升序
- `nextAfterSeq` 必须等于本页最后一个事件的 `realtimeSeq`
- 后一页不能再次返回前一页已经返回过的序号

### 2.3 裁剪边界与窗口内容必须一致

当恢复后的 checkpoint 已经声明：

- `ackedThroughSeq = x`
- `trimmedThroughSeq = y`

则恢复后的窗口中不能再保留 `realtimeSeq <= y` 的事件。  
否则 `trimmedThroughSeq` 就不再表示“已经实际裁剪到的边界”，会破坏 ack/trim 语义。

## 3. 落地要求

- `session-gateway` 的 `RealtimeDeliveryRuntime::restore_client_route_state(...)` 必须执行窗口归一化
- 回归测试至少覆盖：
  - 乱序 handoff 后分页仍然单调
  - 重复 `realtimeSeq` 不会重复出现在窗口中
  - `trimmedThroughSeq` 以内的旧事件不会被恢复回窗口

## 4. 关联标准

- [19-实时事件窗口确认与裁剪标准](./19-实时事件窗口确认与裁剪标准.md)
- [24-实时确认点持久化与恢复标准](./24-实时确认点持久化与恢复标准.md)
- [43-实时Checkpoint不变量归一化标准-2026-04-05](./43-实时Checkpoint不变量归一化标准-2026-04-05.md)

