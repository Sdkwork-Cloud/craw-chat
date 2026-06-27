> Migrated from `docs/架构/39-控制面节点存在性与幽灵节点防护标准-2026-04-05.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 39-控制面节点存在性与幽灵节点防护标准-2026-04-05

## 1. 目标

本标准用于冻结 IM 集群控制面在节点生命周期写接口上的节点存在性约束，避免 `drain`、`activate` 等控制命令通过隐式写入生成“幽灵节点”。

适用范围：

- `POST /backend/v3/api/control/nodes/{nodeId}/drain`
- `POST /backend/v3/api/control/nodes/{nodeId}/activate`
- 后续任何会修改节点 lifecycle 的控制面写接口

## 2. 标准要求

### 2.1 lifecycle 写接口不得隐式创建节点

控制面写接口只能作用于已存在节点。这里的“已存在”至少意味着：

- 节点已经在 cluster runtime/registry 中注册，或
- 节点已经存在合法的 lifecycle 记录，或
- 节点已经通过受控流程出现在 route ownership 目录中

禁止以下行为：

- 调用 `drain(node_missing)` 时自动创建 `node_missing`
- 调用 `activate(node_missing)` 时自动创建 `node_missing`
- 通过控制面写接口补出仅存在 lifecycle、但不在真实拓扑中的伪节点

### 2.2 未知节点必须显式返回 `node_not_found`

当调用方对未知节点执行 lifecycle 写操作时：

- 核心状态机必须返回 `node_not_found`
- HTTP API 必须返回 `404 Not Found`
- 返回体中的错误码必须稳定，便于运维编排识别

示例：

```json
{
  "code": "node_not_found",
  "message": "node not found: node_missing"
}
```

## 3. 为什么必须这样

若允许控制面写接口隐式创建节点，会导致：

- 运维系统误以为节点已存在并参与拓扑
- `ops/cluster` 与 `ops/diagnostics` 出现不真实的 lifecycle 视图
- drain/migrate 编排可能对一个并不存在的节点继续执行后续操作
- SaaS 与私有化环境下的自动化运维无法区分“真实节点异常”与“错误节点 ID”

这会直接破坏商业环境中对集群状态可观测性和可编排性的要求。

## 4. 落地要求

- 核心 cluster bridge 必须先校验节点存在性，再允许修改 lifecycle
- `drain` 与 `activate` 的回归测试必须覆盖未知节点场景
- 控制面 HTTP 回归测试必须覆盖未知节点返回 `404` 的场景
- 后续若引入独立 route directory service 或 commercial control plane，本标准保持不变

