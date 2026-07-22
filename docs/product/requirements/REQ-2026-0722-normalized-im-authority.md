# REQ-2026-0722: Normalize IM Persistence Authority

- Owner: `im-platform`
- Status: in-progress
- Source: architecture governance
- Date: 2026-07-22
- Specs: `REQUIREMENTS_SPEC.md`, `DOMAIN_SPEC.md`, `DATABASE_SPEC.md`,
  `MIGRATION_SPEC.md`, `EVENT_SPEC.md`, `DEPENDENCY_MANAGEMENT_SPEC.md`

## Problem

IM currently mixes canonical communication facts with CQRS projection tables,
generic serialized snapshots, and an event journal described as a second business
authority. This duplicates Conversation, membership, timeline, contact, direct-chat,
and read-cursor state and makes the boundary with `sdkwork-agents` difficult to
reason about.

The application manifest declares `publish.status = DRAFT`; every package is
disabled and marked `releaseBuildDeferred`. `STABLE/0.1.0` package URLs describe a
future release lane and are not evidence of an installed production release.

## Goals

- Make normalized IM tables the only business system of record.
- Use `Conversation -> Message -> Member -> ReadCursor` only for human or channel
  communication. Keep Agents `Session -> Turn -> Item` terminology and ownership
  separate.
- Preserve the dependency direction `sdkwork-im -> sdkwork-agents -> sdkwork-kernel`.
- Store only stable Agents resource identifiers in IM-owned assignment, binding,
  and dispatch records.
- Commit an IM state mutation, its audit event, and its outbound integration event
  in one PostgreSQL transaction.
- Remove duplicate timeline, contact, direct-chat binding, and projector-owned
  business authority before the first release.
- Keep all list paths keyset-paged, tenant/organization scoped, bounded, and backed
  by query-serving indexes.

## Non-Goals

- Moving Agent sessions, turns, items, prompts, model state, or provider state into IM.
- Making Agents depend on IM.
- Copying Agents OpenAPI, generated transports, repositories, or database tables.
- Retaining a compatibility facade for projection terminology in a pre-launch app.
- Treating in-memory state or serialized JSON documents as a production authority.

## Acceptance Criteria

1. Active database contracts contain no `im_projection_*` table.
2. `im_conversations`, `im_conversation_members`,
   `im_conversation_messages`, and `im_conversation_read_cursors` are canonical
   normalized authorities.
3. Timeline reads use `im_conversation_messages`; no second message timeline is
   persisted.
4. Contacts are derived by bounded joins over `im_friendships`, `im_direct_chats`,
   preferences, and IAM SDK data; no contact mirror is persisted.
5. Direct-chat conversation binding is owned only by `im_direct_chats.conversation_id`.
6. IM-to-Agents tables contain stable references only and have no foreign key or
   dependency on `ai_agent_*` persistence.
7. Mutation and outbox tests prove atomic commit and idempotent replay behavior.
8. Canonical specs, PRD, technical architecture, database documentation, and active
   ADR indexes describe the normalized model and contain no claim that projector
   state is a business authority.
9. Architecture checks reject an Agents-to-IM dependency and reject new
   `im_projection_*` DDL or SQL.

## Non-Functional Requirements

- Security: every query and uniqueness constraint carries tenant and organization
  scope; Agents identifiers are opaque and are never trusted as authorization.
- Privacy: IM persists only the visible communication record and minimal stable
  cross-domain references.
- Performance: hot-path lists use indexed keyset pagination with a maximum page
  size of 200; message and membership writes do not require journal replay.
- Reliability: PostgreSQL mutations use transactions, idempotency keys, optimistic
  concurrency where applicable, and transactional outbox delivery.

## Affected Surfaces

- database
- backend services and repositories
- runtime composition
- app/open APIs through unchanged communication semantics
- SDK source contracts where projection terminology is publicly exposed
- operations, recovery, and migration documentation

## Trace

- Decision: `docs/architecture/decisions/ADR-20260722-normalized-im-authority.md`
- Migration: `docs/migrations/MIGRATION-20260722-normalized-im-authority.md`
- Local contract: `specs/IM_DOMAIN_AND_PERSISTENCE_SPEC.md`
- Dependency contract: `specs/IM_AGENTS_DEPENDENCY_AND_DATABASE_SPEC.md`

## Verification

```text
pnpm test:database-naming-standard
pnpm test:contract:database
pnpm test:database-framework-standard
cargo test -p im-adapters-postgres-journal
node ../sdkwork-specs/tools/check-application-layering.mjs --root .
node ../sdkwork-specs/tools/check-component-port-bindings.mjs --root .
```
