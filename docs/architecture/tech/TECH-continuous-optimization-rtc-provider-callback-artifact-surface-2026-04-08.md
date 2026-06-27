> Migrated from `docs/review/continuous-optimization-rtc-provider-callback-artifact-surface-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# RTC Provider Callback / Artifact Surface Continuous Optimization Review

日期：2026-04-08

## 本轮目标

补齐 Step 06 / Wave B 中 RTC provider 外部能力面的最后两个缺口：

- provider callback ingest
- recording artifact export

## 变更摘要

- `services/im-call-runtime/src/lib.rs`
  - 新增 `RtcRuntime::map_provider_callback(...)`
  - 新增 `RtcRuntime::recording_artifact(...)`
  - 新增 `POST /backend/v3/api/rtc/provider_callbacks`
  - 新增 `Retired recording artifact HTTP read; call artifacts are delivered as Drive-backed IM records`
- `services/sdkwork-im-cloud-gateway/src/node/rtc.rs`
  - 新增 callback handler
  - 新增 recording artifact handler
- `services/sdkwork-im-cloud-gateway/src/node/build.rs`
  - 镜像暴露同名 RTC provider surface

## 设计决策

- callback 面继续使用 provider-agnostic contract：
  - `RtcCallbackRequest`
  - `RtcCallbackEvent`
- artifact 面继续使用 provider contract：
  - `RtcRecordingArtifact`
- 不在 domain 中引入任何厂商 SDK DTO。
- standalone `im-call-runtime` 中：
  - callback 作为 provider/integration surface，不复用 standalone conversation gateway 限制。
  - artifact 作为 session resource surface，继续沿用 standalone session 访问边界。
- `sdkwork-im-server` 中：
  - artifact 继续复用 conversation write guard，避免 conversation-bound RTC session 绕过本地授权边界。
  - callback 不复用 member mutation guard，保持 provider callback 与用户会话动作分离。

## 测试覆盖

- `services/im-call-runtime/tests/http_smoke_test.rs`
  - `test_map_rtc_provider_callback_over_http`
  - `test_get_rtc_recording_artifact_over_http`
- `services/sdkwork-im-cloud-gateway/tests/http_e2e_test.rs`
  - `test_local_minimal_profile_maps_rtc_provider_callback_over_http`
  - `test_local_minimal_profile_gets_rtc_recording_artifact_over_http`

## 验证记录

已执行并通过：

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p im-call-runtime --offline --test http_smoke_test -- --nocapture`
- `cargo test -p im-call-runtime --offline --test rtc_runtime_persistence_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test http_e2e_test test_local_minimal_profile_issues_rtc_participant_credential_over_http -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test http_e2e_test test_local_minimal_profile_gets_rtc_provider_health_over_http -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test http_e2e_test test_local_minimal_profile_maps_rtc_provider_callback_over_http -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test http_e2e_test test_local_minimal_profile_gets_rtc_recording_artifact_over_http -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test rtc_runtime_persistence_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 当前结论

- Step 06 / `06-B` 的 RTC provider HTTP surface 已从：
  - `session + credential + health`
  收口到：
  - `session + credential + callback + health + artifact`
- 当前剩余 RTC provider 侧架构债务：
  - `rtc-aliyun / rtc-tencent` adapter 尚未实现
  - recording artifact 尚未统一回流到 `ObjectStorageProvider`
  - provider callback 的内部认证/profile 仍需要标准化

