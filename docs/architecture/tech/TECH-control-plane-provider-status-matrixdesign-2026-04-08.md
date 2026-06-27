> Migrated from `docs/架构/150R-control-plane-provider-status-matrix设计-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150R control-plane provider status matrix 设计

## 1. 背景

`07-C10` 到 `07-C14` 已分步补齐 provider-policy 与 provider snapshot 的显式 `status`。下一步需要一份集中状态矩阵，避免消费者继续依赖零散测试或字段推断。

## 2. 固定词汇

- `registry`
- `bindings`
- `preview`
- `applied`
- `noop`
- `history`
- `diff`
- `rolled_back`
- `invalid`
- `conflict`
- `unavailable`
- `forbidden`
- `unauthorized`

## 3. 设计结论

- `registry / bindings / preview / applied / noop / history / diff / rolled_back` 覆盖成功与读面
- `invalid / conflict / unavailable / forbidden / unauthorized` 覆盖错误与鉴权面
- 由 `provider_status_contract_test` 统一冻结该矩阵

## 4. 交付锚点

- `07-C15`
- `09R`
- `150R`

