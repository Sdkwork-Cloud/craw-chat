> Migrated from `docs/架构/150L-control-plane-provider-policy-noop设计-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150L control-plane provider-policy no-op 设计
## 1. 背景

`07-C8` 已经把 committed 结果回显补齐，但相同 policy 的重复提交仍会继续追加 history 和版本。这会制造无意义版本噪声，也会让 audit 与运维视图出现无实际变化的写入记录。

## 2. 目标

- 对相同 policy state 的重复提交做 no-op 抑制。
- 在成功响应里显式告诉调用方这是 no-op。
- 保持冲突与真实写入语义不变。

## 3. 非目标

- 不引入新的 HTTP 状态码。
- 不改变 preview 响应结构。
- 不在本轮处理状态枚举统一。

## 4. 模型

- `ProviderPolicyCommit`
  - `applied`
  - `currentVersion`
  - `committedBinding`
  - `diff`

其中：

- `applied=true` 表示本次真实写入并推进版本。
- `applied=false` 表示本次请求是 no-op。

## 5. 判定规则

- 只比较真实 policy state：
  - deployment profile 看当前 `deployment_profiles[domain]`
  - tenant override 看当前 `tenant_overrides[tenantId][domain]`
- 当前 state 已经等于目标 plugin 时，判定为 no-op。
- 仅仅“effective binding 恰好相同”但 policy source 不同，不算 no-op。

## 6. HTTP 面

- 路径: `POST /backend/v3/api/control/provider_bindings`
- 权限: `control.write`
- Success Response:
  - `applied`
  - `currentVersion`
  - `committedBinding`
  - `diff`

## 7. 副作用约束

- no-op 不得追加 history。
- no-op 不得增加版本。
- no-op 不得写 audit。
- no-op 不得刷新 ops。

## 8. 演进关系

- `07-C7` 解决 confirm。
- `07-C8` 解决 committed 回显。
- `07-C9` 解决重复提交噪声。
- 下一轮应统一 preview / conflict / noop / applied 的结果表达，减少调用方分支判断成本。

