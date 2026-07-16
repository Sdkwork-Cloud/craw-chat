# ADR-20260713-group-knowledgebase-binding-and-launch

Status: accepted
Requirement: REQ-2026-0713
Owner: sdkwork-im and sdkwork-knowledgebase maintainers
Date: 2026-07-13
Specs: ARCHITECTURE_DECISION_SPEC.md, API_SPEC.md, SDK_SPEC.md, DATABASE_SPEC.md, MIGRATION_SPEC.md, IAM_SPEC.md, SECURITY_SPEC.md, PRIVACY_SPEC.md, APP_PC_ARCHITECTURE_SPEC.md, DESKTOP_APP_ARCHITECTURE_SPEC.md

## Context

Groups in the PC IM client are Conversation aggregates. Their stable user-facing identity is
`conversationId`; `im_chat_groups.group_id` is a separate space-group model and cannot prove that
every group chat has a matching row. Existing Knowledgebase context binding permits more than one
space for a chat group, lacks organization scope and provisioning state, and cannot by itself
resolve current IM membership.

The previous host-embed integration can render Knowledgebase inside an IM-owned browser surface or
Webview. It does not meet the product requirement for a full independent Knowledgebase desktop
application and cannot safely grant a specific group space from an arbitrary group identifier.

## Decision

1. The scope key for a group knowledge space is `(tenant_id, organization_id, conversation_id)`.
   `organization_id` is a token-derived isolation dimension and the canonical tenant-session
   sentinel `0` is valid; organization login is not an authorization prerequisite. IM Conversation
   remains authoritative for group roles and membership.
2. Knowledgebase owns one dedicated managed group-space binding aggregate per scope. It is the
   sole authoritative group-to-space relation; generic Knowledgebase context-binding mutations for
   `chat_group` are rejected so two competing mappings cannot exist.
3. Group creation is lazy by default: an omitted or `false` `initializeKnowledgebase` input creates
   no Knowledgebase resource and does not inspect Knowledgebase scope. The initial Conversation
   Owner may explicitly request `true`; IM durably creates the group before one Owner-authorized
   ensure attempt, invokes the generated Knowledgebase RPC SDK over the trusted boundary, and
   reports `active`, `provisioning`, or `failed` state without rolling back the group after a remote
   failure. The current Owner may later issue ensure or retry failed provisioning. Admin and Member
   requests in `absent` or `failed` state are denied before link reservation or a Knowledgebase
   call.
4. IM retains a local saga/projection for remote references, retry state, and UI status. It uses
   outbox/inbox delivery and version-aware membership events; it never owns Knowledgebase content
   tables or cross-database foreign keys.
5. IM emits a short-lived, hash-stored, one-time opaque ticket only after the binding is active
   and the requester is a joined non-Guest member. The Knowledgebase app consumes it using an
   approved trusted IM SDK/delegation boundary and rechecks session actor, exact token-derived scope, active
   membership, role, binding version, and expiry.
6. Browser launch uses a standalone Knowledgebase route with a fragment ticket and removes it from
   browser history once consumed. Desktop launch uses an allowlisted
   `sdkwork-knowledgebase://group-launch/<opaque-ticket>` intent handled by the independent
   Knowledgebase Tauri application. IM never creates a knowledge Webview window for this path.
7. Initialization and active-content access are separate authorizations. The current IM Owner is
   the sole initializer and retry actor. After activation, permission mapping is Owner -> Owner,
   Admin -> Writer, Member -> Reader, muted -> Reader, Guest -> denied. Removal, leaving, role
   reduction, owner transfer, deletion, and dissolution are processed from IM lifecycle events;
   dissolve archives rather than hard-deletes by default.

## Alternatives

1. **Automatically create a Knowledgebase space for every group**: rejected because most groups
   never use a knowledge base and early creation creates orphaned resources. An explicit initial
   Owner opt-in remains a controlled post-persistence provisioning attempt.
2. **Use `im_chat_groups.group_id`**: rejected because it is not the Conversation group identity
   used by the PC client and is optional for group conversations.
3. **Reuse generic `chat_group` context binding as the authority**: rejected because its generic
   cardinality and lifecycle semantics cannot enforce the managed one-space group invariant.
4. **Let the renderer call Knowledgebase directly to create a space**: rejected because it creates
   authorization bypasses, races, and orphaned spaces.
5. **Pass a space ID or session token in a browser URL/deep link**: rejected because it leaks
   authority through logs, history, application activation, or persistent client state.
6. **Open an IM-owned Tauri Webview**: rejected because it is not an independent Knowledgebase
   process and does not provide the complete standalone application lifecycle.

## Consequences

- Both products require coordinated contracts, migrations, generated SDK materialization, and
  service health/dependency configuration.
- Direct generic group-context mutation is no longer a supported public workflow.
- The standalone Knowledgebase desktop package must register and validate its deep-link protocol,
  forward intents to its single-instance process, and focus/create stable per-group windows.
- Failed provisioning is observable and retryable. Explicit deletion is reflected as deleted and
  is never silently recreated by a click.
- IM-to-Knowledgebase lifecycle calls use generated SDK/RPC SDK boundaries only, with framework
  verified mTLS and signed caller context. Raw HTTP, manual auth headers, and local SDK forks are
  not permitted at this boundary.
- Staging and production require a deployed Knowledgebase RPC host, a reachable endpoint, issued
  mTLS material, and durable database and Drive storage that pass host preflight. This repository
  does not invent the sibling service image, namespace, DNS name, certificate, Secret, or volume
  claim; those inputs must be supplied by deployment ownership before rollout.
- Migration rollout follows expand-contract sequencing with rollback documentation; no production
  deployment or publication is implied by this decision.

## Verification

```powershell
# IM repository
node ..\sdkwork-specs\tools\check-api-operation-patterns.mjs --root .
node ..\sdkwork-specs\tools\check-api-response-envelope.mjs --root .
pnpm test:database-framework-standard
pnpm test
pnpm check

# Knowledgebase repository
node ..\sdkwork-specs\tools\check-api-operation-patterns.mjs --root ..\sdkwork-knowledgebase
node ..\sdkwork-specs\tools\check-api-response-envelope.mjs --root ..\sdkwork-knowledgebase
pnpm --dir ..\sdkwork-knowledgebase test
pnpm --dir ..\sdkwork-knowledgebase check
```

Focused tests must cover concurrent initialization, Owner-only initialization and retry, active
non-Guest launch authorization, ticket theft/replay/expiry, membership mutation between issue and
consume, exact-space selection, browser URL redaction, deep-link allowlisting, mTLS/caller-context
rejection, and archived/deleted lifecycle behavior.

## Supersedes / Superseded By

- Supersedes the use of host-embed group identifiers or IM-owned knowledge windows for managed
  group knowledge bases.
- Superseded by: none.
