# 150O control-plane provider-policy error status 设计
## 1. 背景

`07-C10` 与 `07-C11` 已经把 provider-policy success 路径统一到显式 `status`，但失败响应仍然只有 `code + message`。调用方在 error 路径仍需额外解释 HTTP 状态码，success/error 的消费方式不一致。

## 2. 目标

- 为 provider-policy 失败响应补稳定 `status`
- 保持已有 `code / message` 兼容
- 通过粗粒度分类降低调用方分支判断成本

## 3. 非目标

- 不移除 `code`
- 不重构成新的统一 envelope 类型
- 不改变 provider-policy 现有 HTTP 语义

## 4. 错误状态分类

- `unauthorized`
- `forbidden`
- `invalid`
- `conflict`
- `unavailable`

扩展兼容:

- `not_found`

## 5. 映射规则

- `401` -> `unauthorized`
- `403` -> `forbidden`
- `400` -> `invalid`
- `409` -> `conflict`
- `503` -> `unavailable`
- `404` -> `not_found`

## 6. provider-policy 关键场景

- `POST /backend/v3/api/control/provider_bindings`
  - `invalid_provider_policy` -> `status=invalid`
  - `provider_policy_conflict` -> `status=conflict`
  - `provider_policy_write_unavailable` -> `status=unavailable`
- `POST /backend/v3/api/control/provider-policies/preview`
  - `provider_policy_preview_unavailable` -> `status=unavailable`
- `GET /backend/v3/api/control/provider-policies`
  - `provider_policy_history_unavailable` -> `status=unavailable`
- `GET /backend/v3/api/control/provider-policies/diff`
  - `provider_policy_diff_unavailable` -> `status=unavailable`
- `POST /backend/v3/api/control/provider-policies/rollback`
  - `provider_policy_rollback_unavailable` -> `status=unavailable`
- 权限不足
  - `permission_denied` -> `status=forbidden`
- 缺失鉴权上下文
  - `auth_context_missing` -> `status=unauthorized`

## 7. 设计取舍

- 选择在 `ControlPlaneError` 层统一补 `status`
- 这样 provider-policy error 能收口，同时不会为每条路由重复造错误模型
- `code` 继续承载细粒度原因，`status` 负责稳定分类

## 8. 演进关系

- `07-C10` 收口 success: `preview / applied / noop`
- `07-C11` 收口 success: `history / diff / rolled_back`
- `07-C12` 收口 error: `invalid / conflict / unavailable / forbidden / unauthorized`
- 至此 provider-policy success/error 两侧都已经具备显式 `status`
## 9. 补充校正

- provider-policy routes do not emit `status=not_found`
- `unknown provider policy version` 表示同一条 provider-policy 版本流上的冲突，不是独立资源缺失
- `GET /backend/v3/api/control/provider-policies/diff` 与 `POST /backend/v3/api/control/provider-policies/rollback` 命中未知版本时，保持 `provider_policy_conflict + status=conflict`
