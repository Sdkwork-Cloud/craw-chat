# SDKWork IM PRD

Status: active
Owner: `im-platform`
Application: `chat`
Updated: 2026-07-23
Specs: `REQUIREMENTS_SPEC.md`, `DOCUMENTATION_SPEC.md`, `SECURITY_SPEC.md`, `PRIVACY_SPEC.md`

## 1. Background And Problem

SDKWork IM provides tenant-isolated communication for people, teams, channels, and approved
external participants across PC web/desktop, H5, and Flutter mobile surfaces. The product must offer
one durable communication record, predictable realtime delivery, generated SDK integration, and
operable standalone or cloud deployment without duplicating identity, file, media, knowledge, or
Agent execution authorities owned by sibling SDKWork applications.

The application is pre-launch. `sdkwork.app.config.json` remains the release-state authority and
currently declares a draft publication with disabled deferred artifacts. Documentation must not
present planned package URLs or generated clients as evidence of a commercial release.

## 2. Target Users

- Employees and teams exchanging direct, group, channel, and room messages.
- Organization administrators managing communication membership, policy, retention, and audit.
- External collaborators participating through explicitly authorized federation and shared-channel flows.
- Application developers integrating IM through generated Open, App, or Backend SDK families.
- Operators deploying and observing standalone or cloud IM runtimes.
- Users invoking an assigned Agent from a Conversation while Agent execution remains owned by
  `sdkwork-agents`.

## 3. Goals And Non-Goals

### Goals

- Provide complete Conversation, Message, Member, and ReadCursor workflows with strict tenant and
  organization isolation.
- Preserve one normalized durable authority for current IM state and one visible Message timeline.
- Commit business state, immutable audit/integration evidence, and required outbox delivery atomically.
- Support bounded keyset pagination, per-Conversation ordering, idempotent writes, and realtime recovery.
- Expose contract-first APIs and generated SDKs for all supported client and management surfaces.
- Keep human communication semantics distinct from Agent Session, Turn, Item, and Interaction semantics.
- Operate with fail-closed authentication, authorization, readiness, and dependency behavior.

### Non-Goals

- Owning IAM login, token issuance, tenant, organization, user, or permission catalogs.
- Owning Drive objects, RTC media runtime, Knowledgebase content, or external provider credentials.
- Persisting Agent prompts, models, provider state, Sessions, Turns, Items, tool calls, or transcripts.
- Introducing a second Message timeline, generic business snapshot authority, or startup state rebuild.
- Treating browser, desktop, mobile, Redis, or process-local caches as server business authorities.
- Reimplementing a sibling application API or database simply because it is mounted by the gateway.

## 4. Product Scope

### Communication

- Direct, group, channel, system, handoff, and room Conversation lifecycles.
- Text, media references, structured content, replies, reactions, pins, edit, recall, and visibility.
- Membership, roles, invitations, bans, preferences, read state, unread state, and per-Conversation sequence.
- Presence, typing, realtime subscriptions, catch-up windows, acknowledgements, and offline delivery.
- RTC call signaling and application-data streams; RTC media remains external.
- Contact, friendship, blocking, external collaboration, and shared-channel relationship workflows.

### Administration And Operations

- Tenant-scoped governance and provider policy integration.
- Audit recording, verification, and export.
- Health, readiness, diagnostics, operational lag, runtime inspection, and commercial-readiness reporting.
- Generated SDK families for IM Open API, user-facing App API, and Backend API.

### Composed Capabilities

- Drive-backed attachment and media references.
- Managed group Knowledgebase launch through opaque, short-lived tickets.
- Agent assignment and dispatch through public `sdkwork-agents` contracts; the resulting visible reply is
  a new IM Message and is not an Agent transcript copy.

## 5. User Scenarios

1. A member opens an inbox, pages through Conversations, reads a bounded Message window, and receives
   ordered realtime updates without loading an unbounded history.
2. An authorized member creates or manages a group, changes membership, and observes the same current
   state across API, SDK, realtime, and client surfaces.
3. A sender submits an idempotent Message; the normalized state and outbound delivery evidence commit
   together, and retries cannot create a duplicate Message.
4. A group owner initializes the group's managed Knowledgebase and members launch it through an opaque
   ticket without exposing storage, credentials, or Knowledgebase internals in IM.
5. A user invokes an assigned Agent; IM records the visible request, Agents owns execution, and IM
   publishes a separate visible reply after the external Turn succeeds.
6. An operator diagnoses health and delivery lag using Backend API operations that report real state and
   never fabricate healthy or zero-valued evidence.

## 6. Success Metrics

| Outcome | Acceptance measure |
| --- | --- |
| Isolation | Cross-tenant and cross-organization negative tests pass for every persistent list and mutation |
| Correctness | Message identity, sequence, idempotency, membership, and read-state contract tests pass |
| Performance | Interactive lists use indexed keyset pagination with a maximum 200-item page |
| Reliability | State, journal evidence, and outbox atomicity tests pass under retry and failure injection |
| Security | Dual-token validation, scoped authorization, secret redaction, and fail-closed startup tests pass |
| Contract quality | OpenAPI, route coverage, generated SDK, and response-envelope validators pass |
| Operations | Readiness, migration, drift, lag, and low-cardinality telemetry report authoritative state |
| Release | Signed artifacts, checksums, SBOM, provenance, staging E2E, HA, recovery, and capacity evidence pass |

## 7. Release Phases

| Phase | Exit condition |
| --- | --- |
| Pre-launch architecture closure | Normalized authority, API/SDK parity, dependency boundaries, and Canon checks pass |
| Release candidate | Supported client builds and direct staging end-to-end tests pass against production-like topology |
| Commercial availability | Security, performance, HA/DR, migration, supply-chain, and operational evidence receive sign-off |

No phase is complete merely because source code, a generated SDK, or a deferred artifact declaration exists.

## 8. Linked Requirements And Contracts

- [Normalized IM authority](../requirements/REQ-2026-0722-normalized-im-authority.md)
- [IM-to-Agents dispatch](../requirements/REQ-2026-0719-agents-dispatch.md)
- [Managed group Knowledgebase](../requirements/REQ-2026-0713-group-knowledgebase.md)
- [Current technical architecture](../../architecture/tech/TECH_ARCHITECTURE.md)
- [HTTP API inventory](../../api-reference.md)
- [Database inventory](../../database-design.md)
- `specs/IM_DOMAIN_AND_PERSISTENCE_SPEC.md`
- `specs/IM_AGENTS_DEPENDENCY_AND_DATABASE_SPEC.md`

## 9. Open Questions

Product expansion remains closed until the current commercial gates pass. New communication
capabilities require a requirement and owner-boundary review before they enter navigation, API,
database, SDK, or release scope.
