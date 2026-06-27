# 09D 实施计划：ops provider binding消费补充

## 目标

- 把控制面已计算出的 `effective provider bindings` 接到 `ops-service` 的只读消费链路。
- 形成 `control-plane -> ops-runtime -> ops-service` 的一致快照，不另起一套 provider 求值逻辑。

## 实施范围

1. `OpsRuntime` 新增 provider binding snapshot 存储能力
2. `ops-service` 新增 `GET /backend/v3/api/ops/provider_bindings`
3. `diagnostic bundle` 携带 `providerBindings`
4. `control-plane-api` 在 provider binding 查询时，把同一份求值结果镜像到 `OpsRuntime`

## 最小契约

`ops-service` 只读视图最小包含：

- `interfaceVersion`
- `tenantId`
- `effectiveBindings`
- `precedence`

镜像规则：

- 不在 ops 侧重新计算 provider 选择
- 不在 ops 侧维护另一套优先级
- 仍以控制面返回结果为准

## 实施步骤

1. 先写 `ops-service` runtime 红测，冻结 snapshot 存储与 diagnostics 输出
2. 再写 `ops-service` HTTP 红测与 public auth 红测
3. 再写 `control-plane -> ops-runtime` 治理联动红测
4. 最后用最小实现打通镜像链路

## 验证命令

- `cargo test -p ops-service --offline --test ops_runtime_test -- --nocapture`
- `cargo test -p ops-service --offline --test http_smoke_test -- --nocapture`
- `cargo test -p ops-service --offline --test public_auth_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`

## 收口标准

- `ops-service` 能返回 provider binding 快照
- diagnostics 中能看到 `providerBindings`
- tenant override 与 deployment profile 的镜像结果能被 ops 看到
- public app 仍要求 `ops.read`
