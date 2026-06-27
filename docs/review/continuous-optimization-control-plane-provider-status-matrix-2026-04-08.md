# Continuous Optimization: control-plane provider status matrix

## 结论

- 新增 `07-C15 / 09R / 150R`
- 新增集中回归测试，冻结 provider control-plane 顶层状态矩阵

## 状态矩阵

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

## 验证

- `cargo test -p control-plane-api --offline --test provider_status_contract_test -- --nocapture`
