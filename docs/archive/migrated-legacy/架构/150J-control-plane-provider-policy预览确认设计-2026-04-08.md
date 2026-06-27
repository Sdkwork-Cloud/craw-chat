# 150J control-plane provider-policy 预览确认设计
## 1. 背景

`07-C6` 已经提供 `POST /backend/v3/api/control/provider-policies/preview`，但 preview 返回的 `baseVersion` 还没有被真实写接口消费。调用方即使先预览，再提交，也无法识别 preview 与 commit 之间是否发生了并发写入。

## 2. 目标

- 让 `POST /backend/v3/api/control/provider_bindings` 支持 `expectedBaseVersion`。
- 在真实版本漂移时拒绝写入，而不是覆盖最新状态。
- 保持旧调用兼容。
- 保持冲突失败无副作用。

## 3. 非目标

- 不引入 preview token 或租约。
- 不处理批量 policy 事务确认。
- 不在本轮改变 preview 的响应结构。

## 4. 模型与语义

- 写请求新增 `expectedBaseVersion`。
- 当 `expectedBaseVersion` 缺失时，沿用原有立即写入语义。
- 当 `expectedBaseVersion` 存在时，真实当前版本必须与其一致。
- 若不一致，返回 `provider_policy_conflict`，消息为 `provider policy version drift: expected X, current Y`。

## 5. 写入规则

- 先做 plugin/domain 合法性校验。
- 再读取当前真实 policy 版本。
- 若 `expectedBaseVersion` 不匹配当前版本，则直接失败。
- 只有匹配时才允许真正写入并追加 history snapshot。

## 6. HTTP 面

- 路径: `POST /backend/v3/api/control/provider_bindings`
- 权限: `control.write`
- Body:
  - `tenantId`
  - `domain`
  - `pluginId`
  - `expectedBaseVersion`
- Conflict:
  - HTTP `409`
  - `code = provider_policy_conflict`
  - `message` 含 expected/current 版本

## 7. 副作用约束

- stale write 不得追加 history。
- stale write 不得写 audit。
- stale write 不得触发 `replace_provider_binding_snapshots`。
- stale write 不得改变当前 `effectiveBinding`。

## 8. 演进关系

- `07-C6` 解决“提交前看到什么”。
- `07-C7` 解决“按看到的版本安全提交”。
- 下一轮应补 committed 结果回显，把 preview、confirm、commit 串成完整闭环。
