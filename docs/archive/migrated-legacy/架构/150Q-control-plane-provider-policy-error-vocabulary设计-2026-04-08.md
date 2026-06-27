# 150Q control-plane provider-policy error vocabulary 设计

## 1. 背景

`07-C12` 已统一 provider-policy error `status`，但旧文档仍把 `not_found` 写进 provider-policy 词汇表。实际运行时并非如此：未知版本来自同一条 policy version stream 的并发/状态冲突，而不是资源不存在。

## 2. 收敛目标

- `GET /backend/v3/api/control/provider-policies/diff`
- `POST /backend/v3/api/control/provider-policies/rollback`
- `unknown provider policy version`
- `status=conflict`
- `code=provider_policy_conflict`
- provider-policy routes do not emit `status=not_found`

## 3. 设计结论

- provider-policy 的错误词汇保持 `invalid / conflict / unavailable / forbidden / unauthorized`
- 未知版本继续归入 `conflict`
- `not_found` 仍可保留在通用控制面错误枚举中，但不属于 provider-policy 路径集合

## 4. 交付锚点

- `07-C14`
- `09Q`
- `150Q`
