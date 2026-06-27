# Step 07-C7: 控制面 provider-policy 预览确认写入闭环
## 当前闭环编号

- 所属 step: `Step 07`
- 当前波次: `07-C7 / CP07-7A`
- 目标: 把 `07-C6` 的 preview 接到真实写接口，在提交时引入 `expectedBaseVersion`，拒绝 preview 之后发生的并发漂移。

## 本轮实现

- `RuntimeProviderRegistry` 新增：
  - `set_deployment_profile_with_expected_version(...)`
  - `set_tenant_override_with_expected_version(...)`
- `POST /backend/v3/api/control/provider_bindings` 请求体新增可选 `expectedBaseVersion`。
- 若当前真实版本与 `expectedBaseVersion` 不一致，返回 `409`。
- 冲突错误码固定为 `provider_policy_conflict`。
- 冲突消息固定描述 `provider policy version drift: expected X, current Y`。
- 冲突时保持无副作用：
  - 不追加 history
  - 不刷新 ops
  - 不写 audit

## 接口冻结

- 路径: `POST /backend/v3/api/control/provider_bindings`
- 权限: `control.write`
- Body:
  - `tenantId`
  - `domain`
  - `pluginId`
  - `expectedBaseVersion`
- 冲突响应:
  - `409`
  - `code = provider_policy_conflict`
  - `message` 包含 expected/current 版本

## 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`

## 文档同步

- `docs/step/07-C7-控制面provider-policy预览确认写入闭环-2026-04-08.md`
- `docs/架构/09J-实施计划-provider-policy预览确认补充-2026-04-08.md`
- `docs/架构/150J-control-plane-provider-policy预览确认设计-2026-04-08.md`
- `docs/review/continuous-optimization-control-plane-provider-policy-preview-confirmation-2026-04-08.md`

## 下一缺口

- 下一轮优先进入 `07-C8`，给成功写入回包补齐提交后的 `currentVersion / committedBinding / diff` 视图，形成 preview -> confirm -> committed 的完整确认链路。
