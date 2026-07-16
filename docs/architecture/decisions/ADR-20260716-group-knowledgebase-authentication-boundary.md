# ADR-20260716-group-knowledgebase-authentication-boundary

Status: accepted
Owner: sdkwork-im and sdkwork-knowledgebase maintainers
Date: 2026-07-16
Requirement: REQ-2026-0713
Specs: `IAM_LOGIN_INTEGRATION_SPEC.md`, `SECURITY_SPEC.md`, `DATABASE_SPEC.md`, `MIGRATION_SPEC.md`

## Context

The initial implementation classified managed group Knowledgebase as an organization-only
capability. It rejected a valid tenant-scoped IM session (`organization_id=0`) before checking the
Conversation or its membership. That classification contradicts the product rule that every IM
Conversation group can own one managed Knowledgebase and that the current group Owner/member model
is the authorization authority.

## Decision

1. Every framework-authenticated IM group can use the managed Knowledgebase lifecycle. A separate
   `ORGANIZATION` login mode is not required.
2. The HTTP boundary derives the principal from the verified Auth Token/access-token session and
   accepts both tenant and organization login scopes. Business input cannot select or override the
   tenant, organization, actor, session, or role.
3. The storage/RPC scope remains `(tenant_id, organization_id, conversation_id)` for isolation and
   idempotency. `organization_id=0` is the canonical tenant-session sentinel; positive canonical
   signed-64-bit values represent organization scopes.
4. Initialization remains Owner-only. Active launch remains limited to joined non-Guest Owner,
   Admin, and Member roles. Knowledgebase continues to enforce the synchronized membership snapshot
   and exact token-derived scope.
5. Launch tickets remain one-time, short-lived, session-bound, actor-bound, and opaque. Allowing the
   tenant sentinel does not relax ticket replay, cross-scope, or cross-member protections.

## Consequences

- Tenant-scoped groups can retrieve, initialize, synchronize, launch, and archive their managed
  Knowledgebase without selecting an organization.
- RPC, repository, access-authorizer, and database validation accept organization scope `0` while
  continuing to reject malformed, negative, or signed-BIGINT-overflow values.
- Client code checks authentication and server-derived group membership/lifecycle only. It does not
  introduce an organization-session preflight.

## Verification

Focused verification covers tenant- and organization-scoped HTTP contexts, Owner/member role
authorization, tenant-scoped RPC caller contexts, repository persistence with `organization_id=0`,
cross-scope denial, ticket replay/session isolation, PC contract tests, and database manifest checks.

## Migration and rollback

Knowledgebase greenfield baselines and the pre-GA legacy schema inputs adopt the nonnegative
organization scope directly. Already initialized PostgreSQL and SQLite environments apply the
forward migration `202607160001_group_knowledgebase_tenant_scope`; it preserves all rows and keys
while changing only the organization-scope constraint. There is deliberately no destructive down
migration: once tenant-scoped bindings exist, restoring `organization_id > 0` would reject valid
data. Operational rollback disables the group Knowledgebase entrypoint and rolls application code
forward to a corrected implementation while retaining bindings for recovery.

## Supersedes / Superseded By

- Supersedes the group Knowledgebase-specific conclusions in
  `ADR-20260715-auth-context-capability-composition.md`.
- Superseded by: none.
