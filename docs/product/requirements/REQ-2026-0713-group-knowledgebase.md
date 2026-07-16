# REQ-2026-0713-group-knowledgebase

Status: ready
Owner: sdkwork-im and sdkwork-knowledgebase maintainers
Source: group knowledge base capability request
Specs: REQUIREMENTS_SPEC.md, API_SPEC.md, SDK_SPEC.md, DATABASE_SPEC.md, IAM_SPEC.md, SECURITY_SPEC.md, PRIVACY_SPEC.md, APP_PC_ARCHITECTURE_SPEC.md, DESKTOP_APP_ARCHITECTURE_SPEC.md, DOCUMENTATION_SPEC.md

## Goal

Give every IM Conversation group one managed SDKWork Knowledgebase space without creating a
space by default when the group is created. Only the current group Owner can initialize the space
exactly once or retry failed provisioning; the initial Owner may explicitly request that same
initialization during group creation. After the binding is active, joined non-Guest Owners,
Admins, and Members can open the complete Knowledgebase product in a browser tab or its
independently running Tauri application.

## Scope

- The group identity is the IM Conversation `conversationId`, scoped by the Auth Token-derived
  tenant and organization dimension. Tenant sessions use `organizationId=0`; organization login is
  not required to create, initialize, or open a group Knowledgebase.
- IM owns group existence, member state, group roles, owner transfer, and user-facing commands.
- Knowledgebase owns the group-space binding, space/document lifecycle, and final content access
  enforcement.
- IM exposes user-facing retrieve, Owner-only ensure/retry, and launch operations through the IM
  generated SDK. The current `Owner` role is authoritative for initialization; an ownership
  transfer changes which actor may perform a later initialization or retry.
- `initializeKnowledgebase` defaults to `false` on group creation. An omitted or `false` value
  must not reserve a binding, call Knowledgebase, or
  create a Knowledgebase record. The initial Owner may explicitly set it to `true`; the server
  durably creates the group before one Owner-authorized provisioning attempt and returns its
  lifecycle state without rolling back an otherwise created group when that remote attempt fails.
- A short-lived, opaque, one-time launch ticket carries no access, refresh, or identity token.
- The Knowledgebase app consumes that ticket only after its normal authenticated session is ready
  and opens the exact authorized space.
- Browser launch opens the standalone Knowledgebase app in a new tab/window with the ticket only
  in the URL fragment. Desktop launch uses the standalone Knowledgebase Tauri process and its
  constrained deep-link protocol.
- IM calls the lifecycle boundary through the generated Knowledgebase RPC SDK with mTLS and a
  signed caller context; raw HTTP and manually assembled credentials are not an integration path.
- Production activation requires a separately deployed, ready Knowledgebase RPC host with durable
  database and Drive storage, an approved network route, and issued mTLS material. This IM
  repository does not define production DNS names, Secret names, certificate paths, or storage
  claims for that sibling service.

## Non-Goals

- No implicit knowledge space creation during group creation; create-time initialization is only
  available through the explicit Owner opt-in.
- No mapping to legacy `im_chat_groups.group_id` or to a group without a Conversation identity.
- No iframe, IM-owned Webview window, arbitrary executable launch, raw HTTP, or caller-controlled
  tenant, organization, actor, role, or space identifier.
- No silent recreation after an explicitly deleted group knowledge space.
- No automatic physical deletion of documents when a group is dissolved; lifecycle defaults to
  archive and retention.

## Acceptance Criteria

1. A group-create request with omitted or `false` `initializeKnowledgebase` creates no
   Knowledgebase binding or space and does not validate Knowledgebase scope, reserve a link, or
   call Knowledgebase. An initial Owner can explicitly send `true` for a valid group scope; the
   group is durable before its one provisioning attempt, and the response reports `active`,
   `provisioning`, or `failed` without rolling back a successfully created group. Only the current
   Owner can retry a failed initialization from any authenticated group scope; Admin and Member requests in `absent` or `failed`
   state are denied without reserving a link or calling Knowledgebase.
2. Concurrent ensure requests for the same `(tenant, organization, conversation)` produce one
   active Knowledgebase space and one authoritative binding; all Owner retries are idempotent.
   Knowledgebase persists that binding as the sole group-to-space authority while IM persists only
   its link, lifecycle, ticket, and event-delivery projections.
3. Initialization authorization and active-content access are distinct. The current Owner alone
   may initialize or retry. Once active, Owner maps to Knowledgebase Owner, Admin to Writer,
   Member to Reader, muted member retains Reader access, and joined non-Guest members may open the
   space. Guest, left, removed, and non-member actors are denied immediately.
4. A group Header shows an accessible knowledge-base icon only for group conversations, and group
   information exposes the Owner-only knowledgebase management entry. The command is disabled
   while provisioning and exposes a recoverable Owner retry after failure. The client obtains the
   authoritative current-member role rather than inferring it from a cached member list; an access
   read failure is fail-closed and permits only a safe re-read, never launch, popup reservation, or
   initialization.
5. A browser click synchronously reserves a new tab before asynchronous work, then navigates only
   to the standalone Knowledgebase group-launch route with the opaque ticket in its fragment. The
   ticket is not placed in query parameters, storage, or logs and is removed from browser history
   immediately after consumption.
6. A desktop click invokes a typed native command that accepts only an opaque launch ticket. The
   command can open only the registered Knowledgebase deep-link protocol; it cannot open arbitrary
   URLs, paths, or executables. The Knowledgebase application focuses an existing group window or
   creates one full product window.
7. Launch ticket validation is atomic and binds tenant, organization, authenticated actor,
   conversation, active space, group role, binding version, expiry, and single-use state. A ticket
   stolen by another authenticated user, replayed, expired, or invalidated by membership change is
   denied without exposing a space.
8. All HTTP operations use the SDKWork v3 success envelope and Problem Details errors; frontend
   and service consumers use generated SDKs, generated RPC SDKs, or approved composed facades
   only. The lifecycle RPC host accepts framework-verified mTLS and signed caller context from the
   IM service identity and rejects request fields as a substitute for that authority.
9. Group membership and lifecycle events are idempotent and version-aware. Owner transfer updates
   effective permissions; group dissolution archives access and retains audit history.
10. PostgreSQL and local development migrations, SDK materialization, security tests, browser and
    Tauri tests, documentation, operational verification, and migration rollback evidence are
    updated together. Production readiness evidence includes the Knowledgebase RPC host database
    and Drive preflight, mTLS issuance, and an approved deployment topology without inventing
    environment-specific endpoints or secret values in source control.

## Quality Attributes

| Area | Requirement |
| --- | --- |
| Security | Auth Token-derived scope isolation, Conversation membership authorization, least privilege, Owner-only initialization, opaque one-time tickets, mTLS + signed caller context, fail-closed authorization, redacted logs, no credential-bearing URL or persistent client storage. |
| Privacy | Only opaque ticket hashes and minimum audit metadata are persisted; no document content or session secrets enter IM records. |
| Reliability | Database uniqueness, idempotency keys, outbox/inbox replay, retryable provisioning, lifecycle reconciliation, and archive rather than destructive defaults. |
| Performance | Header action is non-blocking, provisioning is asynchronous, no list-all client reads, and membership authorization is version-aware with invalidation on events. |
| Operations | Health/readiness includes the required Knowledgebase service dependency and its database/Drive preflight; dashboards surface provisioning failures, ticket consume failures, and synchronization lag without logging tickets. |

## Traceability

- Architecture: [ADR-20260713-group-knowledgebase-binding-and-launch.md](../../architecture/decisions/ADR-20260713-group-knowledgebase-binding-and-launch.md)
- Product canon: [PRD.md](../prd/PRD.md)
- Technical architecture: [TECH_ARCHITECTURE.md](../../architecture/tech/TECH_ARCHITECTURE.md)
- Verification: API contract, SDK materialization, database, Rust, TypeScript, browser, Tauri, and
  cross-service launch tests added with the implementation.
