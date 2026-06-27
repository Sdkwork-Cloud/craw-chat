# S05 Loop44 Closure And S07 Next Mainline Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Close the remaining Loop 44 documentation batch for `S05`, verify the closure evidence is fresh, and then switch the active mainline to `S07` for the next implementation loop.

**Architecture:** `S05` already has code and test evidence; this batch is about making every execution, review, architecture, and release artifact agree on the same as-built truth. Once the closure set is consistent, the next loop should pivot to `S07` and select one concrete runtime gap instead of continuing to harden `S05`.

**Tech Stack:** Markdown docs, Rust workspace verification via `cargo test`, repo status and search via `git` and `rg`

---

### Task 1: Freeze Loop 44 Closure Scope

**Files:**
- Modify: `docs/review/S05-执行卡-2026-04-10.md`
- Modify: `docs/review/S00-S14-全局闭环复核-2026-04-10.md`
- Modify: `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
- Modify: `docs/release/CHANGELOG.md`

- [ ] **Step 1: Re-read the repeated-step prompt and current execution artifacts**

Run: `rg -n "Loop 44|S05|S07|step_closure|local_closure" docs/prompts docs/review docs/架构 docs/release`
Expected: current closure language and lagging references are visible.

- [ ] **Step 2: Update the closure source-of-truth docs**

Write the minimum edits so every shared document says:
- `S05 = step_closure`
- stronger `staged / manifest` transaction proof is deferred durability hardening
- `release_closure = no` because `S07` remains `local_closure`
- active mainline switches from `S05` to `S07`

- [ ] **Step 3: Verify the old blocker wording is gone**

Run: `rg -n "S05 = not_closed / local_closure|mainline：继续 `S05`|mainline: continue `S05`" docs/review docs/架构 docs/release`
Expected: no stale Loop 44 blocker wording remains in the updated global docs.

### Task 2: Create Missing Loop 44 Artifacts

**Files:**
- Create: `docs/step/115-S05-step-closure-loop44-current-checkpoint-2026-04-11.md`
- Create: `docs/review/S05-Loop44补充-2026-04-11.md`
- Create: `docs/架构/152CJ-Loop44补充-2026-04-11.md`
- Create: `docs/release/2026-04-11-v0.0.44-loop-44.md`

- [ ] **Step 1: Create the Loop 44 step checkpoint**

Record: closure batch type, single main gap decision, evidence reused, closure decision, remaining deferred backlog, and next-loop input.

- [ ] **Step 2: Create review and architecture addenda**

Record: why `repair-marker + operator surface` is sufficient for `S05`, what is still deferred, and why `S07` becomes the next mainline.

- [ ] **Step 3: Create the Loop 44 release note and prepend changelog entry**

Record: loop number, affected step, evidence, risks, and the `S07` follow-up entrypoint.

### Task 3: Fresh Verification

**Files:**
- Verify: `services/control-plane-api/**`
- Verify: `services/local-minimal-node/**`
- Verify: updated `docs/**`

- [ ] **Step 1: Run the fresh package evidence for the closure claim**

Run: `cargo test -p control-plane-api --offline --tests -- --nocapture`
Expected: all `control-plane-api` tests pass.

- [ ] **Step 2: Run the fresh operator-wrapper evidence**

Run: `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
Expected: `deployment_profile_test` passes.

- [ ] **Step 3: Run consistency searches**

Run: `rg -n "S05 = step_closure|release_closure = no|主线：继续 `S07`|mainline：继续 `S07`|deferred durability hardening" docs/review docs/架构 docs/release docs/step`
Expected: updated files agree on the Loop 44 closure decision and next mainline.

### Task 4: Start The Next Loop On S07

**Files:**
- Inspect: `docs/review/S07-执行卡-2026-04-10.md`
- Inspect: `docs/review/S07-质量审计与复盘-2026-04-10.md`
- Inspect: `docs/review/S07-架构兑现与回写决议-2026-04-10.md`
- Inspect: `services/conversation-runtime/**`

- [ ] **Step 1: Re-read the repeated-step prompt before the next loop**

Run: `Get-Content -Raw docs/prompts/反复执行Step指令.md`
Expected: loop protocol is reloaded before choosing the next gap.

- [ ] **Step 2: Pick one concrete S07 gap and drive a TDD loop**

Target: one of `thread`, `shared-external history`, `invited/shared history`, or `retention enforcement`, based on current code and dependency reality.

- [ ] **Step 3: Verify and backwrite the next loop**

Run the smallest fresh tests that prove the new `S07` behavior, then update `docs/step`, `docs/review`, `docs/架构`, and `docs/release`.
