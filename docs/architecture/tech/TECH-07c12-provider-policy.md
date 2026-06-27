> Migrated from `docs/step/07-C12-控制面provider-policy错误状态闭环-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 07-C12: 控制面 provider-policy 错误状态闭环
## 当前闭环编号

- 所属 step: `Step 07`
- 当前波次: `07-C12 / CP07-12A`
- 目标: 为 provider-policy error 路径补统一 `status` 分类，让调用方在失败场景也能稳定消费。

## 本轮实现

- `services/control-plane-api/src/lib.rs`
  - `ControlPlaneError` 响应新增 `status`
  - 按 HTTP 语义冻结错误状态:
    - `invalid`
    - `conflict`
    - `unavailable`
    - `forbidden`
    - `unauthorized`
- 现有 `code` 与 `message` 保持不变
- 现有 HTTP 状态码保持不变

## 接口冻结

- `POST /backend/v3/api/control/provider_bindings`
  - cross-domain plugin id -> `status=invalid`
  - stale expectedBaseVersion -> `status=conflict`
  - runtime 未启用 -> `status=unavailable`
- `POST /backend/v3/api/control/provider-policies/preview`
  - runtime 未启用 -> `status=unavailable`
  - 无权限 -> `status=forbidden`
- `GET /backend/v3/api/control/provider-policies`
  - runtime 未启用 -> `status=unavailable`
- `GET /backend/v3/api/control/provider-policies/diff`
  - runtime 未启用 -> `status=unavailable`
- `POST /backend/v3/api/control/provider-policies/rollback`
  - runtime 未启用 -> `status=unavailable`
- 公网入口缺失合法鉴权 -> `status=unauthorized`

## 验证

- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 文档同步

- `docs/step/07-C12-控制面provider-policy错误状态闭环-2026-04-08.md`
- `docs/架构/09O-实施计划-provider-policy错误状态补充-2026-04-08.md`
- `docs/架构/150O-control-plane-provider-policy-error-status设计-2026-04-08.md`
- `docs/review/continuous-optimization-control-plane-provider-policy-error-status-2026-04-08.md`

## 下一缺口

- 下一轮可进入 `07-C13`，评估是否把 provider-policy 的成功/失败响应进一步统一为同一结果 envelope，或在当前成功/失败都具备 `status` 的基础上冻结边界。
## 补充校正

- `unknown provider policy version` 仍然属于 provider-policy 版本流冲突，返回 `provider_policy_conflict`
- provider-policy routes do not emit `status=not_found`
- `GET /backend/v3/api/control/provider-policies/diff` 与 `POST /backend/v3/api/control/provider-policies/rollback` 遇到未知版本时，保持 `status=conflict`

