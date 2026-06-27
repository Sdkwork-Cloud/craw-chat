# 09Q 实施计划补充: provider-policy 错误词汇收敛

## 对齐范围

- `07-C14`
- `09Q`
- `150Q`

## 目标

- 让 `unknown provider policy version` 在 HTTP 层稳定映射为 `status=conflict`
- 保持 `provider_policy_conflict` 作为未知版本与版本漂移的统一错误码
- 明确 provider-policy routes do not emit `status=not_found`

## 实施点

- 新增路由级回归测试，覆盖 `GET /backend/v3/api/control/provider-policies/diff` 未知版本
- 新增路由级回归测试，覆盖 `POST /backend/v3/api/control/provider-policies/rollback` 未知版本
- 同步 step / review / 架构文档中的错误词汇说明

## 冻结语义

- `unknown provider policy version`
- `status=conflict`
- `code=provider_policy_conflict`
