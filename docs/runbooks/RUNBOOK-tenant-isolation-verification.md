# RUNBOOK: Tenant Isolation Verification

Status: active
Owner: `im-platform` and SDKWork security
Updated: 2026-07-24
Specs: `SECURITY_SPEC.md`, `DATABASE_SPEC.md`, `IAM_SPEC.md`, `DOCUMENTATION_SPEC.md`

## Trigger

- Scheduled tenant-isolation audit.
- A schema, repository predicate, authorization policy, or cross-organization workflow changes.
- A suspected cross-tenant disclosure or mutation occurs.

## Prerequisites

- Read-only access to the selected PostgreSQL environment through the approved secret manager.
- Two isolated test tenants and organizations with independently issued valid dual-token sessions.
- Generated IM SDK clients configured through the normal application auth runtime.
- Security-owner approval before any mutation-based production test.

PostgreSQL normalized tables are the durable authority. Tenant isolation is enforced by scoped keys,
repository predicates, typed request context, membership/role checks, and object authorization; IM
does not rely on a separate schema or alternate read model per tenant.

## Procedure

### 1. Verify machine contracts and repository predicates

```bash
pnpm db:contract:check
pnpm db:validate
node scripts/dev/sdkwork-im-multi-tenant-isolation-contract.test.mjs
pnpm test:iam-auth-integration
```

The checks must prove that scoped tables and SQL access include both `tenant_id` and
`organization_id`. Review any new exception with the database and security owners; do not waive the
predicate because an HTTP route currently performs an earlier check.

### 2. Inspect bounded query plans

Use known test-scope identifiers and read-only plans against normalized authority tables:

```bash
psql "$SDKWORK_DATABASE_URL" -v ON_ERROR_STOP=1 -c "
  EXPLAIN (ANALYZE, BUFFERS)
  SELECT message_id, message_seq
  FROM im_conversation_messages
  WHERE tenant_id = '<test-tenant-id>'
    AND organization_id = '<test-organization-id>'
    AND conversation_id = '<test-conversation-id>'
  ORDER BY message_seq DESC
  LIMIT 100;
"
```

Confirm the selected index begins with tenant and organization scope and the query remains bounded.
Repeat the scoped inspection for `im_conversations`, `im_conversation_members`, and
`im_conversation_read_cursors` when those repositories changed.

### 3. Verify object isolation through generated SDKs

1. Using tenant A's generated SDK client, create or select a test Conversation and Message.
2. Using tenant B's independently authenticated SDK client, attempt retrieve, list, update, delete,
   membership, read-cursor, realtime-subscription, and attachment-reference operations for tenant
   A's opaque identifiers.
3. Require a non-disclosing `403` or `404` according to the authored API contract. No response may
   reveal tenant A's existence, metadata, member list, Message content, count, cursor, or timing
   detail beyond the standard error contract.
4. Verify tenant A's state and audit evidence are unchanged after every tenant B attempt.

Do not assemble raw credential headers, tenant headers, or organization headers for this test. The
generated SDK and appbase auth runtime must supply verified session credentials.

### 4. Verify cross-organization and shared-channel policy

- Repeat the negative matrix for two organizations in the same tenant.
- Confirm tenant-root scope uses canonical organization `0` only where the contract permits it.
- Inspect `im_external_connections` and `im_shared_channel_policies` through bounded scoped queries.
- Prove shared history is visible only when the current connection, channel policy, membership, and
  lifecycle state all authorize it; suspension or membership removal must revoke subsequent access.
- Confirm realtime delivery and catch-up enforce the same scope policy as HTTP reads.

### 5. Record evidence

Attach redacted command results, query plans, SDK test identities, trace IDs, negative response codes,
and unchanged row/checksum evidence to a new review record. Do not attach tokens, connection URLs,
Message bodies, or raw tenant data.

## Rollback And Incident Handling

This audit is read-only unless a separately approved test plan says otherwise. On any isolation
failure, remove the affected instances from traffic, preserve audit evidence, revoke exposed test
credentials, and escalate under the tenant-isolation incident process. Do not hide the failure with
a cache purge, compatibility table, or UI restriction.

## Escalation

- Primary: SDKWork security incident owner
- Database: SDKWork database operations
- Application: `im-platform` on-call
