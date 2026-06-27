# ADR-20260619-im-rpc-discovery-integration-deferred

Status: accepted (deferred integration)  
Owner: sdkwork-im  
Date: 2026-06-19  
Specs: RPC_SPEC.md, RUST_RPC_SPEC.md, RPC_SDK_WORKSPACE_SPEC.md, DEPLOYMENT_SPEC.md, ENVIRONMENT_SPEC.md, DATABASE_SPEC.md, ARCHITECTURE_DECISION_SPEC.md, TEST_SPEC.md

## Context

Sdkwork IM already ships:

- HTTP ingress through `sdkwork-web-framework` (`services/sdkwork-im-cloud-gateway`).
- Persistence through `sdkwork-database` (`crates/sdkwork-im-database-pool`, postgres adapters).
- RPC **contracts** under `apis/rpc/` and generated `sdkwork-im-rpc-sdk`.
- Rust RPC **binding scaffold** in `crates/sdkwork-im-rpc-service-rust` (tonic adapters, manifests, health helpers).

There is **no hosted gRPC service process** in the IM workspace yet. Split-deploy routing still uses static topology env vars (`configs/topology/`, gateway upstream URLs). The sibling `sdkwork-discovery` product is available for service registration and config watch once RPC hosts exist.

Integrating discovery before runnable RPC servers would add operational complexity without runtime benefit.

## Decision

**Defer `sdkwork-discovery` integration until the first IM RPC service host ships.**

Until then:

1. Keep RPC proto authority in `apis/rpc/` and generated SDK in `sdks/sdkwork-im-rpc-sdk/`.
2. Keep HTTP as the only production transport; gateway env upstreams remain the service location source of truth.
3. Do **not** add `sdkwork-discovery` to `Cargo.toml` workspace dependencies or `sdkwork.workflow.json` release checkout until Phase 1 below starts.
4. Document the phased adoption plan in this ADR and `specs/README.md`.

## Phased adoption plan

### Phase 0 — Current (HTTP-only, contracts ready)

| Item | State |
| --- | --- |
| Proto authority | `apis/rpc/sdkwork/communication/**` |
| RPC SDK | `sdks/sdkwork-im-rpc-sdk/` |
| Rust scaffold | `crates/sdkwork-im-rpc-service-rust` |
| Discovery | Not integrated |
| Verification | `pnpm test:rpc-contract`, `cargo test -p sdkwork-im-rpc-service-rust` |

### Phase 1 — First hosted RPC service (prerequisite for discovery)

Ship one runnable gRPC host process (recommended first candidate: `comms-conversation-service` or `session-gateway` RPC surface mapped from `ConversationService` / `RealtimeService` in `sdkwork-im-rpc.manifest.json`).

Requirements:

- Thin tonic server crate under `services/` or `bin/` using `sdkwork-im-rpc-service-rust` dispatchers.
- Calls existing runtime/service ports; **no** direct SQLx or axum handler logic in RPC adapters (`RUST_RPC_SPEC.md`).
- Health + optional reflection gated by env.
- Topology profile documents bind address and gRPC URL (`SDKWORK_IM_*_GRPC_URL` or service-specific keys per `ENVIRONMENT_SPEC.md`).

Gate: `cargo test -p <rpc-host-crate>` plus contract parity against HTTP operationIds.

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

- `specs/README.md` keeps discovery status **Deferred** until Phase 1 completes.
- `AGENTS.md` platform framework note remains accurate.
- RPC contract work can continue without discovery dependency.
- Phase 2 introduces a new CI/dev dependency on `sdkwork-discovery` for integration tests only after RPC hosts exist.

## Verification

Current (Phase 0):

```bash
pnpm test:rpc-contract
pnpm test:web-framework-standard
pnpm test:database-framework-standard
cargo test -p sdkwork-im-rpc-service-rust
```

Future (Phase 2+):

```bash
# After adding discovery integration test
node scripts/dev/sdkwork-im-discovery-integration.test.mjs
pnpm --dir ../sdkwork-discovery dev
```
