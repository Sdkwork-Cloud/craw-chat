# Step 11 CP11-2 Local Quant Baseline Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Produce one repeatable local-minimal quantitative run for connection, message, and stream scenarios, then write the measured results into Step 11 review evidence.

**Architecture:** Add a machine-readable local baseline config under `tools/perf/`, and a focused `local-minimal-node` integration suite that reads the config, executes three in-process scenarios, and emits metrics without introducing brittle throughput thresholds. Keep the implementation scoped to the current local-minimal runtime and reuse the Step 11 catalog/doc as the authoritative vocabulary.

**Tech Stack:** Rust integration tests, Axum in-process app execution, tokio-tungstenite, serde_json, Markdown review docs.

---

### Task 1: Lock the CP11-2 local baseline contract with failing tests

**Files:**
- Create: `services/local-minimal-node/tests/performance_quant_baseline_test.rs`
- Test: `cargo test -p local-minimal-node --offline --test performance_quant_baseline_test`

- [ ] **Step 1: Write the failing contract and quant tests**

Cover:
- `tools/perf/step-11-cp11-2-local-baseline.json` exists
- `docs/部署/性能与灾备演练场景.md` points to that local baseline
- connection scenario opens a configured number of websocket sessions and emits metrics
- message scenario posts a configured number of messages and emits metrics
- stream scenario appends a configured number of frames and emits metrics

- [ ] **Step 2: Run the suite to verify it fails**

Run: `cargo test -p local-minimal-node --offline --test performance_quant_baseline_test`
Expected: FAIL because the local baseline config and doc wiring do not exist yet.

### Task 2: Implement the repeatable local baseline

**Files:**
- Create: `tools/perf/step-11-cp11-2-local-baseline.json`
- Modify: `tools/perf/step-11-scenario-catalog.json`
- Modify: `docs/部署/性能与灾备演练场景.md`

- [ ] **Step 1: Add the machine-readable local baseline**

Freeze:
- profile and tier
- connection count
- message count
- stream frame count

- [ ] **Step 2: Wire the operator doc and catalog to the local baseline**

Document:
- where the baseline lives
- which test executes it
- how it relates to later `Pre-Release Tier` and `Capacity Tier`

- [ ] **Step 3: Re-run the quant suite**

Run: `cargo test -p local-minimal-node --offline --test performance_quant_baseline_test -- --nocapture`
Expected: PASS and print quantitative metrics for all three scenarios.

### Task 3: Write Step 11 review evidence

**Files:**
- Create: `docs/review/step-11-cp11-2-连接消息流量化基线-执行卡-2026-04-08.md`
- Create: `docs/review/step-11-cp11-2-连接消息流量化基线-架构兑现与回写决议-2026-04-08.md`
- Create: `docs/review/step-11-cp11-2-连接消息流量化基线-质量审计与复盘-2026-04-08.md`
- Create: `docs/review/step-11-容量基准结果-2026-04-08.md`
- Modify: `docs/review/step-11-执行卡-2026-04-08.md`

- [ ] **Step 1: Capture the measured numbers**

Record:
- environment
- configured counts
- measured duration
- throughput
- p95 latency

- [ ] **Step 2: Update the step-level execution card**

Reflect:
- `CP11-2` complete
- `Step 11` still blocked on `CP11-3` and `CP11-4`

### Task 4: Backwrite focused architecture evidence and verify

**Files:**
- Modify: `docs/架构/09-实施计划.md`
- Modify: `docs/架构/131-连接管理与分层弹性扩容架构设计-2026-04-06.md`
- Modify: `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md`
- Modify: `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`

- [ ] **Step 1: Append focused as-built entries**

Backwrite:
- Step 11 local quantitative baseline into `09`
- connection/message/stream quantitative evidence into `131`
- local-minimal tier evidence into `137`
- Step 11 metric vocabulary plus measured output into `140`

- [ ] **Step 2: Re-run focused verification**

Run:
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline --test performance_quant_baseline_test -- --nocapture`

Expected:
- PASS
- `CP11-2` closed
- next move is `CP11-3`
