# ADR-20260619-im-rpc-discovery-integration-deferred

Status: accepted (Phase 1 complete, Phase 2 discovery deferred)  
Owner: sdkwork-im  
Date: 2026-06-19  
Last Updated: 2026-06-29  
Specs: RPC_SPEC.md, RUST_RPC_SPEC.md, RPC_SDK_WORKSPACE_SPEC.md, DEPLOYMENT_SPEC.md, ENVIRONMENT_SPEC.md, DATABASE_SPEC.md, ARCHITECTURE_DECISION_SPEC.md, TEST_SPEC.md

## Context

Sdkwork IM already ships:

- HTTP ingress through `sdkwork-web-framework` (`services/sdkwork-im-cloud-gateway`).
- Persistence through `sdkwork-database` (`crates/sdkwork-im-database-pool`, postgres adapters).
- RPC **contracts** under `apis/rpc/` and generated `sdkwork-im-rpc-sdk`.
- Rust RPC **binding scaffold** in `crates/sdkwork-im-rpc-service-rust` (tonic adapters, manifests, health helpers).
- **Three hosted gRPC service processes** (Phase 1 complete):
  - `services/session-gateway-rpc-bin` (port 50051) — Realtime/Presence RPC
  - `services/sdkwork-comms-conversation-rpc-bin` (port 50052) — Conversation/Message RPC
  - `services/sdkwork-comms-conversation-internal-rpc-bin` (port 50053) — Internal room orchestration RPC

All three RPC hosts use `sdkwork-rpc-framework` (`sdkwork-rpc-server`, `sdkwork-rpc-discovery`, `sdkwork-rpc-client`, `sdkwork-rpc-core`) and support optional discovery registration via `SDKWORK_IM_DISCOVERY_ENDPOINT`. When the env var is unset, registration returns `Ok(None)` and the host runs in standalone mode.

Split-deploy routing still uses static topology env vars (`configs/topology/`) as the primary fallback until Phase 2 discovery ships. Gateway upstream URLs currently cover HTTP only; RPC upstream configuration is deferred until business services consume RPC clients.

The sibling `sdkwork-discovery` product control plane remains available for Phase 2 integration.

## Decision

**Phase 1 (RPC hosts) is complete. Phase 2 (`sdkwork-discovery` product integration) remains deferred until business services consume RPC clients.**

Phase 1 status (complete):

1. RPC proto authority in `apis/rpc/` and generated SDK in `sdks/sdkwork-im-rpc-sdk/`.
2. Three hosted gRPC service processes shipped (`session-gateway-rpc-bin`, `sdkwork-comms-conversation-rpc-bin`, `sdkwork-comms-conversation-internal-rpc-bin`).
3. Optional discovery registration via `SDKWORK_IM_DISCOVERY_ENDPOINT` is supported through `sdkwork-rpc-discovery` framework crate.
4. `sdkwork-discovery` product itself is NOT added to `Cargo.toml` workspace dependencies until Phase 2.

## Phased adoption plan

### Phase 0 — Complete (HTTP-only, contracts ready)

| Item | State |
| --- | --- |
| Proto authority | `apis/rpc/sdkwork/communication/**` |
| RPC SDK | `sdks/sdkwork-im-rpc-sdk/` |
| Rust scaffold | `crates/sdkwork-im-rpc-service-rust` |
| Discovery | Not integrated |

### Phase 1 — Complete (RPC hosts shipped)

Three hosted gRPC service processes shipped:

| Service | Port | gRPC services |
| --- | --- | --- |
| `services/session-gateway-rpc-bin` | 50051 | RealtimeService, PresenceService |
| `services/sdkwork-comms-conversation-rpc-bin` | 50052 | ConversationService, MessageService |
| `services/sdkwork-comms-conversation-internal-rpc-bin` | 50053 | DistributedRuntime, MessageDispatch, RoomOrchestration |

All hosts:
- Use `sdkwork-im-rpc-service-rust` dispatchers via `sdkwork-rpc-server`.
- Support optional discovery registration via `SDKWORK_IM_DISCOVERY_ENDPOINT` (returns `Ok(None)` when unset).
- Topology profiles document bind addresses in `configs/topology/`.

Gate: `cargo test -p sdkwork-im-rpc-service-rust`, `pnpm test:rpc-contract`.

### Phase 2 — Discovery registration

After Phase 1 host runs in split-services topology:

1. Add `sdkwork-discovery` sibling checkout to `sdkwork.workflow.json` (same pattern as `sdkwork-web-framework`).
2. Add workspace dependency on `sdkwork-discovery-rpc-sdk` (or generated Rust client crate) for registration client only.
3. On RPC host bootstrap:
   - Register instance with namespace `communication`, service name = canonical `service_id` (e.g. `comms-conversation-service`).
   - Publish labels: `grpc_url`, `profile_id`, `deployment_mode`, `schema_version`.
   - Renew lease on interval; deregister on graceful shutdown.
4. Use `SDKWORK_DISCOVERY_*` env overlay per sibling README; production requires PostgreSQL storage and signed service tokens.
5. IM database tables remain prefixed `im_`; discovery uses `discovery_` prefix in its own database (`DATABASE_SPEC.md` §33.3).

Gate: new Node contract test `sdkwork-im-discovery-integration.test.mjs` (register → discover → deregister smoke against local discovery dev stack).

### Phase 3 — Consumer-side discovery (optional, after registration stable)

- Gateway or internal callers resolve upstream gRPC URLs via discovery watch instead of static env.
- Keep static env as fallback during migration window (`MIGRATION_SPEC.md`).
- Config registry watch for RPC deadline/TLS policy overrides.

## Service registration map (target)

| Canonical `service_id` | gRPC services (from manifest) | Notes |
| --- | --- | --- |
| `comms-conversation-service` | `ConversationService`, `MessageService` | Primary Phase 1 candidate |
| `session-gateway` | `RealtimeService`, `PresenceService` | Realtime plane |
| `comms-social-service` | `SocialService`, `ContactService` | After social RPC host |
| `comms-space-service` | Space-related RPC when proto stabilizes | Follows HTTP owner |
| `streaming-service` | `StreamService` | Stream plane |
| `notification-service` | `NotificationService` | App surface |
| `governance-service` | `AdminService` (backend v3) | Control plane |

Legacy folder names (`social-service`, `space-service`) may appear as registration aliases until crate rename completes (see ADR-20260617-comms-service-naming-boundaries).

## Alternatives

1. **Integrate discovery now with HTTP-only services** — rejected: no registrable gRPC endpoints; adds moving parts without callers.
2. **Skip discovery; permanent static gRPC URLs** — rejected for cloud split-services; acceptable only for unified-process dev smoke.
3. **Embed registry in IM gateway** — rejected: violates platform boundary; discovery is a sibling product.

## Consequences

- `specs/README.md` records discovery status as **Phase 2 Deferred** (Phase 1 RPC hosts complete via `sdkwork-rpc-framework`).
- `AGENTS.md` RPC and discovery boundary note remains accurate: three `*-rpc-bin` hosts ship; `sdkwork-discovery` product control plane is not yet integrated.
- RPC contract work and RPC host operations can continue without the `sdkwork-discovery` product dependency.
- Phase 2 will introduce a new CI/dev dependency on `sdkwork-discovery` for integration tests once business services consume RPC clients and the discovery product is checked out.

## Verification

Current (Phase 1 complete):

```bash
# RPC contract and host verification
pnpm test:rpc-contract
cargo test -p sdkwork-im-rpc-service-rust
pnpm test:sdkwork-im-session-gateway-rpc-bin
pnpm test:session-gateway-rpc-bin-rust

# Platform framework parity
pnpm test:web-framework-standard
pnpm test:database-framework-standard
```

Future (Phase 2+ after `sdkwork-discovery` checkout):

```bash
# Discovery integration smoke against local discovery dev stack
node scripts/dev/sdkwork-im-discovery-integration.test.mjs
pnpm --dir ../sdkwork-discovery dev
```
