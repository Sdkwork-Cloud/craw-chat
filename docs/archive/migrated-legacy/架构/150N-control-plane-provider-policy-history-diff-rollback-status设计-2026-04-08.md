# 150N control-plane provider-policy history diff rollback status 设计
## 1. 背景

`07-C10` 已统一 `preview / applied / noop` 的成功状态，但 `GET /backend/v3/api/control/provider-policies`、`GET /backend/v3/api/control/provider-policies/diff`、`POST /backend/v3/api/control/provider-policies/rollback` 仍然没有显式 `status`，调用方依然需要通过路由语义推断当前结果类型。

## 2. 目标

- 让 provider-policy 所有成功路径都带显式 `status`。
- 为历史、diff、回滚冻结稳定状态值:
  - `history`
  - `diff`
  - `rolled_back`
- 保持既有字段结构尽量不变。

## 3. 非目标

- 不修改 `ProviderPolicyHistory` 与 `ProviderPolicyDiff` contract。
- 不统一错误 envelope。
- 不改变 rollback 的业务副作用链路。

## 4. HTTP 面

- `GET /backend/v3/api/control/provider-policies`
  - 返回 `status=history`
- `GET /backend/v3/api/control/provider-policies/diff`
  - 返回 `status=diff`
- `POST /backend/v3/api/control/provider-policies/rollback`
  - 返回 `status=rolled_back`

其余字段继续平铺输出：

- history / rolled_back:
  - `currentVersion`
  - `items`
- diff:
  - `fromVersion`
  - `toVersion`
  - `deploymentProfileChanges`
  - `tenantOverrideChanges`

## 5. 设计取舍

- 选择在 control-plane API 层做响应包装，而不是把 `status` 下沉到 contract。
- 这样可以保持底层 contract 纯粹，避免为 HTTP 路由语义污染共享模型。

## 6. 演进关系

- `07-C10` 收口 `preview / applied / noop`
- `07-C11` 收口 `history / diff / rolled_back`
- 完成后，provider-policy success 路径已经具备统一结果状态
- 下一轮只剩 error 路径是否继续统一的问题
