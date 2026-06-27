# User Module Runtime Provider Selection Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 让 `sdkwork-im-server` 默认启动入口支持 `principal-profile-upstream-context / principal-profile-external-catalog` 的真实配置选择。

**Architecture:** 默认入口继续经由统一 `PrincipalProfileProvider` port，不新增旁路。external 形态以外部目录 JSON 作为最小真实目录源，保证主链路元数据、bootstrap member、message actor 行为与显式注入测试一致。

**Tech Stack:** Rust, Axum, serde, standalone.split-services.development runtime

---

### Task 1: 冻结默认入口 external 选择回归测试

**Files:**
- Create: `services/sdkwork-im-cloud-gateway/tests/principal_profile_provider_runtime_selection_test.rs`
- Test: `services/sdkwork-im-cloud-gateway/tests/principal_profile_provider_runtime_selection_test.rs`

- [x] **Step 1: 写失败测试**

- [x] **Step 2: 运行红灯**

Run: `cargo test -p sdkwork-im-cloud-gateway --test principal_profile_provider_runtime_selection_test -- --nocapture`
Expected: FAIL，且默认入口仍输出 local user metadata

- [x] **Step 3: 保持断言聚焦**

断言 `displayName / externalSystem / externalPrincipalId / principalProfilePluginId`

- [x] **Step 4: 运行通过验证**

Run: `cargo test -p sdkwork-im-cloud-gateway --test principal_profile_provider_runtime_selection_test -- --nocapture`
Expected: PASS

### Task 2: 落地默认 provider 选择与 external adapter

**Files:**
- Modify: `services/sdkwork-im-cloud-gateway/src/node/principal_profile.rs`

- [x] **Step 1: 增加 provider 选择配置解析**

- [x] **Step 2: 实现 `ExternalCatalogPrincipalProfileProvider`**

- [x] **Step 3: 接入默认 `build_default_principal_profile_provider()`**

- [x] **Step 4: 保持 `PrincipalProfileProvider` 统一边界**

不允许 conversation/member/message 主链路绕过 provider

### Task 3: 回归验证与文档回写

**Files:**
- Create: `docs/review/continuous-optimization-principal-profile-runtime-provider-selection-2026-04-09.md`
- Create: `docs/step/continuous-optimization-principal-profile-runtime-provider-selection-2026-04-09.md`
- Create: `docs/架构/150BH-principal-profile-runtime-provider-selection-design-2026-04-09.md`
- Modify: `docs/review/README.md`
- Modify: `docs/step/README.md`
- Modify: `docs/架构/README.md`

- [x] **Step 1: 跑 principal-profile 相关回归**

Run: `cargo test -p sdkwork-im-cloud-gateway principal_profile_provider -- --nocapture`
Expected: PASS

- [x] **Step 2: 回写文档**

- [ ] **Step 3: 进入下一轮**

优先补 `retired-lifecycle-install.cmd / retired-lifecycle-deploy.cmd` help discoverability
