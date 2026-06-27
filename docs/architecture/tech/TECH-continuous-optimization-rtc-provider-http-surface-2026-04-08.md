> Migrated from `docs/review/continuous-optimization-rtc-provider-http-surface-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - RTC Provider HTTP Surface - 2026-04-08

## 本轮目标
在 Step 06 / Wave B 中把 RTC provider 能力从 runtime 内部闭环推进到外部 HTTP surface，补齐 credential 与 provider health。

## 实际完成
- `services/im-call-runtime/src/lib.rs`
  - 新增 `IssueRtcParticipantCredentialRequest`
  - 新增 `POST /im/v3/api/calls/sessions/{rtc_session_id}/credentials`
  - 新增 `GET /backend/v3/api/rtc/provider_health`
  - 对外复用 `RtcRuntime::issue_participant_credential(...)` 与 `provider_health_snapshot(...)`
- `services/sdkwork-im-cloud-gateway/src/node/rtc.rs`
  - 新增同等能力的本地节点 handler
- `services/sdkwork-im-cloud-gateway/src/node/build.rs`
  - 暴露与 `im-call-runtime` 对齐的两条 RTC provider surface 路由
- `services/im-call-runtime/tests/http_smoke_test.rs`
  - 新增 credential / provider health HTTP red-green tests
- `services/sdkwork-im-cloud-gateway/tests/http_e2e_test.rs`
  - 新增 credential / provider health e2e tests

## 验证结果
以下命令已重新执行并通过：
- `cargo fmt --all`
- `cargo fmt --all --check`
- `$env:CARGO_TARGET_DIR='C:\Users\admin\.codex\memories\target-rtc-surface-service'; cargo test -p im-call-runtime --offline --test http_smoke_test -- --nocapture`
- `$env:CARGO_TARGET_DIR='C:\Users\admin\.codex\memories\target-rtc-surface-service'; cargo test -p im-call-runtime --offline --test rtc_runtime_persistence_test -- --nocapture`
- `$env:CARGO_TARGET_DIR='C:\Users\admin\.codex\memories\target-rtc-surface-local'; cargo test -p sdkwork-im-cloud-gateway --offline --test http_e2e_test test_local_minimal_profile_issues_rtc_participant_credential_over_http -- --nocapture`
- `$env:CARGO_TARGET_DIR='C:\Users\admin\.codex\memories\target-rtc-surface-local'; cargo test -p sdkwork-im-cloud-gateway --offline --test http_e2e_test test_local_minimal_profile_gets_rtc_provider_health_over_http -- --nocapture`
- `$env:CARGO_TARGET_DIR='C:\Users\admin\.codex\memories\target-rtc-surface-local'; cargo test -p sdkwork-im-cloud-gateway --offline --test rtc_runtime_persistence_test -- --nocapture`
- `$env:CARGO_TARGET_DIR='C:\Users\admin\.codex\memories\target-rtc-surface-local'; cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

说明：由于 `D:` 盘构建缓存空间耗尽，本轮测试使用 `CARGO_TARGET_DIR` 切换到 `C:` 盘独立缓存目录完成验证。

## 当前判断
- Step 06 / Wave B 已从“runtime provider 已接入”推进到“session + credential + health 外部 surface 已闭环”。
- `im-call-runtime` 与 `sdkwork-im-server` 现在对外暴露一致的 RTC provider surface。
- 下一轮最优优先级仍是 `callback / artifact` surface，或转入 `object-storage-s3` baseline。

