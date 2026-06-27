# CHANGELOG

## v0.0.86 - 2026-04-12

- Loop `86`
- step `S07`
- `control-plane-api` fixed proactive stale-claim normalization on the shared-channel sync next-write path
  - ordinary next writes now reclaim stale pending claim metadata before system dispatch / backlog queue evaluation
  - the reclaim runs even when the shared-channel trigger is unconfigured
  - operator reclaim / repair / republish stale normalization behavior remains intact
- TDD:
  - red: `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_next_write_reclaims_stale_claim_metadata_when_trigger_is_unconfigured -- --nocapture`
  - red symptom: a second ordinary write left the original stale claimed pending item serialized with `ownerActorId / ownerActorKind / claimedAt / leaseExpiresAt`
  - green: next-write dispatch now reclaims stale pending claim metadata before queue evaluation, so the older item returns to `unclaimed`
- fresh verification:
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_next_write_reclaims_stale_claim_metadata_when_trigger_is_unconfigured -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test -- --nocapture` = `31 passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `87 passed`
- artifacts:
  - `docs/step/157-S07-shared-channel-sync-next-write-proactive-stale-claim-reclaim-loop86-current-checkpoint-2026-04-12.md`
  - `docs/review/S07-Loop86补充-2026-04-12.md`
  - `docs/架构/152CJ-Loop86补充-2026-04-12.md`
  - `docs/release/2026-04-12-v0.0.86-loop-86.md`
- closure:
  - `S07` remains `not_closed / local_closure`
  - remaining main gap: `automatic stale detection / timeout reclaim / scheduler`

## v0.0.85 - 2026-04-12

- Loop `85`
- step `S07`
- `control-plane-api` fixed stale-lease renewal on the shared-channel sync targeted pending-republish `trigger_unconfigured` path
  - same-owner stale republish now refreshes `claimedAt / leaseExpiresAt` before returning `trigger_unconfigured`
  - the refreshed pending state is persisted even when no trigger is configured
  - dispatch-enabled republish renewal, owner-bound claim/republish semantics, and repair stale normalization remain intact
- TDD:
  - red: `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_republish_renews_stale_lease_for_same_operator_when_trigger_unconfigured -- --nocapture`
  - red symptom: same-owner stale republish returned `trigger_unconfigured`, but left `claimedAt / leaseExpiresAt` unchanged and inventory `leaseStatus = stale`
  - green: same-owner stale republish now renews lease metadata before the unconfigured early return and persists the refreshed state
- fresh verification:
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_republish_renews_stale_lease_for_same_operator_when_trigger_unconfigured -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test -- --nocapture` = `30 passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `passed`
- artifacts:
  - `docs/step/156-S07-shared-channel-sync-unconfigured-same-owner-stale-republish-lease-renew-loop85-current-checkpoint-2026-04-12.md`
  - `docs/review/S07-Loop85补充-2026-04-12.md`
  - `docs/架构/152CJ-Loop85补充-2026-04-12.md`
  - `docs/release/2026-04-12-v0.0.85-loop-85.md`
- closure:
  - `S07` remains `not_closed / local_closure`
  - remaining main gap: `automatic stale detection / timeout reclaim / scheduler`

## v0.0.84 - 2026-04-12

- Loop `84`
- step `S07`
- `control-plane-api` fixed stale-lease renewal on the shared-channel sync targeted pending-republish failure path
  - same-owner stale republish now refreshes `claimedAt / leaseExpiresAt` before dispatch
  - failed dispatch persistence no longer clears owner metadata from a same-owner stale request
  - pending claim ownership, same-owner stale claim renewal, and dead-letter transition behavior remain intact
- TDD:
  - red: `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_republish_renews_stale_lease_for_same_operator_on_failure -- --nocapture`
  - red symptom: failed same-owner stale republish cleared `claimedAt / leaseExpiresAt` and returned the request to the unclaimed pool
  - green: same-owner stale republish now renews lease metadata before failed dispatch persistence runs
- fresh verification:
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_republish_renews_stale_lease_for_same_operator_on_failure -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_renews_stale_lease_for_same_operator -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_targeted_republish_dead_letter_reclaims_claim_metadata -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `85 passed`
- artifacts:
  - `docs/step/155-S07-shared-channel-sync-same-owner-stale-republish-lease-renew-loop84-current-checkpoint-2026-04-12.md`
  - `docs/review/S07-Loop84补充-2026-04-12.md`
  - `docs/架构/152CJ-Loop84补充-2026-04-12.md`
  - `docs/release/2026-04-12-v0.0.84-loop-84.md`
- closure:
  - `S07` remains `not_closed / local_closure`
  - remaining main gap: `trigger-unconfigured targeted republish stale normalization`

## v0.0.83 - 2026-04-12

- Loop `83`
- step `S07`
- `control-plane-api` fixed stale-lease renewal on the shared-channel sync targeted pending-claim surface
  - same-owner claim on a stale pending request now refreshes `claimedAt / leaseExpiresAt`
  - successful same-owner re-claim now returns inventory to `leaseStatus = active`
  - pending claim ownership, foreign takeover, and unconfigured repair reclaim behavior remain intact
- TDD:
  - red: `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_renews_stale_lease_for_same_operator -- --nocapture`
  - red symptom: claim returned success, but `claimedAt` stayed unchanged and inventory remained `leaseStatus = stale`
  - green: same-owner stale claim now renews lease metadata before returning success
- fresh verification:
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_renews_stale_lease_for_same_operator -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_repair_reclaims_stale_claim_when_trigger_is_unconfigured -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `84 passed`
- artifacts:
  - `docs/step/154-S07-shared-channel-sync-same-owner-stale-claim-lease-renew-loop83-current-checkpoint-2026-04-12.md`
  - `docs/review/S07-Loop83补充-2026-04-12.md`
  - `docs/架构/152CJ-Loop83补充-2026-04-12.md`
  - `docs/release/2026-04-12-v0.0.83-loop-83.md`
- closure:
  - `S07` remains `not_closed / local_closure`
  - remaining main gap: `automatic stale detection / timeout reclaim / scheduler`

## v0.0.82 - 2026-04-12

- Loop `82`
- step `S07`
- `control-plane-api` fixed stale-claim reclaim on the shared-channel sync repair surface when no trigger is installed
  - `repair-shared-channel-sync` now reclaims stale pending ownership before returning `trigger_unconfigured`
  - reclaimed pending items remain queued but return to the `unclaimed` pool
  - configured repair dispatch and explicit reclaim-surface behavior remain intact
- TDD:
  - red: `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_repair_reclaims_stale_claim_when_trigger_is_unconfigured -- --nocapture`
  - red symptom: repair returned `reclaimed = 0` and left stale owner metadata in pending backlog
  - green: unconfigured repair now persists stale reclaim before the early return
- fresh verification:
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_repair_reclaims_stale_claim_when_trigger_is_unconfigured -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_repair_reclaims_stale_claim_before_dispatch -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_stale_claim_reclaim_surface_clears_owner_metadata -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `83 passed`
- artifacts:
  - `docs/step/153-S07-shared-channel-sync-unconfigured-repair-stale-reclaim-loop82-current-checkpoint-2026-04-12.md`
  - `docs/review/S07-Loop82补充-2026-04-12.md`
  - `docs/架构/152CJ-Loop82补充-2026-04-12.md`
  - `docs/release/2026-04-12-v0.0.82-loop-82.md`
- closure:
  - `S07` remains `not_closed / local_closure`
  - remaining main gap: `automatic stale detection / timeout reclaim / scheduler`

## v0.0.81 - 2026-04-11

- Loop `81`
- step `S07`
- `control-plane-api` fixed owner metadata leakage during shared-channel sync pending -> dead-letter transition
  - claimed requests no longer keep `ownerActorId / ownerActorKind / claimedAt / leaseExpiresAt` after targeted republish failure crosses the dead-letter threshold
  - dead-letter inventory now reports those requests as `unclaimed`
  - existing dead-letter requeue and pending ownership guard behavior remain intact
- TDD:
  - red: `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_targeted_republish_dead_letter_reclaims_claim_metadata -- --nocapture`
  - red symptom: dead-lettered item still serialized `ownerActorId`
  - green: dead-letter transition clears owner metadata before failure-bucket persistence
- fresh verification:
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_targeted_republish_dead_letter_reclaims_claim_metadata -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test dead_letter -- --nocapture` = `6 passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `82 passed`
- artifacts:
  - `docs/step/152-S07-shared-channel-sync-dead-letter-owner-metadata-reclaim-loop81-current-checkpoint-2026-04-11.md`
  - `docs/review/S07-Loop81补充-2026-04-11.md`
  - `docs/架构/152CJ-Loop81补充-2026-04-11.md`
  - `docs/release/2026-04-11-v0.0.81-loop-81.md`
- closure:
  - `S07` remains `not_closed / local_closure`
  - remaining main gap: `automatic stale detection / timeout reclaim / scheduler`

## v0.0.80 - 2026-04-11

- Loop `80`
- step `S07`
- `control-plane-api` fixed stale owner metadata restoration during shared-channel sync dead-letter requeue
  - dead-letter items now return to pending backlog as `unclaimed`
  - failure-budget rearm remains intact
  - targeted requeue isolation remains intact
- TDD:
  - red: `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_dead_letter_requeue_reclaims_stale_claim_metadata -- --nocapture`
  - red symptom: requeued pending item still serialized `ownerActorId`
  - green: dead-letter requeue clears owner metadata before restoring pending backlog
- fresh verification:
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_dead_letter_requeue_reclaims_stale_claim_metadata -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_dead_letter_requeue_rearms_failure_budget_before_next_repair_attempt -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_dead_letter_inventory_and_targeted_requeue_only_restores_selected_actor -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `81 passed`
- artifacts:
  - `docs/step/151-S07-shared-channel-sync-dead-letter-requeue-stale-metadata-reclaim-loop80-current-checkpoint-2026-04-11.md`
  - `docs/review/S07-Loop80补充-2026-04-11.md`
  - `docs/架构/152CJ-Loop80补充-2026-04-11.md`
  - `docs/release/2026-04-11-v0.0.80-loop-80.md`
- closure:
  - `S07` remains `not_closed / local_closure`
  - remaining main gap: `automatic stale detection / timeout reclaim / scheduler`

## v0.0.79 - 2026-04-11

- Loop `79`
- step `S07`
- `control-plane-api` added an explicit stale pending-claim reclaim surface for shared-channel sync
  - `POST /backend/v3/api/control/social/runtime/reclaim-stale-pending-shared-channel-sync` now clears stale ownership without dispatching backlog
  - reclaimed items remain pending and return to the unclaimed pool
  - repair reclaim, stale takeover, and next-write stale retry behavior remain intact
- TDD:
  - red: `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_stale_claim_reclaim_surface_clears_owner_metadata -- --nocapture`
  - red symptom: the new operator route returned `404`
  - green: stale reclaim returns `reclaimed = 1` and leaves the pending item `unclaimed`
- fresh verification:
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_stale_claim_reclaim_surface_clears_owner_metadata -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_repair_reclaims_stale_claim_before_dispatch -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_next_ready_pair_retry_failure_reclaims_stale_claim_metadata -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `80 passed`
- artifacts:
  - `docs/step/150-S07-shared-channel-sync-stale-claim-reclaim-surface-loop79-current-checkpoint-2026-04-11.md`
  - `docs/review/S07-Loop79补充-2026-04-11.md`
  - `docs/架构/152CJ-Loop79补充-2026-04-11.md`
  - `docs/release/2026-04-11-v0.0.79-loop-79.md`
- closure:
  - `S07` remains `not_closed / local_closure`
  - remaining main gap: `automatic stale detection / timeout reclaim / scheduler`

## v0.0.78 - 2026-04-11

- Loop `78`
- step `S07`
- `control-plane-api` fixed stale owner metadata persistence in shared-channel sync next-write retry failure handling
  - stale claimed pending backlog no longer keeps obsolete owner metadata after a failed retry
  - the same stale reclaim rule now applies to runtime failure persistence, repair retry failure, and targeted republish retry failure
  - Loop77 ownership guard and unclaimed auto-retry behavior remain intact
- TDD:
  - red: `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_next_ready_pair_retry_failure_reclaims_stale_claim_metadata -- --nocapture`
  - red symptom: the original stale pending item still serialized `ownerActorId`
  - green: stale owner metadata is reclaimed before the failed retry is written back
- fresh verification:
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_next_ready_pair_retry_failure_reclaims_stale_claim_metadata -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_next_healthy_ready_pair_write_respects_active_pending_claim_ownership -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_backlog_retries_on_next_healthy_ready_pair_write -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `79 passed`
- artifacts:
  - `docs/step/149-S07-shared-channel-sync-next-write-stale-metadata-reclaim-loop78-current-checkpoint-2026-04-11.md`
  - `docs/review/S07-Loop78补充-2026-04-11.md`
  - `docs/架构/152CJ-Loop78补充-2026-04-11.md`
  - `docs/release/2026-04-11-v0.0.78-loop-78.md`
- closure:
  - `S07` remains `not_closed / local_closure`
  - remaining main gap: `automatic stale detection / timeout reclaim / scheduler`

## v0.0.77 - 2026-04-11

- Loop `77`
- step `S07`
- `control-plane-api` fixed a next-write auto-retry ownership bug in shared-channel sync
  - actively claimed pending backlog is no longer flushed by unrelated healthy ready-pair writes
  - legacy `Untracked` claims are also blocked from system auto-retry
  - same-key incoming requests are blocked while those claims exist
  - unclaimed backlog still retries on the next healthy ready-pair write
- TDD:
  - red: `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_next_healthy_ready_pair_write_respects_active_pending_claim_ownership -- --nocapture`
  - red symptom: healthy ready-pair write flushed claimed backlog, `left = 0`, `right = 1`
  - green: claimed backlog is preserved while unrelated healthy work still dispatches
- fresh verification:
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_next_healthy_ready_pair_write_respects_active_pending_claim_ownership -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_backlog_retries_on_next_healthy_ready_pair_write -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `78 passed`
- artifacts:
  - `docs/step/148-S07-shared-channel-sync-next-write-claim-ownership-loop77-current-checkpoint-2026-04-11.md`
  - `docs/review/S07-Loop77补充-2026-04-11.md`
  - `docs/架构/152CJ-Loop77补充-2026-04-11.md`
  - `docs/release/2026-04-11-v0.0.77-loop-77.md`
- closure:
  - `S07` remains `not_closed / local_closure`
  - remaining main gap: `automatic stale detection / timeout reclaim / scheduler`

## v0.0.76 - 2026-04-11

- Loop `76`
- step `S07`
- `control-plane-api` hardened the shared-channel sync repair seam so stale pending claim ownership is reclaimed before repair dispatch
  - `SocialSharedChannelSyncRepairResponse` now includes `reclaimed`
  - `repair_shared_channel_sync(...)` clears stale ownership before dispatch
  - `control.social_runtime_shared_channel_sync_repaired` audit payload now includes `reclaimed`
  - targeted stale-repair regression now proves pre-repair `leaseStatus = stale` and post-repair `reclaimed = 1`
- TDD:
  - red: `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_repair_reclaims_stale_claim_before_dispatch -- --nocapture`
  - red symptom: `repair_json["reclaimed"] = null`
  - green: stale-aware repair returns `reclaimed = 1`
- fresh verification:
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_repair_reclaims_stale_claim_before_dispatch -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `77 passed`
- artifacts:
  - `docs/step/147-S07-shared-channel-sync-stale-claim-repair-reclaim-loop76-current-checkpoint-2026-04-11.md`
  - `docs/review/S07-Loop76补充-2026-04-11.md`
  - `docs/架构/152CJ-Loop76补充-2026-04-11.md`
  - `docs/release/2026-04-11-v0.0.76-loop-76.md`
- closure:
  - `S07` remains `not_closed / local_closure`
  - remaining main gap: `automatic stale detection / timeout reclaim / scheduler`

## v0.0.75 - 2026-04-11

- Loop `75`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel conflict suggested-action contract`
  - `social_shared_channel_sync_conflict_details(...)` 当前新增 `suggestedAction`
  - 当前最小语汇收敛为 `wait_for_owner_release_or_expiry / takeover_pending_request / takeover_with_legacy_override`
  - claim conflictItems 与 republish/release/takeover conflict details 当前全部复用同一套 suggested-action 语义
  - stale foreign claim 当前虽然仍阻止普通 targeted claim，但当前会显式提示 operator 改走 `takeover_pending_request`
  - 本轮没有改变 allow/deny 语义；补的是 machine-readable remediation hint，而不是新的 stale policy / scheduler
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture`
  - red：初始失败点为 `conflict_items[0]["suggestedAction"] = null`
  - green：active/stale/legacy 三档 suggestedAction 与 package regression 全部通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `75 passed`
- 新增 `docs/step/146-S07-shared-channel-sync-conflict-suggested-action-loop75-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop75补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop75补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.75-loop-75.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已把冲突面从“只有诊断字段”推进到“也有机器可读下一步动作建议”
  - 下一主缺口继续收敛为 `automatic stale detection / timeout reclaim / scheduler`

## v0.0.74 - 2026-04-11

- Loop `74`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel dead-letter metadata parity assertion lock`
  - dead-letter inventory lifecycle 当前新增 direct parity assertions
  - dead-letter inventory item 当前显式锁定 `leaseStatus = unclaimed`
  - dead-letter inventory item 当前显式锁定 `takeoverEligible = false`
  - dead-letter inventory item 当前显式锁定 `legacyTakeoverRequired = false`
  - dead-letter inventory item 当前显式锁定 `ownerActorId / ownerActorKind / claimedAt / leaseExpiresAt = null`
  - 本轮没有改变 runtime 行为；复核后的真实现状是 pending/dead-letter inventory 已通过共享 helper 暴露同一套 lease/takeover metadata
  - 当前仍没有 richer claim batch remediation / suggested-action contract、scheduler、stale-aware repair 与 release-ready exactly-once semantics
- TDD / verification：
  - green：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_dead_letter_inventory_and_targeted_requeue_only_restores_selected_actor -- --nocapture`
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_dead_letter_inventory_and_targeted_requeue_only_restores_selected_actor -- --nocapture` = `passed`
- 新增 `docs/step/145-S07-shared-channel-sync-dead-letter-metadata-parity-loop74-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop74补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop74补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.74-loop-74.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已把 `dead-letter metadata parity` 从隐式 helper 事实推进成显式 regression contract
  - 下一主缺口继续收敛为 `claim richer batch remediation / suggested-action contract`

## v0.0.73 - 2026-04-11

- Loop `73`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel targeted claim conflictItems visibility seam`
  - `SocialSharedChannelSyncPendingClaimResponse` 当前新增 `conflictItems`
  - foreign-owned pending request 在 targeted claim 冲突时，当前会逐 item 回写 `conflictItems`
  - `conflictItems` 当前复用共享 conflict details helper，语汇与 takeover / republish / release owner-conflict 保持一致
  - claim route audit payload 当前也显式带上 `conflictItems`
  - 本轮没有改变 claim 的 status / claimed / conflicted 计数语义；补的是 aggregate response 上的 item-level diagnostics
  - 当前仍没有 dead-letter metadata parity 专项断言、richer claim batch remediation contract、scheduler 与 stale-aware repair policy
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture`
  - red：初始失败点为 `foreign targeted pending claim should expose conflictItems as an array`
  - green：claim conflictItems 与 package regression 全部通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `75 passed`
- 新增 `docs/step/144-S07-shared-channel-sync-claim-conflict-items-loop73-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop73补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop73补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.73-loop-73.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + pending inventory + targeted pending republish + targeted pending claim + claim conflictItems visibility + republish ownership guard + pending targeted release + pending claimedAt visibility + pending targeted takeover + pending leaseExpiresAt visibility + pending stale-lease takeover guard + pending/dead-letter inventory leaseStatus visibility + pending/dead-letter inventory takeoverEligible visibility + legacy untracked takeover explicit override + takeover conflict machine-readable details symmetry + republish/release owner-conflict machine-readable details symmetry + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口继续收敛为 `pending/dead-letter response parity for lease/takeover metadata`

## v0.0.72 - 2026-04-11

- Loop `72`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel republish/release owner-conflict machine-readable details symmetry seam`
  - 新增共享 helper，把 pending item 的 `requestKey / ownerActorId / leaseExpiresAt / leaseStatus / takeoverEligible / legacyTakeoverRequired` 收敛成统一 conflict `details`
  - targeted republish 的 `shared_channel_sync_owner_conflict` 当前显式返回 `details`
  - targeted release 的 `shared_channel_sync_owner_conflict` 当前也显式返回 `details`
  - takeover / republish / release 三条 owner-conflict 写路径当前复用同一 details helper
  - 本轮没有改变 claim / republish / release / takeover 的 allow/deny 语义；补的是 owner-conflict error surface symmetry
  - 当前仍没有 dead-letter metadata parity 专项断言、claim targeted per-item conflict details、scheduler 与 stale-aware repair policy
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture`
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture`
  - red：初始失败点分别为 `foreign_republish_json["details"]["requestKey"] = null` 与 `foreign_release_json["details"]["requestKey"] = null`
  - green：foreign republish conflict、foreign release conflict 与 package regression 全部通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `75 passed`
- 新增 `docs/step/143-S07-shared-channel-sync-owner-conflict-details-parity-loop72-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop72补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop72补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.72-loop-72.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + pending inventory + pending targeted republish + pending targeted claim + republish ownership guard + pending targeted release + pending claimedAt visibility + pending targeted takeover + pending leaseExpiresAt visibility + pending stale-lease takeover guard + pending/dead-letter inventory leaseStatus visibility + pending/dead-letter inventory takeoverEligible visibility + legacy untracked takeover explicit override + takeover conflict machine-readable details symmetry + republish/release owner-conflict machine-readable details symmetry + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口收敛为 `pending/dead-letter response parity for lease/takeover metadata`

## v0.0.71 - 2026-04-11

- Loop `71`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel takeover conflict machine-readable details symmetry seam`
  - `ControlPlaneError` 当前支持可选 `details`
  - active foreign takeover conflict 当前显式返回 `details`
  - legacy override-required conflict 当前也显式返回 `details`
  - `details` 当前至少覆盖 `requestKey / ownerActorId / leaseExpiresAt / leaseStatus / takeoverEligible / legacyTakeoverRequired`
  - operator 当前不再必须回头读 inventory 才能判断 takeover 被拒绝的 lease / owner / legacy override gate 原因
  - 本轮没有改变 takeover allow/deny 语义；补的是 write-path conflict surface symmetry
  - 当前仍没有 dead-letter metadata parity 专项断言、claim/release conflict details parity、scheduler 与 stale-aware repair policy
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture`
  - red：初始失败点为 `active_takeover_json["details"]["requestKey"] = null`
  - green：active conflict 与 legacy override-required conflict 当前都显式返回 machine-readable `details`
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `75 passed`
- 新增 `docs/step/142-S07-shared-channel-sync-takeover-conflict-details-loop71-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop71补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop71补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.71-loop-71.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + pending inventory + pending targeted republish + pending targeted claim + republish ownership guard + pending targeted release + pending claimedAt visibility + pending targeted takeover + pending leaseExpiresAt visibility + pending stale-lease takeover guard + pending/dead-letter inventory leaseStatus visibility + pending/dead-letter inventory takeoverEligible visibility + legacy untracked takeover explicit override + takeover conflict machine-readable details symmetry + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口收敛为 `pending/dead-letter response parity for lease/takeover metadata`

## v0.0.70 - 2026-04-11

- Loop `70`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel legacy untracked takeover explicit override seam`
  - inventory item 当前显式返回 `legacyTakeoverRequired`
  - `takeoverEligible` 当前只代表普通 takeover route 的资格，不再把 foreign `untracked` claim 算进去
  - targeted takeover request 当前新增 `allowLegacyUntracked`
  - 默认 takeover 当前会对 foreign `untracked` claim 返回 `409 shared_channel_sync_legacy_takeover_override_required`
  - 显式 `allowLegacyUntracked = true` 时，legacy compatibility takeover 当前仍可执行
  - takeover response 当前显式返回 `legacyOverrideUsed`
  - 当前仍没有 dead-letter metadata parity 专项断言、scheduler 与 stale-aware repair policy
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture`
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_legacy_untracked_requires_explicit_override -- --nocapture`
  - red：初始失败点分别为 `legacyTakeoverRequired = null` 与 `untracked` foreign claim 仍被普通视为可接管
  - green：普通 stale takeover、legacy explicit override 与 package regression 全部通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_legacy_untracked_requires_explicit_override -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `75 passed`
- 新增 `docs/step/141-S07-shared-channel-sync-legacy-untracked-takeover-override-loop70-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop70补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop70补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.70-loop-70.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + pending inventory + pending targeted republish + pending targeted claim + republish ownership guard + pending targeted release + pending claimedAt visibility + pending targeted takeover + pending leaseExpiresAt visibility + pending stale-lease takeover guard + pending/dead-letter inventory leaseStatus visibility + pending/dead-letter inventory takeoverEligible visibility + legacy untracked takeover explicit override + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口收敛为 `pending/dead-letter response parity for lease/takeover metadata`

## v0.0.69 - 2026-04-11

- Loop `69`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel pending/dead-letter inventory takeoverEligible visibility seam`
  - `SocialSharedChannelSyncInventoryItemResponse` 当前显式返回 `takeoverEligible`
  - `takeoverEligible` 当前按当前 actor、`control.write` 权限与 owner/lease 状态派生
  - read-only viewer 当前稳定看到 `takeoverEligible = false`
  - foreign writer 当前会看到 active foreign claim = `false`、legacy `untracked` foreign claim = `true`、stale foreign claim = `true`
  - takeover 成为新 owner 之后，`takeoverEligible` 当前回到 `false`
  - 本轮没有改变 takeover route 语义；只补 current-operator derived metadata
  - 当前仍没有治理确认后的 legacy `untracked` policy、configurable SLA / lease threshold、scheduler，且 `repair-shared-channel-sync` 仍不是 stale-aware policy
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture`
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture`
  - red：两条 targeted test 初始失败点均为 `takeoverEligible = null`
  - green：current-operator takeover eligibility、Loop61 pending inventory/republish 与 package regression 全部通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_inventory_and_targeted_republish_only_materializes_selected_actor -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `75 passed`
- 新增 `docs/step/140-S07-shared-channel-sync-takeover-eligibility-visibility-loop69-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop69补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop69补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.69-loop-69.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + pending inventory + pending targeted republish + pending targeted claim + republish ownership guard + pending targeted release + pending claimedAt visibility + pending targeted takeover + pending leaseExpiresAt visibility + pending stale-lease takeover guard + pending/dead-letter inventory leaseStatus visibility + pending/dead-letter inventory takeoverEligible visibility + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口收敛为 `legacy untracked claim policy hardening`

## v0.0.68 - 2026-04-11

- Loop `68`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel pending/dead-letter inventory leaseStatus visibility seam`
  - 新增 `SocialSharedChannelSyncLeaseStatus = unclaimed / active / stale / untracked`
  - `PendingSharedChannelSyncRequest` 当前会在读 inventory 时派生 `leaseStatus`
  - `pending-shared-channel-sync` 与 `dead-letter-shared-channel-sync` inventory item 当前显式返回 `leaseStatus`
  - `leaseExpiresAt` 缺失的 legacy claimed item 当前显式返回 `untracked`
  - 本轮没有改变 claim / release / takeover 语义；只补 operator-visible derived metadata
  - 当前仍没有 `takeoverEligible`、configurable SLA / lease threshold、scheduler，且 `repair-shared-channel-sync` 仍不是 stale-aware policy
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture`
  - red：初始失败点为 `leaseStatus = null`
  - green：release lifecycle、takeover lifecycle、Loop62 ownership、Loop63 release lifecycle、Loop61 pending inventory/republish 与 package regression 全部通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_inventory_and_targeted_republish_only_materializes_selected_actor -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --test social_runtime_cli_test -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `passed`
  - note：首次 full package run 曾出现 `control-plane-api cli did not exit within timeout`，fresh rerun 未复现，当前记为 cold-run/load flake
- 新增 `docs/step/139-S07-shared-channel-sync-lease-status-visibility-loop68-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop68补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop68补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.68-loop-68.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + pending inventory + pending targeted republish + pending targeted claim + republish ownership guard + pending targeted release + pending claimedAt visibility + pending targeted takeover + pending leaseExpiresAt visibility + pending/dead-letter inventory leaseStatus visibility + pending stale-lease takeover guard + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口收敛为 `legacy untracked claim policy / takeover eligibility visibility`

## v0.0.67 - 2026-04-11

- Loop `67`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel pending stale-lease takeover guard seam`
  - `takeover-pending-shared-channel-sync-targeted` 当前会阻断 `leaseExpiresAt > now` 的 active foreign claim
  - active foreign lease 当前返回 `409 shared_channel_sync_owner_conflict`
  - expired foreign lease 当前仍可沿用既有 targeted takeover route 完成手工接管
  - `leaseExpiresAt` 缺失的 legacy pending claim 当前保持最小兼容，不会被 stale guard 阻断
  - 当前仍没有 operator-visible stale-status derived field、configurable SLA / lease threshold、scheduler，且 `repair-shared-channel-sync` 仍是粗粒度全量 route
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture`
  - red：初始失败点为 active foreign claim takeover 返回 `200`
  - green：active-lease conflict、expired-lease takeover、Loop62 ownership、Loop63 release lifecycle、Loop61 pending inventory/republish 与 package regression 全部通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_inventory_and_targeted_republish_only_materializes_selected_actor -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `75 passed`
- 新增 `docs/step/138-S07-shared-channel-sync-stale-lease-takeover-guard-loop67-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop67补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop67补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.67-loop-67.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + pending inventory + pending targeted republish + pending targeted claim + republish ownership guard + pending targeted release + pending claimedAt visibility + pending targeted takeover + pending leaseExpiresAt visibility + pending stale-lease takeover guard + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口收敛为 `operator stale-status visibility / explicit stale-evaluated metadata`

## v0.0.66 - 2026-04-11

- Loop `66`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel pending leaseExpiresAt visibility seam`
  - `PendingSharedChannelSyncRequest` 当前新增 durable `leaseExpiresAt`
  - `pending-shared-channel-sync` 与 `dead-letter-shared-channel-sync` inventory item 当前显式返回 `ownerActorId / ownerActorKind / claimedAt / leaseExpiresAt`
  - owner claim 当前会按固定 `15m` lease window 写入 `leaseExpiresAt`
  - targeted takeover 当前会和 `claimedAt` 一起刷新 `leaseExpiresAt`
  - owner-only targeted release 当前会清空 `ownerActorId / ownerActorKind / claimedAt / leaseExpiresAt`
  - 同 owner 重复 claim 当前保留原 `claimedAt / leaseExpiresAt`，不扩展到 stale detection / lease refresh 语义
  - 当前仍没有 stale-age threshold / timeout policy / scheduler，且 `repair-shared-channel-sync` 仍是粗粒度全量 route
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture`
  - red：初始失败点为 `claimed pending inventory item should expose leaseExpiresAt after claim`
  - green：leaseExpiresAt lifecycle 回归、Loop65 takeover、Loop62 ownership、Loop61 pending inventory/republish 与 package regression 全部通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_inventory_and_targeted_republish_only_materializes_selected_actor -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `75 passed`
- 新增 `docs/step/137-S07-shared-channel-sync-pending-lease-expires-at-visibility-loop66-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop66补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop66补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.66-loop-66.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + pending inventory + pending targeted republish + pending targeted claim + republish ownership guard + pending targeted release + pending claimedAt visibility + pending targeted takeover + pending leaseExpiresAt visibility + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口收敛为 `stale-age threshold / operator judgment / SLA semantics`

## v0.0.65 - 2026-04-11

- Loop `65`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel pending targeted takeover seam`
  - 新增 `POST /backend/v3/api/control/social/runtime/takeover-pending-shared-channel-sync-targeted`
  - targeted takeover 当前只接管 `foreign-owned` 的 pending request
  - takeover 成功后会切换 `ownerActorId / ownerActorKind` 并刷新 `claimedAt`
  - stale owner takeover 后再做 targeted republish 当前会继续收到 `409 shared_channel_sync_owner_conflict`
  - unclaimed / self-owned / missing selected request 当前保持 `noop`
  - 当前仍没有 `leaseExpiresAt / SLA`、automatic stale detection / scheduler，且 `repair-shared-channel-sync` 仍是粗粒度全量 route
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture`
  - red：初始失败点为 takeover route 返回 `404`
  - green：takeover 回归、Loop62 ownership、Loop63 release lifecycle、Loop61 pending inventory/republish 与 package regression 全部通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_inventory_and_targeted_republish_only_materializes_selected_actor -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `75 passed`
- 新增 `docs/step/136-S07-shared-channel-sync-pending-takeover-loop65-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop65补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop65补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.65-loop-65.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + pending inventory + pending targeted republish + pending targeted claim + republish ownership guard + pending targeted release + pending claimedAt visibility + pending targeted takeover + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口收敛为 `leaseExpiresAt / stale-age policy / SLA semantics`

## v0.0.64 - 2026-04-11

- Loop `64`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel pending claimedAt visibility seam`
  - `PendingSharedChannelSyncRequest` 当前新增 durable `claimedAt`
  - `pending-shared-channel-sync` 与 `dead-letter-shared-channel-sync` inventory item 当前显式返回 `ownerActorId / ownerActorKind / claimedAt`
  - owner claim 当前会写入 `claimedAt`
  - owner-only targeted release 当前会清空 `ownerActorId / ownerActorKind / claimedAt`
  - 同 owner 重复 claim 当前保留原 `claimedAt`，不扩展到 lease refresh 语义
  - 当前仍没有 `leaseExpiresAt / SLA`、force release / takeover，且 `repair-shared-channel-sync` 仍是粗粒度全量 route
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture`
  - red：初始失败点为 `claimed pending inventory item should expose claimedAt after claim`
  - green：claimedAt 可见性回归、Loop62 ownership 回归、Loop61 pending inventory/republish 回归与 package regression 全部通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_inventory_and_targeted_republish_only_materializes_selected_actor -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `74 passed`
- 新增 `docs/step/135-S07-shared-channel-sync-pending-claimed-at-visibility-loop64-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop64补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop64补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.64-loop-64.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + pending inventory + pending targeted republish + pending targeted claim + republish ownership guard + pending targeted release + pending claimedAt visibility + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口收敛为 `stale-claim takeover / lease / SLA semantics`

## v0.0.63 - 2026-04-11

- Loop `63`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel pending targeted release lifecycle seam`
  - 新增 `POST /backend/v3/api/control/social/runtime/release-pending-shared-channel-sync-targeted`
  - targeted release 当前只允许 owner operator 清空 selected pending request 的 owner 元数据
  - foreign-owned request 的 release 当前返回 `409 shared_channel_sync_owner_conflict`
  - released request 当前会返回 unowned pool，可被其他 operator 再次 claim，并复用既有 targeted republish
  - 当前仍没有 `claimedAt / leaseExpiresAt / SLA`、force release / takeover，且 `repair-shared-channel-sync` 仍是粗粒度全量 route
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture`
  - red：初始失败点为 release route 返回 `404`
  - green：release lifecycle e2e、Loop62 ownership 回归、Loop61 republish 回归与 package regression 全部通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_inventory_and_targeted_republish_only_materializes_selected_actor -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `74 passed`
- 新增 `docs/step/134-S07-shared-channel-sync-pending-release-lifecycle-loop63-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop63补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop63补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.63-loop-63.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + pending inventory + targeted pending republish + pending targeted claim + republish ownership guard + pending targeted release + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口收敛为 `stale-claim / lease / SLA semantics`

## v0.0.62 - 2026-04-11

- Loop `62`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel pending targeted claim + republish ownership seam`
  - 新增 `POST /backend/v3/api/control/social/runtime/claim-pending-shared-channel-sync-targeted`
  - pending/dead-letter inventory item 当前显式返回 `ownerActorId / ownerActorKind`
  - durable `PendingSharedChannelSyncRequest` 当前保留可选 owner 元数据
  - targeted claim 当前使用当前 control operator 的 `actorId / actorKind` 绑定被选中的 pending request
  - targeted republish 当前只允许 owner operator 投递已 claim request；foreign-owned 或 unclaimed request 返回 `409 shared_channel_sync_owner_conflict`
  - Loop61 的 targeted republish 旧回归当前已升级为“先 claim 再 republish”的新合同
  - 当前仍没有 `claimedAt / leaseExpiresAt / SLA`、targeted unclaim / release，且 `repair-shared-channel-sync` 仍是粗粒度全量 route
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture`
  - red：初始失败点为 claim route 返回 `404`
  - green：ownership e2e、Loop61 回归修正与 package regression 全部通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_inventory_and_targeted_republish_only_materializes_selected_actor -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `73 passed`
- 新增 `docs/step/133-S07-shared-channel-sync-pending-claim-ownership-loop62-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop62补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop62补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.62-loop-62.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + pending inventory + targeted pending republish + pending targeted claim + republish ownership guard + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口收敛为 `claim lifetime / lease / SLA semantics`

## v0.0.61 - 2026-04-11

- Loop `61`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel pending inventory + targeted republish seam`
  - 新增 `GET /backend/v3/api/control/social/runtime/pending-shared-channel-sync`
  - pending inventory item 当前显式返回 `requestKey / request / failureCount / lastError`
  - 新增 `POST /backend/v3/api/control/social/runtime/republish-pending-shared-channel-sync-targeted`
  - targeted republish 当前允许按 `requestKey` 只投递选中的 pending request 到 remote runtime
  - 成功投递的选中 request 会从 pending backlog 清理；未选中的 pending request 保持不动
  - 现有 `repair-shared-channel-sync` 全量 operator surface 保持不变
  - 当前仍没有 inventory filter/pagination、delivery lease owner、SLA 或 exactly-once semantics
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_inventory_and_targeted_republish_only_materializes_selected_actor -- --nocapture`
  - red：初始失败点为 pending inventory route 返回 `404`
  - green：pending inventory + targeted republish e2e 与 package regression 回归通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_inventory_and_targeted_republish_only_materializes_selected_actor -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `72 passed`
- 新增 `docs/step/132-S07-shared-channel-sync-pending-targeted-republish-loop61-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop61补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop61补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.61-loop-61.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + pending inventory + targeted pending republish + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口收敛为 `delivery ownership / lease / SLA semantics`

## v0.0.60 - 2026-04-11

- Loop `60`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel dead-letter inventory + targeted requeue seam`
  - 新增 `GET /backend/v3/api/control/social/runtime/dead-letter-shared-channel-sync`
  - inventory item 当前显式返回 `requestKey / request / failureCount / lastError`
  - 新增 `POST /backend/v3/api/control/social/runtime/requeue-dead-letter-shared-channel-sync-targeted`
  - targeted requeue 当前允许按 `requestKey` 只回灌被选中的 dead-letter request
  - 被选中 request 在 targeted requeue 时会把 `failureCount` 重置为 `0`，同时保留 `lastError`
  - 未被选中的 dead-letter request 会继续停留在 `dead_letter_shared_channel_sync_requests`
  - 现有“全量 dead-letter requeue” route 保持不变
  - 当前仍没有 inventory filter/pagination、automatic retry scheduler、remote republish ownership / lease / exactly-once semantics
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_dead_letter_inventory_and_targeted_requeue_only_restores_selected_actor -- --nocapture`
  - red：初始失败点为 dead-letter inventory route 返回 `404`
  - green：dead-letter inventory + targeted requeue e2e 与 package regression 回归通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_dead_letter_inventory_and_targeted_requeue_only_restores_selected_actor -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `71 passed`
- 新增 `docs/step/131-S07-shared-channel-sync-dead-letter-targeted-requeue-loop60-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop60补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop60补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.60-loop-60.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter inventory + dead-letter targeted/all requeue + failure-budget rearm`
  - 下一主缺口收敛为 `remote republish / delivery ownership semantics`

## v0.0.59 - 2026-04-11

- Loop `59`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel dead-letter requeue failure-budget rearm seam`
  - `requeue_dead_letter_shared_channel_sync_requests()` 在 dead-letter -> pending 迁移时会把 `failureCount` 重置为 `0`
  - 同一迁移仍保留 `lastError`
  - requeue 后的首次 repair 失败只会把 request 留在 `pending_shared_channel_sync_requests`，并把 `failureCount` 记为 `1`
  - request 不会因为历史 dead-letter 前的旧失败次数，在 requeue 后第一次失败时立刻回到 dead-letter
  - 当前仍没有 targeted requeue / automatic retry scheduler / remote republish ownership / exactly-once semantics
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_dead_letter_requeue_rearms_failure_budget_before_next_repair_attempt -- --nocapture`
  - red：初始失败点为 requeue 后 pending item 的 `failureCount` 仍为 `3`
  - green：failure-budget rearm e2e 与 package regression 回归通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_dead_letter_requeue_rearms_failure_budget_before_next_repair_attempt -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `70 passed`
- 新增 `docs/step/130-S07-shared-channel-sync-dead-letter-rearm-loop59-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop59补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop59补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.59-loop-59.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter requeue + failure-budget rearm`
  - 下一主缺口收敛为 `requeue selection contract / remote republish / delivery ownership semantics`

## v0.0.58 - 2026-04-11

- Loop `58`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel dead-letter requeue operator seam`
  - 新增 `POST /backend/v3/api/control/social/runtime/requeue-dead-letter-shared-channel-sync`
  - dead-letter request 当前可以从 `dead_letter_shared_channel_sync_requests` 回灌到 `pending_shared_channel_sync_requests`
  - requeue 响应显式返回 `pendingBefore / deadLetterBefore / requeued / pendingAfter / deadLetterAfter`
  - requeue 行为会写入 `control.social_runtime_shared_channel_sync_dead_letter_requeued` audit 事件
  - 现有 `repair-shared-channel-sync` 路径继续承担真正的投递与 backlog 清理
  - 当前 requeue 仍是“全量回灌”语义，且保留 `failureCount / lastError`
  - 当前仍没有 automatic retry scheduler / remote republish ownership / exactly-once semantics
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_dead_letter_requeue_restores_pending_backlog_and_repair_materializes_remote_runtime -- --nocapture`
  - red：初始失败点为 requeue route 返回 `404`
  - green：dead-letter requeue e2e 与 package regression 回归通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_dead_letter_requeue_restores_pending_backlog_and_repair_materializes_remote_runtime -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `69 passed`
- 新增 `docs/step/129-S07-shared-channel-sync-dead-letter-requeue-loop58-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop58补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop58补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.58-loop-58.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + next healthy ready-pair write auto-retry + repeated-failure dead-letter + dead-letter requeue`
  - 下一主缺口收敛为 `automatic retry scheduler / remote republish / delivery ownership semantics`

## v0.0.57 - 2026-04-11

- Loop `57`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel repeated-failure dead-letter seam`
  - `SocialControlState` 新增 `dead_letter_shared_channel_sync_requests`
  - 同一 shared-channel sync request 连续失败达到固定阈值 `3` 后，会从 `pending` 转入 `dead-letter`
  - dead-lettered request 不再进入 `next healthy ready-pair write` 自动重试队列，也不再被 `repair-shared-channel-sync` 继续消费
  - `repair-shared-channel-sync` 响应新增 `deadLetterBefore / deadLettered / deadLetterAfter`
  - `SocialAggregateCountsResponse` 与 `repair-social-runtime-dir` operator report 新增 `deadLetterSharedChannelSyncRequests`
  - `repair-derived-snapshot` 与 `repair-social-runtime-dir` replay social commit journal 时会保留 dead-letter backlog
  - 当前仍没有 dead-letter requeue / retry scheduler / remote republish ownership / exactly-once semantics
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_repeated_failure_moves_request_to_dead_letter_and_stops_repair_retry -- --nocapture`
  - red：初始失败为 `repeated failure should remove the request from pending backlog`
  - green：dead-letter e2e 与 package regression 回归通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_repeated_failure_moves_request_to_dead_letter_and_stops_repair_retry -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `68 passed`
- 新增 `docs/step/128-S07-shared-channel-sync-dead-letter-loop57-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop57补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop57补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.57-loop-57.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + next healthy ready-pair write auto-retry + repeated-failure dead-letter`
  - 下一主缺口收敛为 `automatic retry scheduler / dead-letter requeue / remote republish / delivery ownership semantics`

## v0.0.56 - 2026-04-11

- Loop `56`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel pending backlog next-write auto-retry seam`
  - dispatch 前会先读取 `pending_shared_channel_sync_requests`
  - dispatch 队列当前会合并 `pending backlog + 本次 ready-pair request`，并按 request key 去重
  - 当 trigger 已恢复健康且再次发生 ready pair 写入时，旧 backlog 会在同一轮 dispatch 中先被自动重放
  - 同轮中的本次新请求也会继续投递；成功项会沿既有成功路径 best-effort 清理 backlog
  - 当前仍没有后台 retry scheduler / dead-letter / remote republish ownership / exactly-once semantics
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_backlog_retries_on_next_healthy_ready_pair_write -- --nocapture`
  - red：初始失败为 `next healthy ready-pair write should flush the pending shared-channel sync backlog`
  - green：next-write auto-retry e2e 与 package regression 回归通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_pending_backlog_retries_on_next_healthy_ready_pair_write -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `67 passed`
- 新增 `docs/step/127-S07-shared-channel-sync-next-write-auto-retry-loop56-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop56补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop56补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.56-loop-56.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair + next healthy ready-pair write auto-retry`
  - 下一主缺口收敛为 `automatic retry scheduler / dead-letter / remote republish / delivery ownership semantics`

## v0.0.55 - 2026-04-11

- Loop `55`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel sync durable pending backlog + operator repair seam`
  - `SocialControlState` 新增 `pending_shared_channel_sync_requests`
  - ready pair 存在但 `shared_channel_sync_trigger` 未配置时，请求会被 durable 记入 backlog，而不是静默丢弃
  - trigger dispatch 失败时，失败项与其后的未投递项会被 durable 记入 backlog；social durable truth 保持 committed
  - 新增 `POST /backend/v3/api/control/social/runtime/repair-shared-channel-sync`
  - `repair-derived-snapshot` 与 `repair-social-runtime-dir` replay social commit journal 时会保留 backlog
  - `SocialAggregateCountsResponse` 新增 `pendingSharedChannelSyncRequests`
  - 当前仍没有自动 retry / dead-letter / remote republish / exactly-once semantics
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_sync_failure_persists_pending_work_and_repair_replays_remote_runtime_materialization -- --nocapture`
  - red：初始失败为 `social-state.json` 中不存在 `pending_shared_channel_sync_requests`，且 repair route 缺失
  - green：pending backlog persistence + repair replay e2e 与 package regression 回归通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_sync_failure_persists_pending_work_and_repair_replays_remote_runtime_materialization -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `66 passed`
- 新增 `docs/step/126-S07-shared-channel-sync-pending-repair-loop55-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop55补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop55补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.55-loop-55.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - shared-channel sync 当前已具备 `durable pending backlog + operator repair`，但自动 retry / remote republish owner 仍未闭环
  - 下一主缺口收敛为 `automatic retry / dead-letter / remote republish / delivery ownership semantics`

## v0.0.54 - 2026-04-11

- Loop `54`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `standalone public HTTP runtime consumer wiring`
  - 新增 `PublicSharedChannelLinkedMemberSyncTrigger`
  - 新增 `build_public_shared_channel_sync_trigger(...)`
  - 新增 `build_public_app_with_shared_channel_sync_trigger(...)`
  - 新增 `configured_public_shared_channel_sync_trigger(...)`
  - 新增 `SHARED_CHANNEL_SYNC_TARGET_BASE_URL_ENV`
  - `services/control-plane-api/src/main.rs` 当前会在检测到 `SDKWORK_IM_SHARED_CHANNEL_SYNC_TARGET_BASE_URL` 时装配真实 consumer；未配置时回落到原有 `build_public_app()`
  - shared-channel sync 当前通过 public bearer 身份调用 standalone `conversation-runtime` 的 `/im/v3/api/chat/conversations/shared_channel_links/sync`
  - 当前 bridge 只支持 `http://` public runtime target；`https://` 与 cross-service outbox / retry / remote republish 继续 deferred
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_http_trigger_materializes_remote_runtime_linked_member_over_public_runtime -- --nocapture`
  - red：初始失败为缺少 `build_public_shared_channel_sync_trigger` 与 `build_public_app_with_shared_channel_sync_trigger`
  - clarification：e2e 初版误把 linked member 当作 `/members` active member；最终修正为验证 linked actor 读取 shared history
  - green：standalone public HTTP trigger 与 package regression 回归通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_http_trigger_materializes_remote_runtime_linked_member_over_public_runtime -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `65 passed`
- 新增 `docs/step/125-S07-standalone-control-plane-public-http-consumer-loop54-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop54补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop54补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.54-loop-54.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - standalone `control-plane-api` 现已具备 `ready-pair auto-trigger -> public HTTP sync bridge -> standalone runtime linked history read` 的最小跨进程闭环
  - 下一主缺口收敛为 `cross-service shared-channel sync outbox / retry / delivery ownership boundary`

## v0.0.53 - 2026-04-11

- Loop `53`
- 执行 step：`S07`
- 在 `control-plane-api`、`local-minimal-node`、`projection-service` 落地 `local-minimal embedded real runtime consumer wiring`
  - `control-plane-api` 抽出 embedded control surface builder，并补齐 `runtime_dir + governance_sinks + shared_channel_sync_trigger` 组合 builder
  - `local-minimal-node` 生产依赖 `control-plane-api`，并把 `/backend/v3/api/control/*` 合并进默认/公开装配面
  - `local-minimal-node` 新增同进程 `SharedChannelLinkedMemberSyncTrigger`，直接调用 `ConversationRuntime::sync_shared_channel_linked_member(...)`
  - `projection-service::timeline_from_auth_context(...)` 当前已接受 `can_read_shared_history()` 的 linked member
  - `OpsRuntime` service inventory 现显式包含 `control-plane-api`
  - standalone `control-plane-api::build_app*` / `main.rs` 仍未装配真实 consumer，跨服务 outbox / retry / remote republish 继续 deferred
- TDD：
  - red：`cargo test -p local-minimal-node --offline --test control_plane_social_sync_e2e_test test_local_minimal_profile_control_plane_shared_channel_auto_sync_materializes_runtime_linked_member -- --nocapture`
  - red：初始失败断言为 `left: 404`、`right: 200`
  - red：route merge 后同一测试二次失败为 `left: 403`、`right: 200`
  - red：二次红灯暴露 `projection-service` timeline 鉴权仍只允许 active member
  - green：embedded control-plane route、in-process runtime consumer 与 linked timeline read 回归通过
- fresh verification：
  - `cargo fmt -p local-minimal-node -p control-plane-api -p projection-service` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `64 passed`
  - `cargo test -p projection-service --offline --tests -- --nocapture` = `47 passed`
  - `cargo test -p local-minimal-node --offline --test control_plane_social_sync_e2e_test -- --nocapture` = `1 passed`
- 新增 `docs/step/124-S07-local-minimal-control-plane-runtime-consumer-loop53-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop53补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop53补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.53-loop-53.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - `local-minimal-node` 默认装配面已具备 `control-plane ready-pair auto-trigger -> runtime linked-member sync -> projection timeline read` 的最小闭环
  - 下一主缺口收敛为 `standalone control-plane / cross-service shared-channel sync orchestration`

## v0.0.52 - 2026-04-11

- Loop `52`
- 执行 step：`S07`
- 在 `control-plane-api` 落地 `shared-channel auto sync trigger seam`
  - `external_member_link` durable truth 新增 `localActorKind`
  - 新增 `SharedChannelLinkedMemberSyncRequest`
  - 新增可注入 `SharedChannelLinkedMemberSyncTrigger`
  - `bind_external_member_link(...)` 与 `apply_shared_channel_policy(...)` 现在都会在 ready pair 成立时自动 dispatch sync request
  - success dispatch 会写入 `control.shared_channel_linked_member_sync_triggered` audit anchor
  - 默认 builder 仍未装配真实 trigger implementation，outbox / retry / cross-service republish 继续 deferred
- TDD：
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_external_member_link_write_persists_snapshot_commit_and_audit -- --nocapture`
  - red：初始失败断言为 `left: Null`、`right: "user"`
  - red：`cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_external_member_link_auto_triggers_shared_channel_sync_when_policy_already_exists -- --nocapture`
  - red：初始失败为缺失 trigger seam types 与 builder
  - green：durable `localActorKind` 与双路径 auto-trigger 回归通过
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_external_member_link_write_persists_snapshot_commit_and_audit -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_external_member_link_auto_triggers_shared_channel_sync_when_policy_already_exists -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_external_collaboration_test test_control_plane_social_shared_channel_policy_auto_triggers_shared_channel_sync_for_existing_links -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api -p im-domain-core -p im-domain-events` = `passed`
  - `cargo test -p im-domain-core --offline --tests -- --nocapture` = `38 passed`
  - `cargo test -p im-domain-events --offline --tests -- --nocapture` = `5 passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `64 passed`
- 新增 `docs/step/123-S07-control-plane-auto-sync-trigger-loop52-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop52补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop52补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.52-loop-52.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - `control-plane` 已具备 shared durable truth ready-pair resolution + auto-trigger seam
  - 下一主缺口收敛为 `real assembly wiring for shared-channel sync trigger / runtime consumer`

## v0.0.51 - 2026-04-11

- Loop `51`
- 执行 step：`S07`
- 在 `conversation-runtime` 落地 `shared_channel_policy durable sync seam / runtime linked-member materialization`
  - 新增 `SyncSharedChannelLinkedMemberCommand`
  - 新增 `POST /im/v3/api/chat/conversations/shared_channel_links/sync`
  - sync seam 只允许 `system` actor 调用，并把 shared durable truth 派生 payload materialize 成 `role = guest / state = linked` 的 runtime member
  - linked member 会持久化 `sharedChannelPolicyId / externalConnectionId / externalMemberId` 三元锚点，并在 recovery replay 后继续保留同一 truth
  - `control-plane -> conversation-runtime automatic sync trigger / auto-projection`、per-user notification level 与更强 `im_thread_subscription` durable model 继续 deferred
- TDD：
  - red：`cargo test -p conversation-runtime --offline --test http_smoke_test test_sync_shared_channel_linked_member_over_http_materializes_linked_history_reader -- --nocapture`
  - red：初始失败断言为 `left: 404`、`right: 200`
  - green：shared-channel sync HTTP 回归与 runtime replay 回归通过
- fresh verification：
  - `cargo test -p conversation-runtime --offline --test http_smoke_test test_sync_shared_channel_linked_member_over_http_materializes_linked_history_reader -- --nocapture` = `passed`
  - `cargo test -p conversation-runtime --offline --test conversation_flow_test test_sync_shared_channel_linked_member_materializes_runtime_truth_and_survives_recovery_replay -- --nocapture` = `passed`
  - `cargo fmt -p conversation-runtime -p im-domain-core` = `passed`
  - `cargo test -p conversation-runtime --offline --tests -- --nocapture` = `128 passed`
- 新增 `docs/step/122-S07-shared-channel-policy-durable-sync-loop51-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop51补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop51补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.51-loop-51.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - `shared_channel_policy durable sync seam` 已进入最小可运行闭环
  - 下一主缺口收敛为 `control-plane -> conversation-runtime automatic sync trigger / auto-projection`

## v0.0.50 - 2026-04-11

- Loop `50`
- 执行 step：`S07`
- 在 `projection-service` 与 `notification-service` 落地 `shared fanout / rebuild` 最小 runtime truth
  - `projection-service` 新增 `message_posted_notification_principal_ids_from_auth_context(...)` owner seam
  - 该 seam 在调用者仍需是 active member 的前提下，返回 `joined active members + shared-history-visible linked members`
  - `notification-service::request_message_posted_notifications(...)` 已切换到该 seam；actor 仍在 fanout 阶段排除
  - projection replay / rebuild 继续复用同一 `ConversationMember` 投影记录，因此 shared linked recipients 在重放后仍会进入通知目标集合
  - `shared_channel_policy` durable truth 自动投影、per-user notification level 与更强 `im_thread_subscription` durable model 继续 deferred
- TDD：
  - red：`cargo test -p notification-service --offline --test notification_pipeline_test test_request_message_posted_notifications_includes_shared_linked_recipients_from_projection -- --nocapture`
  - red：初始失败断言为 `left: 1`、`right: 2`
  - red：`cargo test -p projection-service --offline --test timeline_projection_test test_message_posted_notification_principal_ids_from_auth_context_includes_shared_linked_members -- --nocapture`
  - red：初始失败为缺少 `message_posted_notification_principal_ids_from_auth_context` seam
  - green：shared linked recipient fanout 与 projection recipient seam 回归通过
- fresh verification：
  - `cargo test -p notification-service --offline --test notification_pipeline_test test_request_message_posted_notifications_includes_shared_linked_recipients_from_projection -- --nocapture` = `passed`
  - `cargo test -p projection-service --offline --test timeline_projection_test test_message_posted_notification_principal_ids_from_auth_context_includes_shared_linked_members -- --nocapture` = `passed`
  - `cargo fmt -p notification-service -p projection-service` = `passed`
  - `cargo test -p projection-service --offline --tests -- --nocapture` = `47 passed`
  - `cargo test -p notification-service --offline --tests -- --nocapture` = `19 passed`
- 新增 `docs/step/121-S07-shared-fanout-rebuild-runtime-truth-loop50-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop50补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop50补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.50-loop-50.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - `shared fanout / rebuild` 已进入最小可运行闭环
  - 下一主缺口收敛为 `shared_channel_policy auto-projection / durable sync`

## v0.0.49 - 2026-04-11

- Loop `49`
- 执行 step：`S07`
- 在 `conversation-runtime` 落地 `thread root-author subscription / notification runtime truth`
  - `create_thread_conversation*` 不再只 materialize creator owner
  - 当 root message author 与 creator 不同且仍是 parent conversation 的 active member 时，runtime 会把 root author 一并写入 thread active member 集合
  - auto-subscribed root author 会保留 `parentConversationId / rootMessageId / threadRole = root_author` metadata，并初始化默认 read cursor
  - recovery replay 后，root author 仍可在 thread 中发消息；现有 `notification-service` message-posted fanout 无需新增 route，即可消费这条 active member truth
  - 更强 `im_thread_subscription` durable model、per-user unread / notification level 与 shared fanout / rebuild 继续 deferred
- TDD：
  - red：`cargo test -p conversation-runtime --offline --test conversation_flow_test test_create_thread_conversation_auto_subscribes_root_message_author_for_notification_truth -- --nocapture`
  - red：初始失败断言为 `left: 1`、`right: 2`
  - green：thread create、default read cursor、replay 后 root author reply 回归通过
- fresh verification：
  - `cargo test -p conversation-runtime --offline --test conversation_flow_test test_create_thread_conversation_auto_subscribes_root_message_author_for_notification_truth -- --nocapture` = `passed`
  - `cargo fmt -p conversation-runtime -p im-domain-core` = `passed`
  - `cargo test -p im-domain-core --offline --tests -- --nocapture` = `38 passed`
  - `cargo test -p conversation-runtime --offline --tests -- --nocapture` = `126 passed`
  - `cargo test -p notification-service --offline --tests -- --nocapture` = `18 passed`
- 新增 `docs/step/120-S07-thread-subscription-notification-runtime-truth-loop49-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop49补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop49补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.49-loop-49.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - `thread subscription / notification runtime truth` 已进入最小可运行闭环
  - 下一主缺口收敛为 `shared fanout / rebuild`

## v0.0.48 - 2026-04-11

- Loop `48`
- 执行 step：`S07`
- 在 `conversation-runtime` 落地 `retention enforcement runtime truth`
  - 新增 `retention_policy_ref -> retention_class` 派生规则，运行时按 policy ref 最后一段生成 commit retention class
  - `conversation.policy_applied` 的 envelope 不再固定写死 `standard`
  - policy 生效后，后续 `conversation/message mutation` commit 会继承当前 conversation policy 派生出的 `retention_class`
  - recovery replay 后继续发消息时，仍保持相同 retention truth
  - `archive / restore / retention owner` 平台闭环仍保持 `S13` owner，不在本轮虚报
- TDD：
  - red：`cargo test -p conversation-runtime --offline --test conversation_flow_test test_applied_retention_policy_ref_propagates_to_subsequent_message_commit_envelopes -- --nocapture`
  - red：初始失败断言为 `left: "standard"`、`right: "compliance"`
  - green：policy event、后续 message commit、replay 后新消息的 retention propagation 回归通过
- fresh verification：
  - `cargo test -p conversation-runtime --offline --test conversation_flow_test test_applied_retention_policy_ref_propagates_to_subsequent_message_commit_envelopes -- --nocapture` = `passed`
  - `cargo fmt -p conversation-runtime -p im-domain-core` = `passed`
  - `cargo test -p im-domain-core --offline --tests -- --nocapture` = `38 passed`
  - `cargo test -p conversation-runtime --offline --tests -- --nocapture` = `125 passed`
- 新增 `docs/step/119-S07-retention-enforcement-runtime-truth-loop48-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop48补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop48补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.48-loop-48.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - `retention enforcement runtime truth` 已闭合
  - 下一主缺口收敛为 `thread subscription / notification runtime truth`

## v0.0.47 - 2026-04-11

- Loop `47`
- 执行 step：`S07`
- 在 `im-domain-core` 与 `conversation-runtime` 落地 `shared-external history runtime truth`
  - `ConversationPolicy::normalize` 接受 `history_visibility = shared`
  - HTTP `add-member` 允许透传 shared/external 锚点属性
  - 带完整锚点的 external-linked 成员落为 `MembershipState::Linked`
  - external-linked 成员可读取 shared-history，但常规写路径仍被拒绝
  - `shared_channel_policy` 仍未自动投影到 runtime，fanout / rebuild / retention 继续 deferred
- TDD：
  - red：`cargo test -p im-domain-core --offline test_conversation_policy_normalize_accepts_shared_history_visibility -- --nocapture`
  - red：`cargo test -p conversation-runtime --offline --test http_smoke_test test_shared_history_visibility_allows_external_linked_history_reads_but_not_writes_over_http -- --nocapture`
  - green：shared-history domain 归一化与 external-linked HTTP 回归均通过
- fresh verification：
  - `cargo test -p im-domain-core --offline test_conversation_policy_normalize_accepts_shared_history_visibility -- --nocapture` = `passed`
  - `cargo test -p conversation-runtime --offline --test http_smoke_test test_shared_history_visibility_allows_external_linked_history_reads_but_not_writes_over_http -- --nocapture` = `passed`
  - `cargo fmt -p conversation-runtime -p im-domain-core` = `passed`
  - `cargo test -p im-domain-core --offline --tests -- --nocapture` = `38 passed`
  - `cargo test -p conversation-runtime --offline --tests -- --nocapture` = `124 passed`
- 新增 `docs/step/118-S07-shared-external-history-runtime-truth-loop47-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop47补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop47补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.47-loop-47.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - `shared-external history runtime truth` 已闭合
  - 下一主缺口收敛为 `retention enforcement runtime truth`

## v0.0.46 - 2026-04-11

- Loop `46`
- 执行 step：`S07`
- 在 `im-domain-core` 与 `conversation-runtime` 落地 `invited history visibility runtime truth`
  - `ConversationPolicy::normalize` 接受 `history_visibility = invited`
  - invited-history 会话新增成员默认落为 `MembershipState::Invited`
  - invited 成员可在未 join 前读取 invited-history
  - `shared` 继续保持 rejected
- TDD：
  - red：`cargo test -p conversation-runtime --offline --test http_smoke_test test_invited_history_visibility_allows_invited_member_history_reads_before_join_over_http -- --nocapture`
  - green：invited-history HTTP 绿测通过，shared-history 拒绝回归通过
- fresh verification：
  - `cargo test -p conversation-runtime --offline --test http_smoke_test test_invited_history_visibility_allows_invited_member_history_reads_before_join_over_http -- --nocapture` = `passed`
  - `cargo test -p conversation-runtime --offline --test http_smoke_test test_create_conversation_rejects_shared_history_visibility_over_http -- --nocapture` = `passed`
  - `cargo test -p im-domain-core --offline --test conversation_domain_builder_test -- --nocapture` = `18 passed`
  - `cargo fmt -p conversation-runtime -p im-domain-core` = `passed`
  - `cargo test -p im-domain-core --offline --tests -- --nocapture` = `38 passed`
  - `cargo test -p conversation-runtime --offline --tests -- --nocapture` = `124 passed`
- 新增 `docs/step/117-S07-invited-history-visibility-loop46-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop46补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop46补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.46-loop-46.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - `invited history visibility runtime truth` 已闭合
  - 下一主缺口收敛为 `shared-external history runtime truth`

## v0.0.45 - 2026-04-11

- Loop `45`
- 执行 step：`S07`
- 在 `im-domain-core` 与 `conversation-runtime` 落地 `thread minimal runtime truth`：
  - 新增 `ConversationScenario::Thread`
  - 新增 `CreateThreadConversationCommand`
  - 新增 `POST /im/v3/api/chat/conversations/threads`
  - thread 现以 `group conversation + root message` 为锚点，并通过 `businessType = thread / businessId = rootMessageId` 暴露 binding truth
  - thread owner metadata 会保留 `parentConversationId / rootMessageId / threadRole`
  - recovery replay 可恢复 thread binding 与 owner metadata
- TDD：
  - red：`cargo test -p conversation-runtime --offline --test conversation_flow_test test_create_thread_conversation_binds_parent_message_runtime_and_survives_recovery_replay -- --nocapture` 先因缺少 thread command/path 失败
  - green：runtime 与 HTTP thread 回归保持通过
- fresh verification：
  - `cargo test -p conversation-runtime --offline --test conversation_flow_test test_create_thread_conversation_binds_parent_message_runtime_and_survives_recovery_replay -- --nocapture` = `passed`
  - `cargo test -p conversation-runtime --offline --test http_smoke_test test_create_thread_conversation_over_http_and_query_binding -- --nocapture` = `passed`
  - `cargo fmt -p conversation-runtime -p im-domain-core` = `passed`
  - `cargo test -p im-domain-core --offline --tests -- --nocapture` = `36 passed`
  - `cargo test -p conversation-runtime --offline --tests -- --nocapture` = `123 passed`
- 新增 `docs/step/116-S07-thread-runtime-truth-loop45-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S07-Loop45补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop45补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.45-loop-45.md`
- 结论：
  - `S07` 仍是 `not_closed / local_closure`
  - `thread minimal runtime truth` 缺口已关闭
  - 下一主缺口收敛为 `invited history visibility runtime truth`

## v0.0.44 - 2026-04-11

- Loop `44`
- 执行 step：`S05`
- 本轮类型：`收口批次`
- 在 `docs/review`、`docs/架构`、`docs/release` 完成 `S05` 的 closure batch 回写：
  - 把 `S05` 从 `not_closed / local_closure` 提升为 `step_closure`
  - 把 `repair-marker based atomic multi-file tx minimal boundary` 正式接受为当前 step 的充分 durability 边界
  - 把更强 `staged / manifest` 级事务证明下放为 `durability hardening backlog`
  - 把全局主线从 `S05` 切换到 `S07`
- fresh verification：
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `62 passed`
  - `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture` = `64 passed`
- 新增 `docs/step/115-S05-step-closure-loop44-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S05-Loop44补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop44补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.44-loop-44.md`
- 结论：
  - `S05 = step_closure`
  - `release_closure = no`，当前唯一全局剩余 step blocker 为 `S07 = local_closure`
  - stronger `staged / manifest` 级事务证明保留为后续 durability hardening backlog

## v0.0.43 - 2026-04-11

- Loop `43`
- 执行 step：`S05`
- 在 `services/control-plane-api` 把 `repair-marker` 的修复结果显式暴露到 operator surface：
  - `SocialRuntimeRepairResponse` 新增 `transactionMarkerCleared`
  - `POST /backend/v3/api/control/social/runtime/repair-derived-snapshot` 现显式返回本次是否清理了 pending marker
  - `control-plane-api repair-social-runtime-dir --json` 现显式返回 `transactionMarkerCleared`
  - 文本型 CLI 输出新增 `transaction-marker-cleared: <bool>`
- 以 TDD 新增并保持通过：
  - `test_control_plane_repair_social_runtime_dir_cli_reports_transaction_marker_clearance_after_snapshot_failure`
- 同步锁定 HTTP repair：
  - failpoint 路径 `transactionMarkerCleared = true`
  - 外部 journal replay 无 marker 路径 `transactionMarkerCleared = false`
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_runtime_cli_test test_control_plane_repair_social_runtime_dir_cli_reports_transaction_marker_clearance_after_snapshot_failure -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api -p local-minimal-node` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `62 passed`
  - `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture` = `64 passed`
- 新增 `docs/step/114-S05-social-repair-marker-operator-surface-loop43-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S05-Loop43补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop43补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.43-loop-43.md`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`
  - `repair-marker operator-surface visibility` 缺口已关闭
  - 下一主缺口收敛为 `S05 step_closure + stronger staged/manifest tx proof necessity`

## v0.0.42 - 2026-04-11

- Loop `42`
- 执行 step：`S05`
- 在 `services/control-plane-api` 为 social journal + snapshot 写路径补齐 `repair-marker based atomic multi-file tx minimal boundary`：
  - runtime-dir social 写路径现在收敛为 `append social-commit-journal -> write social-transaction-marker -> save social-state -> clear marker`
  - 当 `snapshot save` 失败时，`state/social-transaction-marker.json` 会作为 durable pending marker 保留下来
  - startup replay、same-event retry、HTTP `repair-derived-snapshot` 与 standalone `repair-social-runtime-dir` CLI 在修复成功后都会清除 marker
- 以 TDD 新增并保持通过：
  - `test_control_plane_social_file_runtime_leaves_pending_tx_marker_after_snapshot_failure_and_clears_it_after_restart_replay`
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_friend_request_test test_control_plane_social_file_runtime_leaves_pending_tx_marker_after_snapshot_failure_and_clears_it_after_restart_replay -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api -p local-minimal-node` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `61 passed`
  - `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture` = `64 passed`
- 新增 `docs/step/113-S05-social-transaction-marker-loop42-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S05-Loop42补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop42补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.42-loop-42.md`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`
  - `repair-marker based atomic multi-file tx minimal boundary` 缺口已关闭
  - 下一主缺口收敛为 `S05 step_closure + stronger staged/manifest tx proof`

## v0.0.41 - 2026-04-11

- Loop `41`
- 执行 step：`S05`
- 在 `bin/repair-runtime-local.ps1` 与 `bin/repair-runtime-local.sh` 完成 operator surface unification：
  - wrapper 先执行 `local-minimal-node repair-runtime-dir`
  - 当 `state/social-commit-journal.json` 存在时，自动追加 `control-plane-api repair-social-runtime-dir --runtime-dir <path>`
  - 当 social repair 失败时，wrapper 显式失败并传播退出码，避免 generic repair 成功掩盖 social snapshot 仍未修复的状态
- 在 `services/local-minimal-node/src/node.rs` 补齐 `conversation_runtime::RuntimeError::InvalidInput` 与 `ConversationBindingNotFound` 的 `ApiError` 映射，恢复本轮 `local-minimal-node` regression 的可编译状态
- 将 `docs/prompts/反复执行Step指令.md` 收敛为 concise repeated-step prompt，恢复 `provider_plugin_docs_test` 的文档门禁
- 以 TDD 新增并保持通过：
  - `test_repair_runtime_local_ps1_propagates_social_repair_failure_when_social_journal_exists`
  - `test_repair_runtime_local_sh_invokes_social_repair_after_generic_repair_when_social_journal_exists`
- fresh verification：
  - `cargo test -p local-minimal-node --offline --test deployment_profile_test test_repair_runtime_local_ps1_propagates_social_repair_failure_when_social_journal_exists -- --nocapture` = `passed`
  - `cargo test -p local-minimal-node --offline --test deployment_profile_test test_runtime_operation_ps1_wrappers_forward_profile_and_backup_arguments -- --nocapture` = `passed`
  - `cargo test -p local-minimal-node --offline --test provider_plugin_docs_test test_step_docs_and_prompt_capture_repeated_iteration_requirements -- --nocapture` = `passed`
  - `cargo fmt -p local-minimal-node` = `passed`
  - `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture` = `64 passed`
  - `cargo test -p local-minimal-node --offline --tests -- --nocapture` = `passed`
- 新增 `docs/step/112-S05-social-repair-wrapper-unification-loop41-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S05-Loop41补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop41补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.41-loop-41.md`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`
  - `repair-runtime-local.*` 与 standalone social repair CLI 的 operator entrypoint unification 缺口已关闭
  - 下一主缺口收敛为 `atomic multi-file tx + S05 step_closure`

## v0.0.40 - 2026-04-11

- Loop `40`
- 执行 step：`S05`
- 在 `services/control-plane-api` 新增 standalone social repair CLI：
  - `control-plane-api repair-social-runtime-dir --runtime-dir <path> [--json]`
  - 该命令会在 control-plane 进程外直接 replay `state/social-commit-journal.json`
  - replay 成功后会重建 `state/social-state.json`，并以 JSON/文本输出 repair 结果与 aggregate counts
- 以 TDD 新增并保持通过：
  - `test_control_plane_repair_social_runtime_dir_cli_replays_journal_into_snapshot`
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_runtime_cli_test test_control_plane_repair_social_runtime_dir_cli_replays_journal_into_snapshot -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `60 passed`
- 新增 `docs/step/111-S05-social-standalone-repair-loop40-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S05-Loop40补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop40补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.40-loop-40.md`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`
  - `standalone / cross-process journal-only operator repair` 缺口已关闭
  - 下一主缺口收敛为 `atomic multi-file tx + repair-runtime-local operator surface unification + S05 step_closure`

## v0.0.39 - 2026-04-11

- Loop `39`
- 执行 step：`S05`
- 在 `services/control-plane-api` 把 social operator repair 的 authority 从当前 runtime live-state 收敛到 `social-commit-journal.json`：
  - runtime-dir 模式新增 `journal_path` 持有
  - `POST /backend/v3/api/control/social/runtime/repair-derived-snapshot` 现在直接 replay journal 并刷新 live state + derived snapshot
  - repair 现可吸收 runtime 启动后由外部追加到 journal 的 committed truth
- 以 TDD 新增并保持通过：
  - `test_control_plane_social_file_runtime_operator_repair_replays_external_journal_append_into_live_state`
  - `test_control_plane_social_file_runtime_operator_repair_rebuilds_snapshot_after_failpoint`
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_friend_request_test test_control_plane_social_file_runtime_operator_repair_replays_external_journal_append_into_live_state -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_friend_request_test test_control_plane_social_file_runtime_operator_repair_rebuilds_snapshot_after_failpoint -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `59 passed`
- 新增 `docs/step/110-S05-social-journal-repair-loop39-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S05-Loop39补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop39补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.39-loop-39.md`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`
  - `operator repair 依赖 live-state` 缺口已关闭
  - 下一主缺口收敛为 `atomic multi-file tx + standalone journal-only operator repair/replay tooling + S05 step_closure`

## v0.0.38 - 2026-04-11

- Loop `38`
- 执行 step：`S05`
- 在 `services/control-plane-api` 把 social durable truth 的提交确认边界前移到 `journal append`：
  - 当 `social-commit-journal.json` 已 durable 而 `social-state.json` 写失败时，首次写口不再返回 `social_state_unavailable`
  - 写口直接返回 committed truth，并显式带出 `persistence.journalAuthority + persistence.snapshotStatus`
  - `same-event retry` 在 snapshot 可修复时会把 `snapshotStatus` 收敛回 `current`
- 以 TDD 更新并保持通过：
  - `test_control_plane_social_file_runtime_keeps_direct_chat_pair_guard_after_snapshot_save_failure`
  - `test_control_plane_social_file_runtime_replays_same_event_id_after_snapshot_save_failure`
  - `test_control_plane_social_file_runtime_failpoint_forces_next_snapshot_save_failure_once`
  - `test_control_plane_social_file_runtime_operator_repair_rebuilds_snapshot_after_failpoint`
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_friend_request_test test_control_plane_social_file_runtime_failpoint_forces_next_snapshot_save_failure_once -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_friend_request_test test_control_plane_social_file_runtime_replays_same_event_id_after_snapshot_save_failure -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_friend_request_test test_control_plane_social_file_runtime_keeps_direct_chat_pair_guard_after_snapshot_save_failure -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_friend_request_test test_control_plane_social_file_runtime_operator_repair_rebuilds_snapshot_after_failpoint -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `57 passed`
- 新增 `docs/step/109-S05-social-commit-ack-loop38-current-checkpoint-2026-04-11.md`
- 新增 `docs/review/S05-Loop38补充-2026-04-11.md`
- 新增 `docs/架构/152CJ-Loop38补充-2026-04-11.md`
- 新增 `docs/release/2026-04-11-v0.0.38-loop-38.md`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`
  - `journal-commit ack boundary` 缺口已关闭
  - 下一主缺口收敛为 `atomic multi-file tx + journal-only standalone operator repair/replay tooling + S05 step_closure`

## v0.0.37 - 2026-04-10

- Loop `37`
- 执行 step：`S05`
- 在 `services/control-plane-api` 为 social durable truth 补齐 `snapshot save fail` 的稳定 failpoint 与 operator repair 证据：
  - runtime-dir 新增 `state/social-failpoints.json`
  - 一次性 `failNextSnapshotSave` 可稳定制造 `journal committed + snapshot unavailable`
  - 新增 `POST /backend/v3/api/control/social/runtime/repair-derived-snapshot`
  - operator repair 后可直接回写 `social-state.json`，并在重启后读回 repaired truth
- 以 TDD 新增并保持通过：
  - `test_control_plane_social_file_runtime_failpoint_forces_next_snapshot_save_failure_once`
  - `test_control_plane_social_file_runtime_operator_repair_rebuilds_snapshot_after_failpoint`
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_friend_request_test test_control_plane_social_file_runtime_failpoint_forces_next_snapshot_save_failure_once -- --nocapture` = `passed`
  - `cargo test -p control-plane-api --offline --test social_friend_request_test test_control_plane_social_file_runtime_operator_repair_rebuilds_snapshot_after_failpoint -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `58 passed`
- 新增 `docs/step/108-S05-social-failpoint-repair-loop37-current-checkpoint-2026-04-10.md`
- 新增 `docs/review/S05-Loop37补充-2026-04-10.md`
- 新增 `docs/架构/152CJ-Loop37补充-2026-04-10.md`
- 新增 `docs/release/2026-04-10-v0.0.37-loop-37.md`
- 纠正 `docs/review/S00-S14-全局闭环复核-2026-04-10.md` 中对 `S05` 仍缺 `journal replay` 的过时表述，改为当前真实缺口：`atomic tx boundary + full durable step closure`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`
  - `failpoint/repair evidence` 缺口已关闭
  - 下一主缺口收敛为 `atomic tx boundary + standalone journal-only operator repair/replay tooling`

## v0.0.36 - 2026-04-10

- Loop `36`
- 执行 step：`S05`
- 在 `services/control-plane-api` 建立 social 写路径的 `same-event replay/ack contract`：
  - replay key 收敛为 `(tenant_id, event_id)`
  - 同租户同 `eventId` 且 commit 一致时，直接回放已提交结果
  - same-event replay 不再重复追加 `social-commit-journal.json`
  - 同租户复用相同 `eventId` 但提交不同 commit 时，显式返回 `social_event_id_conflict`
- 以 TDD 新增并保持通过：
  - `test_control_plane_social_file_runtime_replays_same_event_id_after_snapshot_save_failure`
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_friend_request_test test_control_plane_social_file_runtime_replays_same_event_id_after_snapshot_save_failure -- --nocapture` = `passed`
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `56 passed`
- 新增 `docs/step/107-S05-social-event-idempotency-loop36-current-checkpoint-2026-04-10.md`
- 新增 `docs/review/S05-Loop36补充-2026-04-10.md`
- 新增 `docs/架构/152CJ-Loop36补充-2026-04-10.md`
- 新增 `docs/release/2026-04-10-v0.0.36-loop-36.md`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`
  - `same-event retry acknowledgment` 缺口已关闭
  - 下一主缺口收敛为 `failpoint/repair evidence + atomic tx boundary + operator replay tooling`

## v0.0.35 - 2026-04-10

- Loop `35`
- 执行 step：`S05`
- 在 `services/control-plane-api` 收敛 `snapshot save fail` 之后的 live-state 语义：
  - `journal append` 成功后，durable truth 已成立
  - 若 `social-state.json` 保存失败，live state 仍推进到 `next_state`
  - 因此后续 `direct_chat pair guard` 等冲突校验不再继续跑在旧内存上
  - 接口仍返回 `social_state_unavailable`，客户端确认/重试语义仍属 deferred
- 以 TDD 新增并保持通过：
  - `test_control_plane_social_file_runtime_keeps_direct_chat_pair_guard_after_snapshot_save_failure`
- fresh verification：
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `55 passed`
- 新增 `docs/step/106-S05-social-live-state-after-snapshot-failure-loop35-current-checkpoint-2026-04-10.md`
- 新增 `docs/review/S05-Loop35补充-2026-04-10.md`
- 新增 `docs/架构/152CJ-Loop35补充-2026-04-10.md`
- 新增 `docs/release/2026-04-10-v0.0.35-loop-35.md`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`
  - live-state stale gap 已关闭
  - 下一主缺口收敛为 `client ack/idempotency contract / atomic tx boundary / failpoint-repair evidence`

## v0.0.34 - 2026-04-10

- Loop `34`
- 执行 step：`S05`
- 在 `services/control-plane-api` 收敛 runtime-dir social durable truth：
  - startup recovery 改为 `journal-authoritative`
  - 若 `social-commit-journal.json` 存在，则从默认空状态 replay 并回写 `social-state.json`
  - 持久化顺序改为 `append social-commit-journal.json -> save social-state.json`
  - `snapshot ahead of journal` 不再在重启后形成 phantom truth
- 以 TDD 新增并保持通过：
  - `test_control_plane_social_file_runtime_discards_friend_request_snapshot_ahead_of_journal`
  - `test_control_plane_social_file_runtime_discards_direct_chat_snapshot_ahead_of_journal`
- 既有 replay 回归继续通过：
  - `test_control_plane_social_file_runtime_replays_friend_request_when_snapshot_is_missing`
  - `test_control_plane_social_file_runtime_replays_direct_chat_pair_guard_when_snapshot_is_missing`
- fresh verification：
  - `cargo fmt -p control-plane-api` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `54 passed`
- 新增 `docs/step/105-S05-social-crash-consistency-loop34-current-checkpoint-2026-04-10.md`
- 新增 `docs/review/S05-Loop34补充-2026-04-10.md`
- 新增 `docs/架构/152CJ-Loop34补充-2026-04-10.md`
- 新增 `docs/release/2026-04-10-v0.0.34-loop-34.md`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`
  - `snapshot ahead of journal` 缺口已关闭
  - 下一主缺口收敛为 `atomic tx boundary / snapshot-save-fail repair / operator replay tooling`

## v0.0.33 - 2026-04-10

- Loop `33`
- 执行 step：`S05`
- 在 `services/control-plane-api` 落地 runtime-dir social startup replay：
  - 启动时先加载 `state/social-state.json`
  - 再幂等 replay `state/social-commit-journal.json`
  - snapshot 缺失/落后时自修复回写 `social-state.json`
  - replay 覆盖 `friend_request / friendship / user_block / direct_chat / external_connection / external_member_link / shared_channel_policy`
- 在 `crates/im-domain-events` 补齐 replay 所需 payload：
  - `FriendRequestSubmittedPayload.requestMessage`
  - `UserBlockedPayload.expiresAt`
- 以 TDD 新增并保持通过：
  - `test_control_plane_social_file_runtime_replays_friend_request_when_snapshot_is_missing`
  - `test_control_plane_social_file_runtime_replays_direct_chat_pair_guard_when_snapshot_is_missing`
- fresh verification：
  - `cargo fmt -p control-plane-api -p im-domain-events` = `passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `52 passed`
- 新增 `docs/step/104-S05-social-journal-replay-loop33-current-checkpoint-2026-04-10.md`
- 新增 `docs/review/S05-Loop33补充-2026-04-10.md`
- 新增 `docs/架构/152CJ-Loop33补充-2026-04-10.md`
- 新增 `docs/release/2026-04-10-v0.0.33-loop-33.md`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`
  - `journal replay` 缺口已关闭
  - 下一主缺口收敛为 `tx boundary / crash consistency / replay-based repair`

## v0.0.32 - 2026-04-10

- Loop `32`
- 执行 step：`S05`
- 本轮属于 `S05` 收口批次，不新增代码能力，主任务是补齐主架构真相文档回写：
  - 更新 `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
  - 明确 `S05` 当前为“默认内存态 + runtime-dir file-backed 样板”双形态
  - 明确 `state/social-state.json + state/social-commit-journal.json` 已落地
  - 明确当前恢复仍以 snapshot 为准，`social-commit-journal.json` 尚未承担 replay truth
- 新增 `docs/step/103-S05-main-architecture-backwrite-loop32-current-checkpoint-2026-04-10.md`
- 新增 `docs/review/S05-Loop32补充-2026-04-10.md`
- 新增 `docs/架构/152CJ-Loop32补充-2026-04-10.md`
- 新增 `docs/release/2026-04-10-v0.0.32-loop-32.md`
- fresh verification：
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `50 passed`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`
  - `Loop31` 的 durable social truth 已与主架构真相文档完全对齐
  - 下一主阻塞继续收敛为 `journal replay / tx boundary / replay-based repair`

## v0.0.31 - 2026-04-10

- Loop `31`
- 执行 step：`S05`
- 在 `services/control-plane-api` 落地 file-backed social durable 样板：
  - social runtime 收敛为统一 `SocialControlState`
  - 新增 `state/social-state.json`
  - 新增 `state/social-commit-journal.json`
  - `SDKWORK_IM_RUNTIME_DIR` 命中时自动启用 file-backed social store
  - 新增 `build_app_with_cluster_and_governance_sinks_and_runtime_dir(...)`
- 以 TDD 新增并锁定：
  - `test_control_plane_social_file_runtime_restores_friend_request_snapshot_and_outbox`
  - `test_control_plane_social_file_runtime_restores_direct_chat_pair_uniqueness_after_rebuild`
- fresh verification：
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `50 passed`
  - `cargo fmt -p control-plane-api` = `passed`
- 新增 `docs/step/102-S05-durable-social-truth-loop31-current-checkpoint-2026-04-10.md`
- 新增 `docs/review/S05-Loop31补充-2026-04-10.md`
- 新增 `docs/架构/152CJ-Loop31补充-2026-04-10.md`
- 新增 `docs/release/2026-04-10-v0.0.31-loop-31.md`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`，但已不再是纯进程内内存 truth；当前已具备 `file-backed snapshot + social outbox` 最小 durable 样板
  - 当前主阻塞已收敛为 `tx boundary / journal replay / full step closure`
  - `release_closure = no`
## v0.0.30 - 2026-04-10

- Loop `30`
- 执行 step：`S05 || S07`
- 在 `crates/im-domain-core` 与 `services/conversation-runtime` 落地 `direct_chat -> conversation` 最小 runtime binding：
  - 新增 `ConversationBusinessBinding`
  - 新增 `BindDirectChatConversationCommand`
  - 新增 `POST /im/v3/api/chat/conversations/direct_chats/bindings`
  - 新增 `GET /im/v3/api/chat/conversations/{conversationId}`
  - 回放恢复后保持 `direct_chat` 业务绑定与唯一索引
- direct-chat binding 当前要求显式传入 `left/right actor kind`，避免把 runtime 成员错误收窄为固定 `user`
- 以 TDD 补齐并锁定：
  - domain：`conversation_domain_builder_test`
  - runtime：`conversation_flow_test`
  - HTTP：`http_smoke_test`
  - structure：`conversation_domain_structure_test`
- fresh verification：
  - `cargo test -p conversation-runtime --offline --tests -- --nocapture` = `119 passed`
  - `cargo test -p im-domain-core --offline --tests -- --nocapture` = `36 passed`
  - `cargo fmt -p conversation-runtime -p im-domain-core` = `passed`
- 新增 `docs/review/S05-Loop30补充-2026-04-10.md`
- 新增 `docs/review/S07-Loop30补充-2026-04-10.md`
- 新增 `docs/架构/152CJ-Loop30补充-2026-04-10.md`
- 新增 `docs/release/2026-04-10-v0.0.30-loop-30.md`
- 结论：
  - `S05` 仍是 `not_closed / local_closure`，但 `direct_chat -> conversation` 已不再是其未实现缺口，主阻塞收敛为 `durable repo / tx / outbox / replay`
  - `S07` 仍是 `local_closure`，但已完成 `direct_chat -> conversation` runtime binding；当前主缺口收敛为 `thread / shared-external / invited-shared history / retention enforcement`
  - `release_closure = no`

## v0.0.29 - 2026-04-10

- Loop `29`
- 执行 step：`S05 || S07`
- 在 `services/control-plane-api` 落地 `user_block` 最小控制面 truth：
  - 新增 `POST /backend/v3/api/control/social/user-blocks`
  - 新增 `GET /backend/v3/api/control/social/user-blocks/{blockId}`
  - 落地 `user_block.blocked` commit、`control.user_block_blocked` audit、定向 scope 冲突约束
- 以 TDD 补齐 `services/control-plane-api/tests/social_friend_request_test.rs` 的 `user_block` 红绿回归，并确认：
  - `cargo test -p control-plane-api --offline --test social_friend_request_test -- --nocapture` = `8 passed`
  - `cargo test -p control-plane-api --offline --tests -- --nocapture` = `48 passed`
- 重新执行 `cargo test -p conversation-runtime --offline --tests -- --nocapture`，结果 `116 passed`，确认 `S07` 当前仍是：
  - 已具备 `reaction/pin`
  - 已具备 `joined / world_readable` 与 `retentionPolicyRef`
  - 仍缺 `thread / shared/external / direct_chat -> conversation`
- 新增 `docs/review/S05-Loop29补充-2026-04-10.md`
- 新增 `docs/review/S07-Loop29补充-2026-04-10.md`
- 新增 `docs/架构/152CJ-Loop29补充-2026-04-10.md`
- 新增 `docs/release/2026-04-10-v0.0.29-loop-29.md`
- 结论：`S05` 仍是 `not_closed / local_closure`，但 `user_block` 已不再是“未实现”；当前全局 gate 仍被 `S05 durable repo/tx/outbox + direct_chat -> conversation` 与 `S07 thread/shared/external/runtime binding` 阻塞。

## v0.0.28 - 2026-04-10

- Loop `28`
- 执行全局 `S00-S14 release_closure reassessment`
- 新增 `docs/review/S00-S14-全局闭环复核-2026-04-10.md`，把本轮结论冻结为：
  - `release_closure = no`
  - `S05 = not_closed / local_closure`
  - `S07 = local_closure`
- 新增 `docs/架构/152CJ-Loop28补充-2026-04-10.md`，并更新 `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`，显式写明当前全局 gate 仍被 `S05 + S07` 阻塞。
- 新增 `docs/release/2026-04-10-v0.0.28-loop-28.md`，固化 Loop-28 的全局复核、验证结果与下一轮输入。
- 最小验证：
  - `social_friend_request_test = 6 passed`
  - `conversation-runtime --tests = 116 passed`
- 结论：本轮阻塞不是代码回退，而是 `S05 / S07` 仍未达到 step 级闭环；当前不得宣称 `wave_closure / release_closure / 1.0.0`。

## v0.0.27 - 2026-04-10

- Loop `27`
- 执行 step：`S14`
- 修复 app SDK README 合约漂移：
  - `sdks/sdkwork-im-sdk/README.md`
  - `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
  - `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md`
- 补回冻结文本：
  - `payload.json`
  - `session.disconnect / realtime.overload / goaway / resume fallback`
  - `4001 / reconnect_required / pull-only / events.pull`
  - `sdk-release-catalog.json`
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
  - `versionDecisionSourcePath = null`
- 明确 `Step 12` 为 inherited baseline，`S14` 为 `SDK / CLI / release / continuous optimization / step-loop protocol` 闭环，不改写旧测试语义
- 回写：
  - `docs/review/S14-执行卡-2026-04-10.md`
  - `docs/review/S14-Loop27补充-2026-04-10.md`
  - `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
  - `docs/架构/152CJ-Loop27补充-2026-04-10.md`
  - `docs/release/2026-04-10-v0.0.27-loop-27.md`
- fresh verification：
  - `chat_cli_contract_test = 22 passed`
  - `chat_cli_e2e_test = 17 passed`
  - `lib_structure_test = 3 passed`
  - `compatibility_matrix_test = 1 passed`
  - `protocol_governance_test = 1 passed`
  - `protocol_registry_test = 1 passed`
- 诚实结论：
  - `CPR14-1 = yes`
  - `CPR14-2 = yes`
  - `CPR14-3 = yes`
  - `S14 = step_closure`
- Deferred：
  - `release_closure`
  - real SDK publish/version freeze evidence

## v0.0.26 - 2026-04-10

- Loop：`26`
- 执行 step：`S13`
- 更新 `docs/review/S13-执行卡-2026-04-10.md`，补齐出口 6 问并将结论冻结为 `S13 = step_closure`。
- 新增 `docs/review/S13-Loop26补充-2026-04-10.md`，沉淀 deploy/profile、runtime-dir、quant、HA/DR、tier-index 的 fresh 证据。
- 更新 `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`，新增 `docs/架构/152CJ-Loop26补充-2026-04-10.md`，把当前 as-built 明确为：
  - deploy/profile/operator/runtime-ops contract 已真实存在
  - runtime-dir `backup / archive / prune / repair / preview / restore` 已形成可审计恢复链
  - `CI Smoke Tier = fresh real evidence`
  - `Pre-Release Tier = evidence_collected_gate_blocked`
  - `Capacity Tier = template_only_pending_execution`
- 新增 `docs/release/2026-04-10-v0.0.26-loop-26.md`，固化本轮真实结论、评分、下一轮输入与 deferred 边界。
- fresh verification：
  - `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`：`62 passed`
  - `cargo test -p local-minimal-node --offline --test runtime_dir_backup_catalog_test -- --nocapture`：`4 passed`
  - `cargo test -p local-minimal-node --offline --test runtime_dir_repair_test -- --nocapture`：`2 passed`
  - `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`：`10 passed`
  - `cargo test -p local-minimal-node --offline --test runtime_dir_restore_test -- --nocapture`：`3 passed`
  - `cargo test -p local-minimal-node --offline --test performance_quant_baseline_test -- --nocapture`：`4 passed`
  - `cargo test -p local-minimal-node --offline --test performance_ha_dr_drill_test -- --nocapture`：`5 passed`
- 当前可以诚实宣称 `S13 = step_closure`；但不得宣称真实 `capacity-dedicated` 结果、多 cell / 多 region / object storage DR、完整生产级 SLO/告警自动化已闭环。

## v0.0.23 - 2026-04-10

- Loop：`23`
- 执行 step：`S12`
- `crates/im-domain-core/src/social.rs` 新增 `ExternalConnection / ExternalMemberLink / SharedChannelPolicy`，并新增 `ensure_cross_tenant_connection` 跨租户约束。
- `crates/im-domain-events/src/lib.rs` 与 `crates/im-domain-events/src/social.rs` 新增 `external_connection / external_member_link / shared_channel_policy` aggregate type 与对应 event type。
- `services/control-plane-api/src/lib.rs` 新增 external collaboration 最小控制面路由、快照读口与审计动作：
  - `POST/GET /backend/v3/api/control/social/external-connections`
  - `POST/GET /backend/v3/api/control/social/external-member-links`
  - `POST/GET /backend/v3/api/control/social/shared-channel-policies`
  - `control.external_connection_established / control.external_member_link_bound / control.shared_channel_policy_applied`
- `services/control-plane-api/tests/social_external_collaboration_test.rs` 与 `crates/im-domain-core/tests/social_domain_contract_test.rs` 完成 TDD 回归，覆盖 cross-tenant、active connection 依赖与 `history_visibility = shared` 边界。
- 已回写 `docs/review/S12-Loop23补充-2026-04-10.md`、`docs/架构/152CJ-Loop23补充-2026-04-10.md`、`docs/release/2026-04-10-v0.0.23-loop-23.md`。
- 当前结论：`S12` 仅可宣称 `CPR12-3 local_closure`，不得宣称 `S12 step_closure`；`AIoT capability / shared republish / shared runtime / durability` 继续 deferred。

## v0.0.22 - 2026-04-10

- Loop：`22`
- 影响 step：`S11`
- 在 `services/automation-service/src/lib.rs` 新增 `GET /backend/v3/api/automation/governance`，公开 `capabilityProfileId / enabledCapabilities / guardrailPolicyId / restrictedToolPrefixes / operatorOverridePermission / operatorOverrideActive` 最小治理快照。
- 为 `agent-tool-calls` 引入最小 guardrail：`ops.`、`admin.` 前缀的受限 tool 默认拒绝；无 `automation.operator_override` 时返回 `automation_guardrail_denied`，并写入 `automation.guardrail_denied` 事件；具备 override 时追加 `automation.operator_override_applied` 事件后放行。
- 在 `services/local-minimal-node/src/node/build.rs` 与 `services/local-minimal-node/src/node/platform.rs` 镜像 governance surface，并把 `automation.guardrail_denied / automation.operator_override_applied` 写入 assembled audit anchor。
- 以 TDD 新增 `services/automation-service/tests/agent_response_lifecycle_test.rs`、`services/automation-service/tests/http_smoke_test.rs`、`services/local-minimal-node/tests/task10_capabilities_e2e_test.rs` 的 guardrail/override/governance 红绿测试。
- 新增 `docs/review/S11-Loop22补充-2026-04-10.md`、`docs/架构/152CJ-Loop22补充-2026-04-10.md`，并更新 `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`，把 `S11` 从 `local_closure` 提升为 `step_closure`。
- 新增 `docs/release/2026-04-10-v0.0.22-loop-22.md`，固化本轮实现、验证、架构口径与下一轮输入。
- fresh verification：
  - `cargo test -p automation-service --offline test_restricted_tool_call_requires_operator_override_and_is_auditable -- --nocapture`：`1 passed`
  - `cargo test -p automation-service --offline test_automation_governance_surface_and_operator_override_over_http -- --nocapture`：`1 passed`
  - `cargo test -p local-minimal-node --offline test_local_minimal_profile_exposes_automation_governance_and_override_audit -- --nocapture`：`1 passed`
  - `cargo test -p automation-service --offline --tests`：`17 passed`
  - `cargo test -p local-minimal-node --offline --test task10_capabilities_e2e_test`：`5 passed`
- 结论：`CPR11-3` 已以“静态 guardrail policy + permission-derived operator override + public governance surface + assembled audit anchor”闭环，当前可以诚实标注 `S11 = step_closure`；控制面写入/版本化若要引入，属于后续优化，不再阻塞 `S11`。

## v0.0.21 - 2026-04-10

- Loop：`21`
- 影响 step：`S11`
- 在 `services/automation-service/src/lib.rs` 新增 `agent-responses` 与 `agent-tool-calls` 的 HTTP surface，把既有 runtime 的 `start / delta / complete / tool request / tool complete` 生命周期暴露为 standalone 主链。
- 在 `services/local-minimal-node/src/node/build.rs` 与 `services/local-minimal-node/src/node/platform.rs` 镜像同一 assembled surface，并新增 `automation.agent_response_started / delta / completed / automation.agent_tool_call_requested / completed` audit anchor 记录。
- 以 TDD 扩展 `services/automation-service/tests/http_smoke_test.rs` 与 `services/local-minimal-node/tests/task10_capabilities_e2e_test.rs`，先得到 `404` 红灯，再转为 HTTP 主链绿灯；同时修正 `AgentSubject` 请求体字段命名与公开返回类型。
- 新增 `docs/review/S11-Loop21补充-2026-04-10.md`、`docs/架构/152CJ-Loop21补充-2026-04-10.md`，并更新 `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`，把 `S11` 当前口径明确为 `local_closure = HTTP lifecycle surface + assembled audit anchors`。
- 新增 `docs/release/2026-04-10-v0.0.21-loop-21.md`，固化本轮实现、验证、剩余缺口与下一轮输入。
- fresh verification：
  - `cargo test -p automation-service --offline --test http_smoke_test test_agent_response_and_tool_call_lifecycle_over_http`：`1 passed`
  - `cargo test -p local-minimal-node --offline --test task10_capabilities_e2e_test test_local_minimal_profile_exposes_agent_response_and_tool_call_lifecycle_over_http`：`1 passed`
  - `cargo test -p automation-service --offline --tests`：`15 passed`
  - `cargo test -p local-minimal-node --offline --test task10_capabilities_e2e_test`：`4 passed`
  - `cargo test -p local-minimal-node --offline --test websocket_e2e_test test_local_minimal_profile_pushes_agent_handoff_lifecycle_events_over_websocket`：`1 passed`
  - `cargo test -p local-minimal-node --offline --test access_control_e2e_test test_agent_handoff_accept_resolve_close_in_local_profile`：`1 passed`
- 结论：`S11` 已不再缺 HTTP 主链，但 `automation guardrail / operator override` 仍未闭环，因此当前只能诚实标注为 `local_closure`，不能宣称 `step_closure`。

## v0.0.20 - 2026-04-10

- Loop：`20`
- 影响 step：`S10`
- 新增 `docs/review/S10-执行卡-2026-04-10.md`，正式冻结 `S10` 的目标、非目标、写集、CPR 与证据口径，纠正此前“只有准入判断、没有执行卡”的治理缺口。
- 基于 fresh verification 重新审计 `streaming-service`、`im-call-runtime`、`media-service` 与 `local-minimal-node` 的 assembled 路径，确认当前仓库已具备 `CPR10-1/2/3` 的真实闭环，无需再补一个伪需求代码切片。
- 新增 `docs/review/S10-质量审计与复盘-2026-04-10.md`、`docs/review/S10-架构兑现与回写决议-2026-04-10.md`，并更新 `docs/review/S10-准入判断-2026-04-10.md`，把 `S10` 从“可准入”提升为 `step_closure`。
- 更新 `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md` 与 `docs/架构/152CJ-Loop20补充-2026-04-10.md`，将当前最准确的 as-built 表述明确为：
  - standalone service 保持 lifecycle/provider truth
  - conversation-bound assembled binding 归 `local-minimal-node`
  - RTC playback URL 通过选中的 `ObjectStorageProvider` 签发
- 新增 `docs/release/2026-04-10-v0.0.20-loop-20.md`，固化本轮收口、验证与下一轮输入。
- fresh verification：
  - `cargo test -p streaming-service --offline --tests`：`13 passed`
  - `cargo test -p local-minimal-node --test openapi_im_v3_contract_test --offline --tests`：`19 passed`
  - `cargo test -p media-service --offline --tests`：`13 passed`
  - `cargo test -p local-minimal-node --offline --test stream_runtime_persistence_test`：`1 passed`
  - `cargo test -p local-minimal-node --offline --test rtc_runtime_persistence_test`：`1 passed`
  - `cargo test -p local-minimal-node --offline --test media_provider_http_test`：`2 passed`
  - `cargo test -p local-minimal-node --offline --test http_e2e_test`：`36 passed`
  - `cargo test -p local-minimal-node --offline --test access_control_e2e_test`：`33 passed`
- 结论：`S10` 已完成 stream / RTC / media 的 step 级收口；下一并行窗口转入 `S11 || S12`，而 provider failover、HA/DR、retention 与 deployment-grade hardening 继续归 `S13`。

## v0.0.19 - 2026-04-10

- Loop：`19`
- 影响 step：`S09`、`S10`
- 在 `crates/im-domain-core/src/conversation.rs` 收紧 `ConversationPolicy::normalize()`：仅允许 `joined / world_readable`，并对 `invited/shared` 返回显式 unsupported，终止“已发布但未实现”的 history visibility 漂移。
- 在 `crates/sdkwork-im-ccp-registry/src/lib.rs` 把 `businessPolicyVocabulary.historyVisibilityModes` 收敛为 `joined / world_readable`，使 registry、control-plane、runtime 对外口径一致。
- 在 `services/conversation-runtime/src/runtime/policy.rs` 移除 `invited/shared` 的隐式活跃成员语义；若 runtime 读路径遇到 legacy 快照，返回显式 unsupported，而不是继续伪装成受支持模式。
- 以 TDD 扩展 `crates/im-domain-core/tests/conversation_domain_builder_test.rs`、`crates/sdkwork-im-ccp-registry/tests/governance_snapshot_test.rs`、`services/control-plane-api/tests/protocol_governance_test.rs`、`services/conversation-runtime/tests/http_smoke_test.rs`，先证明 domain/registry/control-plane/runtime 都在错误接受或发布未实现模式，再保持通过。
- 更新 `docs/review/S09-*`、`docs/review/S10-准入判断-2026-04-10.md`、`docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`、`docs/架构/152CJ-Loop19补充-2026-04-10.md`、`docs/release/2026-04-10-v0.0.19-loop-19.md`，把 `S09` 正式提升为 `step_closure`，并解除 `S10` 准入阻塞。
- fresh verification：
  - `cargo test -p im-domain-core --offline`：`passed`
  - `cargo test -p conversation-runtime --offline --tests`：`passed`
  - `cargo test -p control-plane-api --offline --tests`：`passed`
  - `cargo test -p sdkwork-im-ccp-registry --offline`：`passed`
  - `cargo test -p im-platform-contracts --offline`：`passed`
- 结论：`S09` 已完成“published policy vocabulary == real runtime support”闭环；`S10` 现已具备真实准入条件，未来若要重新发布 `invited/shared`，必须先完成对应 durable truth。

## v0.0.18 - 2026-04-10

- Loop：`18`
- 影响 step：`S09`、`S10`
- 在 `crates/im-domain-core/src/message.rs` 为 `StoredMessagePin / StoredMessage` 补齐 serde，并新增 `ConversationMessageLog::messages_in_order()`，使最小历史消息快照可直接作为 runtime 读结果输出。
- 在 `services/conversation-runtime/src/runtime.rs`、`services/conversation-runtime/src/runtime/membership.rs` 新增 `MessageHistoryResult`、`list_messages_from_auth_context`、`list_messages`，把历史读取收口到 runtime auth-context 边界。
- 在 `services/conversation-runtime/src/runtime/http.rs` 为 `/im/v3/api/chat/conversations/{conversationId}/messages` 增加 `GET`，形成最小稳定历史消息读取面。
- 在 `services/conversation-runtime/src/runtime/policy.rs` 新增 `ensure_history_read_allowed`，让 `history_visibility` 首次真实影响读路径：`joined` 非成员 `403`，`world_readable` 同租户已鉴权非成员 `200`。
- 以 TDD 扩展 `services/conversation-runtime/tests/conversation_domain_structure_test.rs` 与 `services/conversation-runtime/tests/http_smoke_test.rs`，锁定 runtime read-query auth-context 收口和 `joined/world_readable` 行为差异。
- 更新 `docs/review/S09-*`、`docs/review/S10-准入判断-2026-04-10.md`、`docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`、`docs/架构/152CJ-Loop18补充-2026-04-10.md`、`docs/release/2026-04-10-v0.0.18-loop-18.md`，把 Loop-18 的真实读路径策略消费回写为 as-built。
- fresh verification：
  - `cargo test -p im-domain-core --offline`：`passed`
  - `cargo test -p conversation-runtime --offline --tests`：`passed`
  - `cargo test -p control-plane-api --offline --tests`：`passed`
  - `cargo test -p sdkwork-im-ccp-registry --offline`：`passed`
  - `cargo test -p im-platform-contracts --offline`：`passed`
- 结论：`S09` 获得更强的 `local_closure = runtime history visibility partial read consumption`，但 `invited/shared` 仍折叠为活跃成员读取语义，`retention_policy_ref` 仍归 `S13` owner，因此 `S10` 继续阻塞。

## v0.0.17 - 2026-04-10

- Loop：`17`
- 影响 step：`S09`、`S10`
- 在 `crates/im-domain-core/src/conversation.rs` 新增 `ConversationPolicy`、`policy_epoch` 与 capability 判定，使业务策略快照进入 conversation 聚合真相，而不再只停留在 control-plane 发布层。
- 在 `services/conversation-runtime/src/runtime/governance.rs`、`recovery.rs`、`support.rs` 新增独立事件 `conversation.policy_applied` 及 replay 恢复逻辑，把策略事实从 `conversation.created` 分离出来，形成更清晰的 DDD 治理边界。
- 在 `services/conversation-runtime/src/runtime/http.rs` 让 `POST /im/v3/api/chat/conversations` 支持 `policyVersion / capabilityFlags / historyVisibility / retentionPolicyRef`，并在建会后立即绑定策略。
- 在 `services/conversation-runtime/src/runtime/policy.rs` 让 `capability_flags` 开始真实约束 `message.reaction` 与 `message.pin`，使 `S09` 首次从“词汇发布”推进到“runtime 消费”。
- 以 TDD 扩展 `services/conversation-runtime/tests/conversation_flow_test.rs` 与 `services/conversation-runtime/tests/http_smoke_test.rs`，补齐 replay 与 HTTP mainline 证据。
- 更新 `docs/review/S09-*`、`docs/review/S10-准入判断-2026-04-10.md`、`docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`、`docs/架构/152CJ-Loop17补充-2026-04-10.md`、`docs/release/2026-04-10-v0.0.17-loop-17.md`，把 Loop-17 的真实边界回写为 as-built。
- fresh verification：
  - `cargo test -p im-domain-core --offline`：`32 passed`
  - `cargo test -p conversation-runtime --offline --tests`：`113 passed`
  - `cargo test -p control-plane-api --offline --tests`：`40 passed`
  - `cargo test -p sdkwork-im-ccp-registry --offline`：`2 passed`
  - `cargo test -p im-platform-contracts --offline`：`16 passed`
- 结论：`S09` 获得更强的 `local_closure = runtime capability policy consumption`，但 `history_visibility` 尚无读路径、`retention_policy_ref` 尚无 owner 闭环，故仍不是 `step_closure`；`S10` 继续阻塞。

本文件记录当前项目的版本演进、step 闭环进展与可审计变更。

## v0.0.16 - 2026-04-10

- Loop：`16`
- 影响 step：`S09`、`S10`
- 在 `crates/sdkwork-im-ccp-registry/src/lib.rs` 为 `ProtocolGovernanceSnapshot` 新增 `business_policy_vocabulary`，正式冻结 `policy_version / capability_flags / history_visibility / retention_policy_ref` 字段名，以及 `historyVisibilityModes / retentionPolicyScopes` 的稳定值集合。
- 在 `services/control-plane-api/src/lib.rs` 为 `/backend/v3/api/control/protocol-governance` 新增 `businessPolicyVocabulary` 输出，使协议治理快照对 control-plane reader 与后续 SDK facade 可直接消费，不再只停留在架构文档口径。
- 以 TDD 扩展 `crates/sdkwork-im-ccp-registry/tests/governance_snapshot_test.rs`、`services/control-plane-api/tests/protocol_governance_test.rs`，先得到缺字段红灯，再转协议快照与 JSON surface 绿灯。
- 更新 `docs/review/S09-*`、`docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`、`docs/架构/152CJ-Loop16补充-2026-04-10.md`、`sdks/README.md`、`sdks/sdkwork-im-sdk/README.md`、`sdks/sdkwork-control-plane-sdk/README.md`、`docs/release/2026-04-10-v0.0.16-loop-16.md`，把 `S09` 的协议词汇发布面回写为当前 as-built。
- fresh verification：
  - `cargo test -p control-plane-api --offline --tests`：`40 passed`
  - `cargo test -p sdkwork-im-ccp-registry --offline`：`2 passed`
  - `cargo test -p im-platform-contracts --offline`：`16 passed`
- 结论：`S09` 获得 `local_closure = protocol business policy vocabulary publication`，但 `step_closure` 仍待 runtime/business model 对齐；`S10` 继续被 `S09` 阻塞。

## v0.0.15 - 2026-04-10

- Loop：`15`
- 影响 step：`S08`、`S10`
- 在 `services/projection-service` 新增 `reaction/pin summary` 派生面：`src/interactions.rs` 按 `message.reaction_added`、`message.reaction_removed`、`message.pin_added`、`message.pin_removed` 维护 `MessageInteractionSummaryView / pinned_messages`
- 补齐 `src/http.rs`、`src/access.rs`、`src/snapshot.rs`、`src/scope.rs` 与 `src/lib.rs` 装配，使 `interaction-summary`、`pins`、snapshot restore、client-route sync fanout 全部进入同一投影闭环
- 以 TDD 扩展 `timeline_projection_test.rs`、`http_smoke_test.rs`、`projection_snapshot_test.rs`、`lib_structure_test.rs`，锁定 interaction summary、pins query、snapshot restore 与 `lib.rs` 红线
- 更新 `docs/review/S08-Loop15补充-2026-04-10.md`、`docs/review/S10-Loop15补充-2026-04-10.md`、`docs/架构/152CJ-Loop15补充-2026-04-10.md`、`docs/release/2026-04-10-v0.0.15-loop-15.md`，并修正 `152CJ` 主文中过期的“缺 reaction/pin summary”断言
- fresh verification：
  - `cargo test -p projection-service --offline --tests`：`46 passed`
  - `cargo test -p notification-service --offline --tests`：`18 passed`
- 结论：`S08` 提升为 `step_closure`；`S10` 当前阻塞收敛为仅剩 `S09 step_closure`

## v0.0.14 - 2026-04-10

- Loop：`14`
- 影响 step：`S07`、`S08`、`S10`
- 在 `crates/im-domain-core/src/message.rs` 增补 `MessageReactionAdded`、`MessageReactionRemoved`、`MessagePinned`、`MessageUnpinned`，并把 reaction/pin 状态正式纳入 `StoredMessage / ConversationMessageLog`，使 `conversation-runtime` 具备最小 runtime truth。
- 在 `services/conversation-runtime/src/runtime.rs` 新增 `AddMessageReactionCommand`、`RemoveMessageReactionCommand`、`PinMessageCommand`、`UnpinMessageCommand` 及对应结果模型、鉴权构造器与运行时入口，统一收口 message-level mutation。
- 在 `services/conversation-runtime/src/runtime/policy.rs` 新增 reaction/pin 权限校验：reaction 维持活跃成员可操作，pin 限制为群 owner/admin、直聊 owner。
- 在 `services/conversation-runtime/src/runtime/support.rs`、`services/conversation-runtime/src/runtime/recovery.rs`、`services/conversation-runtime/src/runtime/http.rs` 补齐 `message.reaction_added`、`message.reaction_removed`、`message.pin_added`、`message.pin_removed` 的 envelope、replay 与 HTTP surface。
- 新增 HTTP 接口：
  - `POST /im/v3/api/chat/conversations/{conversationId}/messages/{messageId}/interaction_summary`
  - `POST /im/v3/api/chat/conversations/{conversationId}/messages/{messageId}/interaction_summary`
  - `POST /im/v3/api/chat/conversations/{conversationId}/pins`
  - `POST /im/v3/api/chat/conversations/{conversationId}/pins`
- 以 TDD 扩展 `services/conversation-runtime/tests/authority_command_test.rs`、`services/conversation-runtime/tests/conversation_flow_test.rs`、`services/conversation-runtime/tests/http_smoke_test.rs`，覆盖授权建模、幂等、权限、recovery replay 与 HTTP mainline。
- 更新 `docs/review/S07-*`、`docs/review/S08-Loop14补充-2026-04-10.md`、`docs/review/S10-Loop14补充-2026-04-10.md`、`docs/架构/152CJ-Loop14补充-2026-04-10.md`、`docs/release/2026-04-10-v0.0.14-loop-14.md`，把 `reaction/pin` 明确回写为 `conversation-runtime` 的 as-built runtime truth。
- fresh verification：
  - `cargo test -p conversation-runtime --offline --tests`：`111 passed`
- 结论：`S07 reaction/pin truth` 完成最小闭环；`S08` 的唯一剩余缺口收敛为 `projection-service reaction/pin summary`；`S10` 继续等待 `S08 + S09` 同时收口。

## v0.0.13 - 2026-04-10

- Loop：`13`
- 影响 step：`S08`、`S10`
- 在 `services/projection-service` 新增 `contacts read model`，显式消费 `friendship.activated` 建立双向好友联系人视图，并通过 `direct_chat.bound` 回填 `directChatId / conversationId / lastInteractionAt`。
- 新增 `GET /im/v3/api/chat/contacts` 鉴权查询入口，补齐 `contacts` 的 access、HTTP surface、snapshot persist/restore 与乱序 direct-chat binding shadow 回填。
- 为守住 Step-02 结构红线，将 `record_projection_update_delay_for_scope` 从 `services/projection-service/src/lib.rs` 抽出到 `services/projection-service/src/update_delay.rs`，保持 `lib.rs < 1000`。
- 扩展 `services/projection-service/tests/http_smoke_test.rs` 与 `services/projection-service/tests/projection_snapshot_test.rs`，以 TDD 先拿 `404 / missing method` 红灯，再转 `contacts query` 与 `contacts snapshot restore` 绿灯。
- 更新 `docs/review/S05-*`、`docs/review/S08-执行卡-2026-04-10.md`、`docs/review/S08-质量审计与复盘-2026-04-10.md`、`docs/review/S08-架构兑现与回写决议-2026-04-10.md`、`docs/review/S10-准入判断-2026-04-10.md`、`docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`，正式把 `contacts` 回写为 `projection-service` 的当前 as-built 能力，并消除 `S05` 文档中的旧阻塞口径。
- 新增 `docs/release/2026-04-10-v0.0.13-loop-13.md`，记录 `Loop-13` 的实现、验证、回写与下一轮输入。
- fresh verification：
  - `cargo test -p projection-service --offline --tests`：`43 passed`
  - `cargo test -p notification-service --offline --tests`：`18 passed`
- 结论：`S08` 仍 `not_closed`，但剩余缺口已收敛为单项 `reaction/pin summary`；`S10` 继续 `暂不准入`；下一主线应进入 `S07 reaction/pin truth`，如有并行车道可同步推进 `S05 user_block`。

## v0.0.12 - 2026-04-10

- Loop：`12`
- 影响 step：`S05`、`S08`
- 在 `services/control-plane-api` 新增最小 `direct_chat` truth 写链路：`POST /backend/v3/api/control/social/direct_chats/bindings` 与 `GET /backend/v3/api/control/social/direct_chats/{direct_chat_id}`，落地进程内 `direct_chat` runtime、snapshot readback、camelCase commit surface。
- 服务层真实接入 `im-domain-core::social` 做 actor pair 归一化与 active pair 唯一性校验，真实接入 `im-domain-events::social` 生成 `direct_chat.bound` commit，并记录 `control.direct_chat_bound` audit。
- 扩展 `services/control-plane-api/tests/social_friend_request_test.rs`，TDD 先得到 `404/404` 红灯，再转 `6 passed` 绿灯。
- fresh 重跑 `projection-service`，确认 `S08` 基线仍稳定；同时修正文档口径：`contacts read model` 不再缺 `S05 friendship/direct_chat` 上游真值，而是进入 `projection-service` 本地实现缺口。
- 更新 `docs/review/S05-执行卡-2026-04-10.md`、`docs/review/S05-质量审计与复盘-2026-04-10.md`、`docs/review/S05-架构兑现与回写决议-2026-04-10.md`、`docs/review/S08-执行卡-2026-04-10.md`、`docs/review/S08-质量审计与复盘-2026-04-10.md`、`docs/review/S08-架构兑现与回写决议-2026-04-10.md`、`docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`，正式把当前 social truth 口径提升为“contract + event + minimal control-plane write path”，覆盖 `friend_request + friendship + direct_chat`。
- 新增 `docs/release/2026-04-10-v0.0.12-loop-12.md`，记录 `Loop-12` 的真实实现、验证、未闭环边界与下一轮输入。
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_friend_request_test`：`6 passed`
  - `cargo test -p control-plane-api --offline --tests`：`40 passed`
  - `cargo test -p projection-service --offline --tests`：`41 passed`
- 结论：`S05` 仍 `not_closed`；当前 social truth 已覆盖 `friend_request + friendship + direct_chat`，但仍缺 `user_block`、持久化/outbox 与 `direct_chat -> conversation` runtime 真绑定。`S08 contacts read model` 已从“上游阻塞”转为“本地实现缺口”。

## v0.0.11 - 2026-04-10

- Loop：`11`
- 影响 step：`S05`、`S08`
- 在 `services/control-plane-api` 新增最小 `friendship` truth 写链路：`POST /backend/v3/api/control/social/friendships` 与 `GET /backend/v3/api/control/social/friendships/{friendshipId}`，落地进程内 `friendship` runtime、snapshot readback、camelCase commit surface。
- 服务层真实接入 `im-domain-core::social` 做 pair 约束与 active pair 唯一性校验，真实接入 `im-domain-events::social` 生成 `friendship.activated` commit，并记录 `control.friendship_activated` audit。
- 扩展 `services/control-plane-api/tests/social_friend_request_test.rs`，TDD 先得到 `404/404` 红灯，再转 `4 passed` 绿灯。
- 更新 `docs/review/S05-执行卡-2026-04-10.md`、`docs/review/S05-质量审计与复盘-2026-04-10.md`、`docs/review/S05-架构兑现与回写决议-2026-04-10.md`、`docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`，正式把 `S05` 的当前 social truth 口径提升为“contract + event + minimal control-plane write path”，覆盖 `friend_request + friendship`。
- 新增 `docs/release/2026-04-10-v0.0.11-loop-11.md`，记录 `Loop-11` 的真实实现、验证、未闭环边界与下一轮输入。
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_friend_request_test`：`4 passed`
  - `cargo test -p control-plane-api --offline --tests`：`38 passed`
- 结论：`S05` 仍 `not_closed`；当前 social truth 已覆盖 `friend_request + friendship`，但仍缺 `direct_chat/user_block`、持久化/outbox 与 contacts projection 输入。

## v0.0.10 - 2026-04-10

- Loop：`10`
- 影响 step：`S05`、`S08`
- 在 `services/control-plane-api` 新增最小 social truth 写链路：`POST /backend/v3/api/control/social/friend-requests` 与 `GET /backend/v3/api/control/social/friend-requests/{requestId}`，落地进程内 social runtime、snapshot readback、camelCase commit surface。
- 服务层真实接入 `im-domain-core::social` 做 pair 校验，真实接入 `im-domain-events::social` 生成 `friend_request.submitted` commit，并记录 `control.friend_request_submitted` audit。
- 新增 `services/control-plane-api/tests/social_friend_request_test.rs`，TDD 先得到 `404/404` 红灯，再转 `2 passed` 绿灯。
- 更新 `docs/review/S05-执行卡-2026-04-10.md`、`docs/review/S05-质量审计与复盘-2026-04-10.md`、`docs/review/S05-架构兑现与回写决议-2026-04-10.md`、`docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`，正式把 `S05` 的 local closure 提升为“contract + event + minimal control-plane write path”。
- 新增 `docs/release/2026-04-10-v0.0.10-loop-10.md`，记录 `Loop-10` 的真实实现、验证、未闭环边界与下一轮输入。
- fresh verification：
  - `cargo test -p control-plane-api --offline --test social_friend_request_test`：`2 passed`
  - `cargo test -p control-plane-api --offline --tests`：`36 passed`
- 结论：`S05` 仍 `not_closed`；当前 social truth 已进入控制面，但仍缺 `friendship/direct_chat/user_block`、持久化/outbox 与 contacts projection 输入。

## v0.0.9 - 2026-04-10

- Loop：`09`
- 影响 step：`S05`、`S08`
- 在 `im-domain-events` 新增 `social` 事件出口层，落地 social aggregate type、`SocialEventType`、payload schema 与 `social_commit_envelope` helper，使 `friend_request / friendship / user_block / direct_chat` 首次具备统一事件口径。
- 新增 `crates/im-domain-events/tests/social_commit_envelope_test.rs`，以 TDD 锁定 aggregate type wire value、dotted event type、payload 序列化形状和 envelope 构造规则，并额外修正 `user_block.blocked` 命名一致性。
- 更新 `docs/review/S05-执行卡-2026-04-10.md`、`docs/review/S05-质量审计与复盘-2026-04-10.md`、`docs/review/S05-架构兑现与回写决议-2026-04-10.md`、`docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`，正式把 `S05` 的 local closure 从“仅领域合同层”提升为“领域合同层 + 事件出口层”。
- 新增 `docs/release/2026-04-10-v0.0.9-loop-09.md`，记录 `Loop-09` 的局部闭环、剩余阻塞与下一轮 `control-plane-api` 入口。
- fresh verification：
  - `cargo test -p im-domain-events --offline --test social_commit_envelope_test`：`4 passed`
  - `cargo test -p im-domain-events --offline --tests`：`5 passed`
- 结论：`S05` 仍 `not_closed`，但 `domain events` 已不再是缺口；剩余主阻塞已收敛为 `control-plane-api` 写入口与 `projection-service contacts` 输入。

## v0.0.8 - 2026-04-10

- Loop：`08`
- 影响 step：`S05`、`S08`
- 在 `im-domain-core` 新增 `social` 关系域合同层，落地 `FriendRequest`、`Friendship`、`FriendshipEvent`、`UserBlock`、`DirectChat` 以及 `NormalizedUserPair / NormalizedActorPair / pair_hash` 不变量，正式把 `S05` 从纯文档设计推进为可执行代码资产。
- 新增 `crates/im-domain-core/tests/social_domain_contract_test.rs`，以 TDD 方式锁定 pair 归一化、自指拒绝、状态 helper、序列化形状；`cargo test -p im-domain-core --offline --tests` fresh 全绿。
- 新增 `docs/review/S05-执行卡-2026-04-10.md`、`docs/review/S05-质量审计与复盘-2026-04-10.md`、`docs/review/S05-架构兑现与回写决议-2026-04-10.md`，并回写 `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`，明确“已具备 social 合同层，但尚未具备 durable truth 写链路”。
- 新增 `docs/release/2026-04-10-v0.0.8-loop-08.md`，记录 `Loop-08` 的 local closure、剩余阻塞与下轮入口。
- fresh verification：
  - `cargo test -p im-domain-core --offline --test social_domain_contract_test`：`4 passed`
  - `cargo test -p im-domain-core --offline --tests`：`32 passed`
- 结论：`S05` 仍 `not_closed`，但关系域合同基础层已完成 `local_closure`；`S08 contacts read model` 继续等待 `S05` 的 control-plane/event/projection 真值输入。

## v0.0.7 - 2026-04-10

- Loop：`07`
- 影响 step：`S08`、`S10`
- 在 `projection-service` 落地 `member_directory_projection`，新增目录读模型、鉴权读口与 snapshot restore 闭环，并将 `lib.rs` 继续保持在 Step-02 结构红线之下。
- 更新 `docs/review/S08-执行卡-2026-04-10.md`、`docs/review/S08-质量审计与复盘-2026-04-10.md`、`docs/review/S08-架构兑现与回写决议-2026-04-10.md`、`docs/review/S10-准入判断-2026-04-10.md`、`docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`，正式把 `member_directory_projection` 从 blocker 清单移出。
- 新增 `docs/release/2026-04-10-v0.0.7-loop-07.md`，记录 `Loop-07` 的局部闭环、剩余阻塞与下一轮入口。
- fresh verification：
  - `projection-service`：`41 passed`
  - `notification-service`：`18 passed`
  - `friendship/direct_chat truth search`：`NO_MATCH`
  - `reaction/pin truth search`：`NO_MATCH`
- 结论：`S08` 仍 `not_closed`，但本地实现缺口已收敛为 0；剩余仅为 `S05/S07` 上游真值阻塞，`S10` 继续 `暂不准入`。

## v0.0.6 - 2026-04-10

- Loop：`06`
- 聚焦 step：`S08`
- 新增 `docs/review/S08-质量审计与复盘-2026-04-10.md`，基于本轮新鲜测试与代码检索重新审计 `S08`，确认 `projection / notification` 质量稳定，但 `contacts read model`、`reaction/pin summary`、`member_directory_projection` 仍缺闭环证据。
- 新增 `docs/review/S08-架构兑现与回写决议-2026-04-10.md`，正式冻结 `S08.status = not_closed`，并把三项缺口回挂为 `CPR08-4/5/6`。
- 新增 `docs/release/2026-04-10-v0.0.6-loop-06.md`，记录 `Loop-06` 状态、验证结果、评分、下一轮输入。
- fresh verification：
  - `projection-service`：`39 passed`
  - `notification-service`：`18 passed`
  - `missing-scope search`：`NO_MATCH`
- 结论：`S10` 准入状态不变，仍需等待 `S08 + S09` 同时完成 Step 关闭。

## v0.0.5 - 2026-04-10

- Loop：`05`
- 影响 step：`S08`、`S09`、`S10`
- 新增 `docs/review/S08-执行卡-2026-04-10.md`，把新 `S08` 正式冻结为“projection / notification / read model” step，并明确旧 `step-08` 资料不再作为新编号闭环证据。
- 新增 `docs/review/S10-准入判断-2026-04-10.md`，基于 fresh evidence 给出正式结论：`S10` 代码面可运行，但因 `S08 + S09` 未收口而 `暂不准入`。
- fresh 执行 `projection-service`、`notification-service`、`control-plane-api`、`sdkwork-im-ccp-registry`、`im-platform-contracts`、`streaming-service`、`im-call-runtime`、`media-service`，把 `S08/S09/S10` 的判断统一建立在本轮证据上。
- 新增 `docs/release/2026-04-10-v0.0.5-loop-05.md`，固化本轮准入判断、阻塞项、评分与下一轮输入。

## v0.0.4 - 2026-04-10

- Loop：04
- 影响 step：`S09`
- 新增 `docs/review/S09-质量审计与复盘-2026-04-10.md`，把 `S09` 的 fresh tests、剩余风险和下一步收口路径沉淀为正式质量结论。
- 新增 `docs/review/S09-架构兑现与回写决议-2026-04-10.md`，明确当前实现仍在 `152CJ` 的 as-built 边界内，无需回写主架构文档。
- 更新 `docs/review/S09-执行卡-2026-04-10.md`，把 `Q6` 从缺少 review 支撑推进为 `yes`，并把未闭环原因收敛到 `Q1` 与 `S10` 准入联判。

## v0.0.3 - 2026-04-10

- Loop：03
- 影响 step：`S09`、`S14`
- fresh 执行 `cargo test -p sdkwork-im-ccp-registry --offline` 与 `cargo test -p im-platform-contracts --offline`，确认 `S09` 的 registry / contract / provider policy 基线具备可执行证据。
- 在 `services/control-plane-api/tests/protocol_governance_test.rs` 新增 `protocolGovernancePath` 断言，锁定 `sdkCompatibilityBaseline` 的控制面治理路径。
- 更新 `sdks/README.md`、`sdks/sdkwork-im-sdk/README.md`、`sdks/sdkwork-control-plane-sdk/README.md`，使 SDK 文档与 `sdkCompatibilityBaseline` 的 facade、registry、governance、matrix 口径完全一致。

## v0.0.2 - 2026-04-10

- Loop：02
- 影响 step：`S00`、`S09`、`S14`
- 新增 `docs/review/S09-执行卡-2026-04-10.md`，把当前协议治理 / registry / provider baseline 实现波次正式接入新 `Sxx` 执行体系。
- 更新 `docs/review/README.md`，明确新 `Sxx` 执行卡与历史 `step-XX` 执行卡的并存规则。
- 新增 `docs/release/2026-04-10-v0.0.2-loop-02.md`，固定本轮状态、缺口分级、评分基线与下一轮验证入口。
- fresh 执行 `cargo test -p control-plane-api --offline --tests` 通过，确认当前 `S09` 控制面协议 / 治理 / provider surface 具备可执行基线。

## v0.0.1 - 2026-04-10

- Loop：01
- 影响 step：`S00`、`S14`
- 建立 `docs/release` 目录、changelog 规则与版本化 loop 文档机制。
- 重写循环执行提示词，使其对齐 `S00-S14`、`100-*`、`101-*`、`95`、`97`。
- 固化“先批量实现、后逐 step 测试收口、每轮强制更新 changelog”的执行协议。
