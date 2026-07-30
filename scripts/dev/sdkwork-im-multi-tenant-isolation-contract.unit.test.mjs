import assert from 'node:assert/strict';
import test from 'node:test';

import {
  findIsolationViolationsInRustSource,
  loadIsolationContract,
  missingCrossOrganizationAuditFields,
} from './sdkwork-im-multi-tenant-isolation-contract.test.mjs';

test('loads all organization-scoped tables from the canonical table registry', () => {
  const contract = loadIsolationContract();
  assert.equal(contract.organizationScopedTables.length, 63);
  assert.ok(contract.organizationScopedTables.includes('im_rtc_signals'));
  assert.ok(contract.organizationScopedTables.includes('im_user_profiles'));
});

test('requires complete cross-organization audit evidence near the declared event', () => {
  const complete = `tracing::info!(
    event = "im.operation.completed",
    actor_kind = "service-worker",
    actor_id = "worker-1",
    trace_id = "trace-1",
    outcome = "succeeded",
  );`;
  assert.deepEqual(missingCrossOrganizationAuditFields(complete, 'im.operation.completed'), []);

  const incomplete = 'tracing::info!(event = "im.operation.completed", outcome = "succeeded");';
  assert.deepEqual(
    missingCrossOrganizationAuditFields(incomplete, 'im.operation.completed'),
    ['actor_kind', 'actor_id', 'trace_id'],
  );
});

test('accepts a multiline organization-bound query', () => {
  const source = `const SQL: &str = r#"
select message_id
from im_conversation_messages
where tenant_id = $1
  and organization_id = $2
"#;`;
  assert.deepEqual(findIsolationViolationsInRustSource(source), []);
});

test('rejects executable SQL without an organization predicate', () => {
  const source = 'const SQL: &str = "delete from im_commit_journal where event_id = $1";';
  const violations = findIsolationViolationsInRustSource(source);
  assert.equal(violations.length, 1);
  assert.equal(violations[0].table, 'im_commit_journal');
});

test('rejects executable SQL without a tenant predicate', () => {
  const source = 'const SQL: &str = "select event_id from im_commit_journal where organization_id = $1";';
  const violations = findIsolationViolationsInRustSource(source);
  assert.equal(violations.length, 1);
  assert.match(violations[0].reason, /tenant_id or organization_id/u);
});

test('ignores SQL fragments used only as assertion text', () => {
  const source = 'assert!(sql.contains("update im_outbox_events"));';
  assert.deepEqual(findIsolationViolationsInRustSource(source), []);
});

test('accepts only the governed retention purge SQL shape', () => {
  const source = `const SQL: &str = r#"
/* sdkwork:cross-organization-operation=retention-expiry-purge */
delete from im_outbox_events where ctid in (
  select ctid from im_outbox_events
  where retention_until is not null and retention_until <= now()
  order by retention_until asc limit $1
)
"#;`;
  assert.deepEqual(findIsolationViolationsInRustSource(
    source,
    'adapters/postgres-journal/src/retention_cleanup.rs',
  ), []);
});

test('rejects unknown cross-organization operation markers', () => {
  const source = `const SQL: &str = r#"
/* sdkwork:cross-organization-operation=skip-check */
select event_id from im_commit_journal where event_id = $1
"#;`;
  const violations = findIsolationViolationsInRustSource(source);
  assert.equal(violations.length, 1);
  assert.match(violations[0].reason, /unknown cross-organization operation/u);
});

test('rejects unknown operation markers even when the query is organization-bound', () => {
  const source = `const SQL: &str = r#"
/* sdkwork:cross-organization-operation=skip-check */
select event_id from im_commit_journal where organization_id = $1
"#;`;
  const violations = findIsolationViolationsInRustSource(source);
  assert.equal(violations.length, 1);
  assert.match(violations[0].reason, /unknown cross-organization operation/u);
});

test('rejects an approved operation marker outside its contracted source file', () => {
  const source = `const SQL: &str = r#"
/* sdkwork:cross-organization-operation=retention-expiry-purge */
delete from im_outbox_events where ctid in (
  select ctid from im_outbox_events
  where retention_until is not null and retention_until <= now()
  order by retention_until asc limit $1
)
"#;`;
  const violations = findIsolationViolationsInRustSource(source, 'services/example/src/lib.rs');
  assert.equal(violations.length, 1);
  assert.match(violations[0].reason, /not allowed/u);
});

test('rejects journal recovery replay from ordinary runtime repositories', () => {
  const source = `const SQL: &str = r#"
/* sdkwork:cross-organization-operation=journal-recovery-replay */
select event_id from im_commit_journal
where partition_key like $1 || '%'
order by partition_key asc, commit_offset asc
limit $2
"#;`;
  const violations = findIsolationViolationsInRustSource(source);
  assert.equal(violations.length, 1);
  assert.match(violations[0].reason, /unknown cross-organization operation/u);
});

test('checks organization isolation inside CTE statements', () => {
  const source = `const SQL: &str = r#"
with candidate as (
  select event_id from im_outbox_events where publish_status = 'pending'
)
select event_id from candidate
"#;`;
  const violations = findIsolationViolationsInRustSource(source);
  assert.equal(violations.length, 1);
  assert.equal(violations[0].table, 'im_outbox_events');
});

test('accepts governed outbox scope discovery', () => {
  const source = `const SQL: &str = r#"
/* sdkwork:cross-organization-operation=outbox-scope-discovery */
select tenant_id, organization_id from im_outbox_events
where publish_status = 'pending' and available_at <= $1 and aggregate_type = $2
group by tenant_id, organization_id
order by min(available_at), tenant_id, organization_id
limit $3
"#;`;
  assert.deepEqual(findIsolationViolationsInRustSource(
    source,
    'adapters/postgres-journal/src/outbox_store.rs',
  ), []);
});
