# IM 三能力对齐 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or executing-plans for task-by-task execution.

**Goal:** Ship parallel MVP slices for weak-network ARQ nack replay, 10K large-group member directory indexing, and Tauri desktop offline SQLite cache.

**Architecture:** Extend existing CCP business frames and projection member store indexes; add client-side rusqlite without changing server PostgreSQL authority.

**Tech Stack:** Rust (session-gateway, projection-service, runtime-link, Tauri), rusqlite, TypeScript pc-core bridge.

---

## Track A — Weak network ARQ MVP

- [x] Add `rewind_for_nack` + `plan_nack_replay` to `sdkwork-im-runtime-link`
- [x] Add unit tests for nack rewind
- [x] Extend `ClientFrameEnvelope` with `nack_through_seq`
- [x] Handle `events.nack` in `session-gateway/src/websocket.rs`
- [x] IM SDK TypeScript client seq-gap tracker + `events.nack` send

## Track B — 10K large group MVP

- [x] Raise `DEFAULT_CHAT_GROUP_MAX_MEMBERS` to 10_000
- [x] Add `member_directory_index` to `projection-service/src/member_store.rs`
- [x] Rewrite `member_directory_window` to use index iteration
- [x] Add/update tests in `member_directory.rs`

## Track C — Desktop offline MVP

- [x] Add `rusqlite` to `sdkwork-im-pc-desktop` Cargo.toml
- [x] Implement `offline_store.rs` + register Tauri commands
- [x] Add `desktopOfflineStore.ts` in `sdkwork-im-pc-core`
- [x] Wire `ChatService` online persist + offline read fallback

## Verification

- [x] `cargo test -p projection-service member_directory`
- [x] `cargo test -p sdkwork-im-runtime-link nack`
- [x] `cargo test -p sdkwork-im-pc --lib` (offline_store)
- [x] `node scripts/dev/sdkwork-im-three-capabilities-standard.test.mjs`
- [x] Update `OPTIMIZATION_ROADMAP.md` MVP status lines

## Production alignment (2026-07-06)

- [x] Wire eight projection-owned RPC ops in `rpc_projection_dispatch.rs`
- [x] Message favorites index pagination (`message_favorites_index`)
- [x] Filtered favorites list uses index scan (`favorite_matches_filters`)
- [x] Desktop offline pending text/media send queue + hydrate on reconnect
- [x] PC MessageList `sendState` pending/failed UI + `retryFailedMessage`
- [x] Portal real aggregation via `portal-service` + `im-portal-snapshots` (ops/audit)
- [x] Console Security fail-closed (`healthScore: null`); Dashboard empty trends
- [x] Admin Infra metric labels aligned (`realtimeWindowHealth`)
- [x] Flutter inbox cursor pagination (HTTP wire `page_size=20`, bounded sync)
