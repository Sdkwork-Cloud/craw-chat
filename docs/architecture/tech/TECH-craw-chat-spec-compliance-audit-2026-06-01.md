> Migrated from `docs/review/craw-chat-spec-compliance-audit-2026-06-01.md` on 2026-06-24.
> Owner: SDKWork maintainers

# sdkwork-im 规范合规审计（2026-06-01）

## 1) 审计范围

- 应用范围：`apps/sdkwork-im`
- 规范来源：`../../specs`
  - `API_SPEC.md`
  - `DATABASE_SPEC.md`
  - `DOMAIN_SPEC.md`
  - `COMPONENT_SPEC.md`
- 审计目标：数据库设计、模型/领域设计、API 设计、组件技术设计与契约门禁一致性。

## 2) 结论（当前范围）

- `P0` 阻断项：无。
- `P1` 当前应用不合规项：无。
- `P2` 规范一致性优化：已完成检查约束命名前缀统一（`ck_ -> chk_`）并通过回归。

## 3) 本轮已落地修复

1. 数据库检查约束前缀统一为 `chk_`，对齐 `DATABASE_SPEC` 索引/约束命名表。
   - 文件：`deployments/database/postgres/migrations/001_im_core_schema.sql`
2. 数据库契约测试断言同步更新：
   - 文件：`services/session-gateway/tests/database_schema_contract_test.rs`

## 4) 关键规范映射

- API 基线：OpenAPI `3.1.2`（或文档化 `3.1.x`）  
  见 `API_SPEC.md` 中 OpenAPI 文档要求与版本规则。
- 数据库：幂等、追加写、投影、索引、约束命名与一致性  
  见 `DATABASE_SPEC.md` 中命名规则、幂等模板、索引规范。
- 领域模型：按业务能力划分边界，避免歧义域命名  
  见 `DOMAIN_SPEC.md` 域原则与域目录。
- 组件规范：`component.spec.json` 契约化与校验通过  
  见 `COMPONENT_SPEC.md`。

## 5) 回归验证（本轮执行）

- `cargo test -p session-gateway --test database_schema_contract_test` ✅
- `cargo test -p session-gateway --test postgres_realtime_sql_contract_test` ✅
- `cargo test -p sdkwork-im-cloud-gateway --test openapi_im_v3_contract_test` ✅
- `cargo test -p control-plane-api --test openapi_export_test` ✅
- `node sdks/test/verify-im-v3-sdk-family-contract.test.mjs` ✅
- `node ../scripts/validate-component-specs.mjs --apps-root ..` ✅（`892 components, 0 failed`）

## 6) 非阻断说明

- 全仓脚本 `node --test ../scripts/api-spec-java-standard.test.mjs` 存在失败项，
  定位在 `apps/sdkwork-clawrouter`，不在 `apps/sdkwork-im` 范围内，不影响本应用结论。

## 7) 建议下一步

1. 以 `apps/sdkwork-im` 为提交范围，优先提交本次命名一致性修复与契约同步。
2. 另起独立任务处理 `sdkwork-clawrouter` 的全仓 API 路径风格违规，避免与本应用审计混提。

