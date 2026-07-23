# SDKWork IM Technical Architecture

Status: active
Owner: `im-platform`
Updated: 2026-07-23
Specs: `DOMAIN_SPEC.md`, `API_SPEC.md`, `SDK_SPEC.md`, `DATABASE_SPEC.md`, `SECURITY_SPEC.md`,
`APP_COMPOSITION_SPEC.md`, `DEPLOYMENT_SPEC.md`, `DOCUMENTATION_SPEC.md`

## 1. Architecture Overview

SDKWork IM is a contract-first, multi-surface communication application. PostgreSQL normalized
state is the only durable business authority. The immutable commit journal records audit and
integration evidence, while transactional outbox/inbox records deliver events. Current state is
never reconstructed from the journal and query correctness never depends on a second persisted model.

```text
PC / H5 / Flutter / integrations
  -> generated IM Open, App, or Backend SDK
  -> standalone gateway or cloud application ingress
  -> sdkwork-web-framework authentication and request context
  -> domain route -> service -> repository
  -> normalized PostgreSQL state + journal evidence + outbox (one transaction)
  -> bounded realtime and integration relays
```

Core principles:

- One domain fact has one write owner and one durable authority.
- IM vocabulary is `Conversation -> Message -> Member -> ReadCursor`.
- All public HTTP behavior is authored in OpenAPI before SDK materialization.
- Lists are tenant-scoped, bounded, and keyset-paged in the data store.
- Cross-domain integrations use public generated SDKs, RPC SDKs, or approved facades.
- Process memory, Redis, browser storage, and device caches are disposable accelerators or bounded
  offline delivery aids, never server business authorities.

## 2. System Boundaries

| Capability | Owner | SDKWork IM responsibility |
| --- | --- | --- |
| Human and channel communication | `sdkwork-im` | Conversation lifecycle, membership, visible Messages, read state, social relationships, delivery, and signaling |
| Identity and access | `sdkwork-iam` / `sdkwork-appbase` | Consume verified identity and permission context; never issue login Sessions or tokens |
| Agent execution | `sdkwork-agents` | Store assignment/dispatch correlation and visible IM Messages only |
| Agent runtime execution | `sdkwork-kernel` through Agents | No direct IM dependency or execution state |
| Files and media objects | `sdkwork-drive` | Store typed Drive references and communication attribution |
| RTC media runtime | `sdkwork-rtc` | Own call signaling and issue scoped handoff credentials |
| Group knowledge content | `sdkwork-knowledgebase` | Own membership authorization and opaque launch-ticket issuance only |

The enforced Agent dependency direction is:

```text
sdkwork-im -> sdkwork-agents -> sdkwork-kernel
```

Agents and Kernel do not import, call, query, or link IM. IM never creates or writes an
`ai_agent_*` table.

## 3. Runtime Modules

| Module | Responsibility |
| --- | --- |
| `sdkwork-api-im-assembly` | Host-neutral composition of IM-owned HTTP routes and approved dependency adapters |
| `sdkwork-api-im-standalone-gateway` | Standalone listener, ingress protection, OpenAPI aggregation, and application routing |
| `session-gateway` | WebSocket lifecycle, authenticated presence, subscriptions, routing, catch-up, and acknowledgements |
| `sdkwork-comms-conversation-service` | Conversation commands and normalized queries for Messages, Members, read state, and personalization |
| `social-service` | Friend, contact, block, external collaboration, and direct-chat relationship state |
| `space-service` | Space, group, channel, membership, invitation, ban, and access-rule state |
| `im-calls-service` | RTC call signaling and scoped credential handoff to `sdkwork-rtc` |
| `streaming-service` | Ordered application-data streams with PostgreSQL compare-and-set state |
| `audit-service` | Immutable compliance evidence and verification/export operations |
| `notification-service` | Durable notification request state; provider delivery requires an authoritative provider worker |
| `automation-service` | Bounded application workflow adapter; Agent execution delegates to `sdkwork-agents` |
| `ops-service` | Health, cluster, operational lag, runtime inspection, diagnostics, and readiness evidence |

Service crates own application behavior. Route crates adapt HTTP. Repositories own bounded database
access. Gateway hosts own listeners and infrastructure, not domain rules.

## 4. Authentication And Request Context

Protected public operations require the SDKWork dual-token model:

```text
Authorization: Bearer <auth-token>
Access-Token: <access-token>
```

`sdkwork-web-framework` validates credentials and resolves a typed `AppContext` containing tenant,
organization, principal, Session, application, device, data scope, and permission scope. Public
clients do not supply identity context headers. Private trusted-edge context is accepted only after
credential validation, signature verification, and removal of client-supplied identity values.

Every repository predicate and uniqueness boundary includes tenant and organization where the data
is scoped. Resource membership and role checks run before Conversation-bound reads and writes.
Agents, Drive, RTC, and Knowledgebase identifiers are opaque references and are not authorization evidence.

## 5. API And SDK Ownership

| Surface | Prefix | Authored authority | Generated SDK family |
| --- | --- | --- | --- |
| Open API | `/im/v3/api` | `apis/open-api/im/sdkwork-im-im.openapi.yaml` | `sdkwork-im-sdk` |
| App API | `/app/v3/api` | `apis/app-api/communication/sdkwork-im-app-api.openapi.yaml` | `sdkwork-im-app-sdk` |
| Backend API | `/backend/v3/api` | `apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml` | `sdkwork-im-backend-sdk` |

The [generated HTTP API inventory](../../api-reference.md) is the complete method, path, and
`operationId` catalog. It excludes sibling APIs mounted by a gateway. Authored `apis/` documents are
the review authority; `sdks/*/openapi/` documents and language transports are generated materializations.

Consumers use:

```text
UI -> application service/port -> injected generated SDK client
```

Raw HTTP, manual auth headers, generated transport edits, local SDK forks, and duplicated DTO
authorities are forbidden. Success responses use `SdkWorkApiResponse`; failures use RFC 9457
`ProblemDetail` with numeric `code`, `traceId`, route template, canonical `operationId`, and i18n key.

## 6. Data Architecture

### 6.1 Normalized authority

The canonical communication chain is:

```text
Conversation
  -> Member
  -> Message (strict message_seq per Conversation)
  -> ReadCursor
```

Typed tables own preferences, visibility, reactions, pins, social relationships, routing,
idempotency, signaling, streams, and delivery state. There is no second Message timeline, generic
JSON business-state snapshot, compatibility view, dual write, or separate persisted query authority.

The [generated database inventory](../../database-design.md) lists every IM-owned table from
`database/contract/table-registry.json`. Field, constraint, index, retention, and migration details
remain authoritative in registry-linked DDL and migration sources.

### 6.2 Mutation transaction

```text
authorize command
  -> lock/read scoped current row
  -> validate idempotency, lifecycle, and version
  -> mutate normalized state
  -> append immutable journal/audit evidence
  -> enqueue required outbox event
  -> commit one PostgreSQL transaction
  -> relay outbox in bounded claimed batches
```

Relay failure cannot roll back committed business state. Outbox claims use bounded batches and
database fencing. Startup reads current normalized state directly and may repopulate disposable caches lazily.

### 6.3 Query and index contract

- Message history orders by `message_seq`, not timestamp.
- Public and interactive pages contain at most 200 items; repositories may fetch one extra row to
  determine continuation.
- Cursor order and tie-break fields are backed by tenant-leading indexes.
- Membership and read-state authorization are bounded indexed lookups.
- Contact results use bounded joins over canonical social state rather than persisted contact copies.
- No repository loads an unbounded result and slices it in a process or client.

### 6.4 Cross-domain data

Cross-domain resources are stored as stable opaque IDs without physical foreign keys to sibling
databases. IM stores no Agent transcript, Drive object metadata authority, RTC provider state, or
Knowledgebase content. Each owner enforces its own lifecycle and deletion policy.

## 7. Realtime And Delivery

The Session Gateway authenticates a WebSocket `auth.init` frame, binds device and Session identity,
and authorizes user or Conversation scopes. Query-string credentials are rejected in production.

Realtime delivery uses ordered sequence windows, bounded catch-up pages, acknowledgements, and
session-scoped disconnect fencing. Redis supports cluster routing, ephemeral presence, and rate
coordination; PostgreSQL remains durable. Redis loss may reduce availability but cannot redefine
Conversation or Message truth.

Client offline stores are principal-scoped and bounded. They may retain a delivery queue or recent
display window but never replace PostgreSQL or merge data across authenticated principals.

## 8. Reliability And Performance

- Idempotent commands bind a stable key to a request hash and reject conflicting reuse.
- Critical lifecycle transitions use unique constraints, row locks, compare-and-set versions, or leases.
- Background workers use fixed batch sizes, lease expiry, bounded retries, dead-letter evidence, and
  low-cardinality telemetry.
- Readiness fails when a required database, migration, schema state, credential validator, or mandatory
  dependency is unavailable.
- Gateway protection applies edge IP and post-auth tenant limits, bounded circuit breakers, request-size
  limits, trusted-proxy validation, and cached single-flight OpenAPI aggregation.
- Million-row operational work must be resumable and database-paged; all-row in-process traversal is
  not a commercially supported path.

## 9. Security, Privacy, And Observability

- Dual-token validation, token identifier reuse prevention, key rotation, and fail-closed production secrets.
- Tenant, organization, principal-kind, principal-id, membership, role, and permission checks at every boundary.
- Parameterized SQL and typed request decoding; no concatenated user input.
- Prompt, credential, token, private Message, provider failure, and sensitive object data redaction.
- Restricted Kubernetes security context, default-deny network policy, immutable image references, and
  signed release artifacts with checksum, SBOM, and provenance.
- Structured logs, trace propagation, bounded diagnostic windows, readiness, and metrics without tenant,
  principal, Conversation, Message, Session, or provider IDs in metric labels.

## 10. Deployment And Runtime Topology

`sdkwork.app.config.json` declares application identity and release state. `etc/` declares concrete
environment, topology, public origin, bind, and upstream configuration.

- `standalone`: one application ingress assembles IM APIs and approved embedded dependency routes.
- `cloud`: application ingress routes to independently deployed service and dependency authorities.
- Both profiles expose identical IM-owned paths, methods, operation IDs, schemas, auth, and errors.
- PostgreSQL is required for durable production IM state. Redis is required where the selected HA
  topology declares clustered routing or distributed coordination.

The current app publication state is draft. Production claims require direct staging, capacity,
security, HA, recovery, migration, and supply-chain evidence for the exact release artifacts.

## 11. Architecture Decisions

- [Normalized IM authority](../decisions/ADR-20260722-normalized-im-authority.md)
- [IM-to-Agents integration](../decisions/ADR-20260719-im-agents-dispatch.md)
- [Group Knowledgebase authentication boundary](../decisions/ADR-20260716-group-knowledgebase-authentication-boundary.md)
- [Auth context capability composition](../decisions/ADR-20260715-auth-context-capability-composition.md)
- [Communication service naming](../decisions/ADR-20260617-comms-service-naming-boundaries.md)
- [RPC discovery deferral](../decisions/ADR-20260619-im-rpc-discovery-integration-deferred.md)

Historical design files do not override this Canon. A new boundary change requires a new requirement
and ADR with explicit supersession.

## 12. Verification

```bash
node docs/sites/scripts/generate-contract-inventories.mjs --check
pnpm test:normalized-im-authority-standard
pnpm test:database-naming-standard
pnpm test:database-framework-standard
pnpm test:apis-authority-standard
pnpm test:sdkwork-im-open-api-route-coverage
pnpm test:web-framework-standard
pnpm test:iam-auth-integration
pnpm test:rtc-signaling-boundary
pnpm test:rpc-contract
pnpm --dir docs/sites run docs:verify
```
