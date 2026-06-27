> Migrated from `docs/review/continuous-optimization-release-bundle-evidence-index-schema-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - release bundle evidence index schema - 2026-04-08

## 1. 本轮背景

- 上一轮已经把 `standalone.split-services.development` 的 post-release evidence index 固定成 machine-readable JSON。
- 但如果没有独立 schema contract，这份 JSON 仍容易在后续 bundle 中继续漂移：
  - 字段名可能变化
  - 状态值可能失控增长
  - `$schema`、bundle manifest、release README 之间没有统一锚点
- 当前环境依然没有真实 `standalone.split-services.development` 发布后执行窗口，因此本轮继续避免伪造真实 evidence，而是先把 evidence index 的 schema contract 冻结下来。

## 2. 实际落地

### 2.1 新增 evidence index schema

- 新增：`artifacts/releases/schemas/post-release-evidence-index.schema.json`
- 当前 schema 已固定：
  - 顶层必填字段：
    - `bundleId`
    - `wave`
    - `profile`
    - `artifact`
    - `state`
    - `boundary`
    - `sampleDoc`
    - `recordTemplate`
    - `evidenceSlots`
  - `artifact = post-release-evidence-index`
  - `evidenceSlots[*]` 的最小必填字段：
    - `id`
    - `kind`
    - `required`
    - `status`
    - `command`

### 2.2 现有 evidence index 已改为显式声明 `$schema`

- 更新：`artifacts/releases/wave-d-2026-04-08/standalone.split-services.development-post-release-evidence-index.json`
- 当前 JSON 已显式声明：
  - `$schema = ../schemas/post-release-evidence-index.schema.json`
- 这意味着后续同类 bundle 不再只是“长得像这份 JSON”，而是要对齐到同一份 schema contract。

### 2.3 bundle 文档入口已同步

- 更新：`artifacts/releases/README.md`
- 更新：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 当前 release bundle 约定已经明确：
  - evidence index schema 的固定路径
  - evidence index 应通过 `$schema` 指向该 contract
  - `Wave D` bundle 已把 schema 与实际 JSON 一起归档

### 2.4 contract gate 已冻结

- 更新：`services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- 新增：
  - `test_local_default_release_bundle_freezes_evidence_index_schema_contract`
- 当前回归门禁已经锁定：
  - schema 文件存在且可解析
  - evidence index 必须显式声明 `$schema`
  - schema 顶层与 slot 必填字段不能漂移
  - release bundle manifest 与 `artifacts/releases/README.md` 都必须公开 schema 路径

## 3. 当前判断

- release bundle 的 evidence index 现在不再只是单个 JSON 样本，而是拥有独立 schema contract。
- 本轮仍然没有伪造真实 operator 执行证据，只是把“未来真实证据应该遵守什么结构”先冻结下来。
- 这能降低后续 bundle 继续各写各的 JSON 结构、难以归档和自动核对的风险。
- 下一轮仍可继续推进：
  - 在真实 `standalone.split-services.development` 发布后验证窗口回填 schema-backed evidence index
  - 为 evidence slot 追加时间戳、文件路径或 checksum 约定
  - 把同类 schema 扩展到更多 release bundle 资产

## 4. fresh evidence

- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_local_default_release_bundle_freezes_evidence_index_schema_contract -- --nocapture`
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture`

