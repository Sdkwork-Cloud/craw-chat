# ADR-20260617-comms-service-naming-boundaries

Status: accepted  
Owner: sdkwork-im  
Date: 2026-06-17  
Specs: DOMAIN_SPEC.md, NAMING_SPEC.md, API_SPEC.md, ARCHITECTURE_DECISION_SPEC.md, MIGRATION_SPEC.md

## Context

Sdkwork IM uses topology v2 split deploy behind `sdkwork-im-server` (`application.public-ingress`).
Several boundaries drifted from SDKWork standards:

- `contact-service` and `social-service` both mount `/im/v3/api/social/*`.
- `interaction-service` exposes `/im/v3/api/interactions/*` while the OpenAPI authority keeps reactions,
  pins, and threads under `/im/v3/api/chat/*`.
- `docs/api-reference.md` documents forbidden `/api/v1/*` paths.
- Gateway `service_id` values use legacy crate folder names instead of canonical `comms-*` capability ids.
- PC client composition still centralizes module catalogs in `sdkwork-im-pc-chat`.

## Decision

### Domain and naming

- Canonical domain: `communication` (abbreviated `comms` in service ids and future crate names).
- SDK family stem remains `im` (`sdkwork-im-sdk`, `/im/v3/api`).
- Canonical split-deploy service ids:

| Capability | Canonical `service_id` | OpenAPI tag | Path prefix |
| --- | --- | --- | --- |
| Social graph | `comms-social-service` | `social` | `/im/v3/api/social/` |
| Spaces / org | `comms-space-service` | `spaces` | `/im/v3/api/spaces/` |
| Conversation write | `comms-conversation-service` | `chat` | `/im/v3/api/chat/` (write) |
| Conversation read | `projection-service` (rename deferred) | `chat` | `/im/v3/api/chat/` (read) |

Legacy aliases `social-service` and `space-service` remain resolvable in gateway config during migration.

### Service ownership

- **`social-service` is the sole HTTP owner** of `/im/v3/api/social/*` for split deploy and monolith merge.
- **`contact-service` is deprecated** as a public HTTP surface. Postgres CRUD handlers may be merged into
  `social-service` in a follow-up; until then the crate stays library-only.
- **`interaction-service` is deprecated** as a public HTTP surface. Reactions, pins, threads, and conversation
  settings stay on `/im/v3/api/chat/*` per `sdkwork-im-im.openapi.yaml`.
- **`space-service`** remains the owner of `/im/v3/api/spaces/*`; OpenAPI gains a `spaces` tag and seed paths.

### API authority

- OpenAPI (`sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml`) is the HTTP contract source of truth.
- Handwritten `docs/api-reference.md` points to OpenAPI and gateway docs; it must not invent `/api/v1/*` paths.

### PC client

- Module catalog constants move to `sdkwork-im-pc-shell` (`moduleRegistry.ts`).
- `sdkwork-im-pc-chat` component capability is `chat`, not `im-pc-chat`.

## Alternatives

1. Keep `contact-service` on `/im/v3/api/contacts/*` — rejected: diverges from locked OpenAPI `social` tag.
2. Promote `/im/v3/api/interactions/*` — rejected: duplicates existing `chat` tree and breaks SDK parity.
3. Rename all crates immediately to `sdkwork-comms-*-service` — deferred: requires Cargo/workspace migration.

## Consequences

- Gateway stops routing `/im/v3/api/interactions/*`.
- Monolith removes duplicate inline social friend-request routes already served by `social_service::build_app`.
- Operators may configure upstream env vars with either legacy or canonical `service_id` during migration.
- Full OpenAPI coverage for all `space-service` routes is incremental after this ADR.

## Verification

```bash
cargo check -p social-service -p space-service -p contact-service -p sdkwork-im-cloud-gateway -p sdkwork-comms-conversation-service
cargo test -p sdkwork-im-cloud-gateway-config
node sdks/materialize-im-v3-openapi-boundaries.mjs
```

### P4 follow-up (2026-06-17)

- Service directories renamed: `services/sdkwork-comms-conversation-service`, `services/sdkwork-im-cloud-gateway`.
- Gateway route registry and health metadata use canonical `comms-conversation-service` / `sdkwork-im-cloud-gateway` ids.
- `database-table-registry.json` writeOwner updated to `comms-conversation-service`.
- Legacy upstream aliases (`conversation-runtime`, `web-gateway`) remain in gateway config for migration.

### P5 follow-up (2026-06-17)

- Fixed dual-token context resolution when `organization_id` sentinel is `default` (TENANT login scope parity).


- `social-service` and `space-service` gained standalone `main.rs` binaries for split deploy.
- `contact-service` supplemental Postgres routes merge into `social-service` when `SDKWORK_IM_DATABASE_URL` is set.
- OpenAPI `spaces` tag expanded with members/groups seed paths.

## Supersedes / Superseded By

- Supersedes informal `/api/v1` documentation in `docs/api-reference.md`.
- Superseded by: none (directory rename completed 2026-06-17).
