> Migrated from `docs/架构/40-路由绑定节点存在性与显式拒绝标准-2026-04-05.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 40-路由绑定节点存在性与显式拒绝标准-2026-04-05

## 1. 背景

在跨节点实时路由模型中，`bind_client_route(tenantId, principalId, clientRouteId, ownerNodeId, connectionKind)` 是客户端路由归属写入口。
如果该入口允许在 `ownerNodeId` 不存在时隐式创建 lifecycle，就会出现两类问题：

- 通过数据面写入口制造幽灵节点与幽灵路由
- 控制面已经收紧 `drain/activate` 的节点存在性约束，但数据面仍可绕过该约束，形成语义分裂

因此，节点存在性不能只在控制面校验，路由绑定本身也必须执行同等级别的显式校验。

## 2. 标准

### 2.1 bind 前必须校验节点存在

`bind_client_route(...)` 在写入路由目录之前必须先验证 `ownerNodeId` 已存在于 cluster lifecycle/registry。

- 已知节点：允许继续判定 `drainStatus`
- 未知节点：直接拒绝，不得进入后续 bind 流程

### 2.2 未知节点必须显式返回 `node_not_found`

当 `ownerNodeId` 不存在时：

- cluster bridge 返回错误码 `node_not_found`
- 错误体中的 `nodeId` 必须回传原始 `ownerNodeId`
- 控制面与数据面都使用相同的节点不存在语义

示例：

```json
{
  "code": "node_not_found",
  "message": "node not found: node_missing",
  "nodeId": "node_missing"
}
```

### 2.3 禁止 bind 入口懒创建 lifecycle

`bind_client_route(...)` 不得通过 `ensure_node_lifecycle(ownerNodeId)` 之类的逻辑懒创建 lifecycle。

允许创建 lifecycle 的入口仅限于：

- 节点 runtime/registry 正式注册时
- 明确的节点编排初始化流程

不允许由设备接入、消息投递、路由绑定等业务写入口反向创造节点元数据。

### 2.4 失败 bind 不得产生任何副作用

当 bind 因未知节点被拒绝时，必须同时满足：

- 不写入 `client route ownership`
- 不创建 `node lifecycle`
- 不改变任何已有节点状态

### 2.5 publish fallback 不适用于非法 bind

`publish_scope_event(...)` 的 `origin node fallback` 仅适用于“无路由”场景，不适用于“非法 ownerNodeId 的 bind 被拒绝”场景。

换言之：

- “无 route” 可以 fallback
- “坏 bind” 必须显式失败，不能通过 fallback 合法化

## 3. 落地要求

- `session-gateway` 回归测试必须覆盖未知节点 bind 拒绝
- 回归测试必须验证失败 bind 后 route 与 lifecycle 都不存在
- 后续若引入独立 route directory service，该标准仍然保持不变

## 4. 与既有标准的关系

- 对 [23-节点排空与路由迁移控制标准](./23-节点排空与路由迁移控制标准.md) 中的 bind 约束做更严格落地
- 与 [39-控制面节点存在性与幽灵节点防护标准-2026-04-05](./39-控制面节点存在性与幽灵节点防护标准-2026-04-05.md) 形成控制面/数据面一致性闭环

