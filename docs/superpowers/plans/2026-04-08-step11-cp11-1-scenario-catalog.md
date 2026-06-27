# Step 11 CP11-1 Scenario Catalog Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Freeze the Step 11 performance, HA, and DR scenario inventory as a machine-readable contract plus an operator-facing execution doc, then backwrite review and architecture evidence.

**Architecture:** Reuse the repo's existing local-minimal-node integration-test pattern to enforce a Step 11 scenario catalog contract under `tools/perf/`. Pair the catalog with a human-readable drill document in `docs/部署/`, and treat both as the baseline for later CP11-2 and CP11-3 execution evidence instead of inventing new load infrastructure prematurely.

**Tech Stack:** Rust integration tests, serde_json, Markdown review docs, architecture as-built backwrites.

---

### Task 1: Lock the CP11-1 contract with failing tests

**Files:**
- Create: `services/local-minimal-node/tests/performance_drill_catalog_test.rs`
- Test: `cargo test -p local-minimal-node --offline --test performance_drill_catalog_test`

- [ ] **Step 1: Write the failing contract tests**

Cover:
- `tools/perf/step-11-scenario-catalog.json` exists
- the catalog freezes `CI Smoke Tier`, `Pre-Release Tier`, and `Capacity Tier`
- the catalog enumerates `connection`, `message`, `stream`, `drain-rebalance`, `restore-recovery`, and `upgrade-rollback`
- the catalog references the current repo assets that seed Step 11
- `docs/部署/性能与灾备演练场景.md` points back to the catalog and mirrors the same tier/scenario vocabulary

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test -p local-minimal-node --offline --test performance_drill_catalog_test`
Expected: FAIL because the catalog/doc files do not exist yet.

### Task 2: Implement the frozen Step 11 baseline

**Files:**
- Create: `tools/perf/step-11-scenario-catalog.json`
- Create: `docs/部署/性能与灾备演练场景.md`

- [ ] **Step 1: Add the machine-readable scenario catalog**

Include:
- tier definitions
- scenario families
- metrics to capture later
- repo asset mapping
- linkage to `CP11-2`, `CP11-3`, and `CP11-4`

- [ ] **Step 2: Add the operator-facing drill document**

Document:
- how operators use the catalog
- tier meanings and sequencing
- which existing tests/scripts seed each scenario family
- what is still deferred to later Step 11 checkpoints

- [ ] **Step 3: Re-run the contract test**

Run: `cargo test -p local-minimal-node --offline --test performance_drill_catalog_test`
Expected: PASS.

### Task 3: Record review and architecture evidence

**Files:**
- Create: `docs/review/step-11-cp11-1-性能与演练场景清单-执行卡-2026-04-08.md`
- Create: `docs/review/step-11-cp11-1-性能与演练场景清单-架构兑现与回写决议-2026-04-08.md`
- Create: `docs/review/step-11-cp11-1-性能与演练场景清单-质量审计与复盘-2026-04-08.md`
- Create: `docs/review/step-11-执行卡-2026-04-08.md`
- Modify: `docs/架构/09-实施计划.md`
- Modify: `docs/架构/131-连接管理与分层弹性扩容架构设计-2026-04-06.md`
- Modify: `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md`
- Modify: `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
- Modify: `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
- Modify: `docs/架构/149-多Cell多Region协议升级与灾备兼容设计-2026-04-06.md`

- [ ] **Step 1: Write CP11-1 review evidence**

Record:
- why CP11-1 is the right next increment
- what catalog/doc was added
- what remains open in Step 11
- the exact verification command and expected outcome

- [ ] **Step 2: Append architecture as-built entries**

Backwrite:
- scenario inventory freeze into `09`
- connection/capacity baseline into `131` and `137`
- HA/DR drill baseline into `138`
- metric capture contract into `140`
- upgrade/rollback drill baseline into `149`

### Task 4: Fresh verification

**Files:**
- Modify: none
- Test: workspace commands

- [ ] **Step 1: Format the focused files**

Run: `cargo fmt --all --check`
Expected: PASS.

- [ ] **Step 2: Run the focused contract suite**

Run: `cargo test -p local-minimal-node --offline --test performance_drill_catalog_test`
Expected: PASS.

- [ ] **Step 3: Reassess Step 11 status**

Expected:
- `CP11-1` closed
- `Step 11` still open pending quantitative drills and failover evidence
- next move is `CP11-2`
