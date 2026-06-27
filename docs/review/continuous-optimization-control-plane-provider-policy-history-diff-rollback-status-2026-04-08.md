# Continuous Optimization: control-plane provider-policy history diff rollback status

## 本轮交付

- 新增 `07-C11 / 09N / 150N` 文档闭环。
- `GET /backend/v3/api/control/provider-policies` 返回 `status=history`。
- `GET /backend/v3/api/control/provider-policies/diff` 返回 `status=diff`。
- `POST /backend/v3/api/control/provider-policies/rollback` 返回 `status=rolled_back`。
- 底层 `ProviderPolicyHistory / ProviderPolicyDiff` contract 保持不变。

## 验证

- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 评价

- 这一轮把 provider-policy 读取与回滚成功响应也纳入显式 `status`。
- 现在 success 路径已经能稳定区分 `preview / applied / noop / history / diff / rolled_back`。
- 仍未统一的是 error 路径，这构成下一轮最自然的缺口。

## 下一步

- 下一轮优先评估是否为 provider-policy 冲突、不可用、参数错误补稳定错误分类，完成 success/error 的整体收口。
