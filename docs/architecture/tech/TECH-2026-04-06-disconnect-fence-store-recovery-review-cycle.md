> Migrated from `docs/review/2026-04-06-disconnect-fence-store-recovery-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-06 Disconnect Fence Store Recovery Review Cycle

## 1. Finding

### 1.1 High: reconnect-required fences disappeared when the cluster bridge was rebuilt

- Affected services:
  - `services/session-gateway`
  - `services/sdkwork-im-cloud-gateway`
- Root cause:
  - Standard 88 introduced a reconnect-required fence after `session.disconnect`
  - but `RealtimeClusterBridge` stored that fence only in an in-memory `HashMap`
  - a new bridge instance had no restore path for disconnect intent

That meant the reconnect contract was correct only while the current bridge process stayed alive.

## 2. Impact

- after a process rebuild, restart, or active-passive takeover, a stale pre-disconnect session could attempt client-route-bound traffic again
- `session.disconnect` semantics were therefore weaker than route/presence semantics suggested
- SDKs and operators could see a platform that looked disconnected before restart but silently lost that boundary after recovery

For a commercial IM platform, explicit disconnect intent must be recoverable. Otherwise restart is a lifecycle escape hatch.

## 3. Reproduction

Red coverage was added first in three places:

- `services/session-gateway/src/cluster.rs`
  - `test_disconnect_fence_survives_bridge_rebuild_with_shared_store`
- `services/session-gateway/tests/http_smoke_test.rs`
  - `test_session_gateway_rebuild_preserves_reconnect_required_fence_until_fresh_resume`
- `services/sdkwork-im-cloud-gateway/tests/http_e2e_test.rs`
  - `test_local_minimal_profile_rebuild_preserves_reconnect_required_fence_until_fresh_resume`

Red evidence:

- `cargo test -p session-gateway --offline disconnect_fence_survives_bridge_rebuild_with_shared_store -- --nocapture`
  - failed because there was no disconnect fence store adapter and no `with_disconnect_fence_store(...)` constructor
- `cargo test -p session-gateway --offline test_session_gateway_rebuild_preserves_reconnect_required_fence_until_fresh_resume -- --exact --nocapture`
  - failed because there was no injectable cluster builder for restart-style verification
- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_rebuild_preserves_reconnect_required_fence_until_fresh_resume -- --exact --nocapture`
  - failed for the same missing persistence seam

Those failures exposed the real gap: the codebase had no persistence/recovery boundary for disconnect fences.

## 4. Fix Design

The minimum correct design is:

1. disconnect fences must have a contract-level persistence interface
2. `RealtimeClusterBridge` must support lazy restore of a fence from that store
3. clearing the fence after fresh `session.resume` must clear both in-memory state and the backing store
4. the recovery path must be testable without coupling the whole service to one storage vendor

## 5. Implementation

- `crates/im-platform-contracts/src/lib.rs`
  - added:
    - `RealtimeDisconnectFenceRecord`
    - `RealtimeDisconnectFenceStore`
- `adapters/local-memory/src/lib.rs`
  - added `MemoryRealtimeDisconnectFenceStore`
- `adapters/local-memory/tests/local_memory_adapter_test.rs`
  - added overwrite/clear coverage for the new fence store
- `services/session-gateway/src/cluster.rs`
  - `RealtimeClusterBridge` now accepts a pluggable disconnect fence store
  - added `RealtimeClusterBridge::with_disconnect_fence_store(...)`
  - disconnect fences are now:
    - written through on disconnect
    - lazily restored on first read after rebuild
    - cleared from both cache and store after fresh resume
  - added bridge rebuild regression coverage
- `services/session-gateway/src/lib.rs`
  - added `build_app_with_cluster(...)` so restart-style tests can rebuild the access plane with a new bridge instance
- `services/session-gateway/tests/http_smoke_test.rs`
  - added restart/rebuild reconnect fence regression
- `services/sdkwork-im-cloud-gateway/tests/http_e2e_test.rs`
  - added local profile restart/rebuild reconnect fence regression

## 6. Verification

### Red

- `cargo test -p session-gateway --offline disconnect_fence_survives_bridge_rebuild_with_shared_store -- --nocapture`
- `cargo test -p session-gateway --offline test_session_gateway_rebuild_preserves_reconnect_required_fence_until_fresh_resume -- --exact --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_rebuild_preserves_reconnect_required_fence_until_fresh_resume -- --exact --nocapture`

Observed red result:

- compilation failed because the persistence seam and restart-test hooks did not exist yet

### Green

- `cargo test -p session-gateway --offline disconnect_fence_survives_bridge_rebuild_with_shared_store -- --nocapture`
- `cargo test -p session-gateway --offline test_session_gateway_rebuild_preserves_reconnect_required_fence_until_fresh_resume -- --exact --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_rebuild_preserves_reconnect_required_fence_until_fresh_resume -- --exact --nocapture`

Observed green result:

- a rebuilt bridge can reload disconnect intent from a shared fence store
- stale session traffic remains blocked after rebuild
- fresh `session.resume` still clears the restored fence correctly

## 7. Design Consequence

The reconnect boundary is no longer tied only to one in-memory bridge instance.

The architecture now has a stable pluggable persistence boundary, which means production deployments can attach a durable implementation without rewriting access-plane logic.

## 8. Important Limitation

The default bridge still uses an in-process memory fence store.

That means:

- the recovery contract is now implemented
- the persistence boundary is now frozen
- but true cross-process durability still depends on wiring a durable store implementation in production

This is still an improvement because the architecture and service paths are no longer hard-coded to volatile-only fence storage.

## 9. Remaining Risks

- the default local-memory adapter is not sufficient for real crash durability
- route/session epoch durability is still unresolved
- error payloads may still reference the original disconnect owner node after recovery, which is acceptable for now but not yet normalized for failover observability

## 10. Next Wave

1. add a real durable disconnect fence store for production/private deployment profiles
2. review whether route/session epochs should share the same recovery backend
3. continue toward durable failover semantics for route ownership and session fencing

