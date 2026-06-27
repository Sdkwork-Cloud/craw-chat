> Migrated from `docs/step/07-C6-控制面provider-policy写前预览闭环-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 07-C6: 控制面 provider-policy 写前预览闭环
## 当前闭环编号

- 所属 step: `Step 07`
- 当前波次: `07-C6 / CP07-6A`
- 目标: 在 `07-C5` 差异查询基础上，补齐 provider policy 写前预览，支持控制面在真正提交前看到预计变更。

## 本轮实现

- `RuntimeProviderRegistry` 新增 `ProviderPolicyPreview`。
- 新增 `preview_upsert(tenantId, domain, pluginId)`，复用现有校验逻辑做虚拟预演。
- control-plane 新增 `POST /backend/v3/api/control/provider-policies/preview`。
- 请求体与 `POST /backend/v3/api/control/provider_bindings` 保持一致。
- 响应固定返回：
  - `baseVersion`
  - `previewVersion`
  - `tenantId`
  - `previewBinding`
  - `diff`
- preview 只做虚拟计算，**无副作用**：
  - 不增加真实版本
  - 不改当前 history
  - 不刷新 ops
  - 不写 audit

## 接口冻结

- 路径: `POST /backend/v3/api/control/provider-policies/preview`
- 权限: `control.write`
- Body:
  - `tenantId`
  - `domain`
  - `pluginId`
- Response:
  - `baseVersion`
  - `previewVersion`
  - `previewBinding`
  - `diff`

## 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`

## 文档同步

- `docs/step/07-C6-控制面provider-policy写前预览闭环-2026-04-08.md`
- `docs/架构/09I-实施计划-provider-policy预览补充-2026-04-08.md`
- `docs/架构/150I-control-plane-provider-policy预览设计-2026-04-08.md`
- `docs/review/continuous-optimization-control-plane-provider-policy-preview-2026-04-08.md`

## 下一缺口

- 下一轮优先进入 `07-C7`，给真实写接口补 `expectedBaseVersion` 或等价的 preview 确认约束，避免 preview 与真正提交之间出现并发漂移。

