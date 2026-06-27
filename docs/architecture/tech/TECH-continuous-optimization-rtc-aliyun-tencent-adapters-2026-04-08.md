> Migrated from `docs/review/continuous-optimization-rtc-aliyun-tencent-adapters-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# RTC Aliyun / Tencent Adapter Continuous Optimization Review

日期：2026-04-08

## 本轮目标

把 provider registry 中已声明但尚未落地的两个 RTC provider 补成真实 adapter：

- `rtc-aliyun`
- `rtc-tencent`

## 变更摘要

- 新增 crate：
  - `adapters/rtc-aliyun`
  - `adapters/rtc-tencent`
- 两个 adapter 都实现了完整的 `RtcProviderPort` contract：
  - `create_session`
  - `close_session`
  - `issue_participant_credential`
  - `refresh_participant_credential`
  - `map_provider_callback`
  - `export_recording_artifact`
  - `provider_health_snapshot`
- `services/im-call-runtime/src/lib.rs`
  - `RtcRuntime::with_store(...)` 默认内建 provider map 扩容为：
    - `rtc-volcengine`
    - `rtc-aliyun`
    - `rtc-tencent`

## 设计决策

- Aliyun / Tencent adapter 继续复用与 Volcengine 相同的 provider-agnostic contract，不引入任何厂商 SDK 类型。
- 默认 global selection 仍然保持 `rtc-volcengine`；Aliyun / Tencent 通过 provider registry override 进入选择面。
- runtime 层新增 tenant override 选择测试，证明 registry 选择结果能够路由到真实 provider，而不是只停留在架构矩阵。

## 新增测试

- `adapters/rtc-aliyun/tests/adapter_contract_test.rs`
  - `test_aliyun_rtc_provider_implements_contract_surface`
- `adapters/rtc-tencent/tests/adapter_contract_test.rs`
  - `test_tencent_rtc_provider_implements_contract_surface`
- `services/im-call-runtime/tests/rtc_runtime_persistence_test.rs`
  - `test_runtime_can_route_to_tenant_selected_builtin_rtc_providers`

## 验证记录

已执行并通过：

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p im-adapter-rtc-aliyun --offline --test adapter_contract_test -- --nocapture`
- `cargo test -p im-adapter-rtc-tencent --offline --test adapter_contract_test -- --nocapture`
- `cargo test -p im-call-runtime --offline --test http_smoke_test -- --nocapture`
- `cargo test -p im-call-runtime --offline --test rtc_runtime_persistence_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test http_e2e_test test_local_minimal_profile_gets_rtc_provider_health_over_http -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test rtc_runtime_persistence_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 当前结论

- `rtc-volcengine / rtc-aliyun / rtc-tencent` 已全部具备最小可运行 adapter。
- RTC provider plugin 体系不再只有 registry 声明，已经形成可实例化、可选择、可测试的三 provider baseline。
- 当前剩余 RTC provider 架构债务：
  - `sdkwork-im-server` 仍未开放 provider registry 注入/配置面
  - recording artifact 仍未统一回流到 `ObjectStorageProvider`
  - provider callback 的内部认证/profile 仍需标准化

