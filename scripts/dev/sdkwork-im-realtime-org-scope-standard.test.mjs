import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const contractControl = read('crates/sdkwork-im-contract-control/src/lib.rs');
const baselineDdl = read(
  'database/ddl/baseline/postgres/0001_im_legacy_baseline.sql',
);

assert.match(
  contractControl,
  /pub organization_id: String/,
  'contract-control realtime records must carry organization_id',
);
assert.match(
  contractControl,
  /fn load_checkpoint\([\s\S]*organization_id: &str/,
  'RealtimeCheckpointStore::load_checkpoint must accept organization_id',
);
assert.match(
  contractControl,
  /pub fn realtime_client_route_scope_key/,
  'shared realtime scope key helper must exist in contract-control',
);
assert.match(
  read('services/session-gateway/src/principal_scope_tests.rs'),
  /test_principal_scope_keys_isolate_organizations/,
  'principal_scope must test org isolation',
);
assert.match(
  read('services/session-gateway/src/realtime.rs'),
  /struct RealtimePrincipalScopeKey \{[\s\S]*organization_id: String/,
  'realtime subscription fanout index must include organization_id',
);
assert.match(
  baselineDdl,
  /pk_im_realtime_checkpoints PRIMARY KEY \(tenant_id, organization_id, client_route_scope_key\)/,
  'baseline DDL must scope realtime checkpoints by organization_id',
);

assert.match(
  baselineDdl,
  /pk_im_presence_states PRIMARY KEY \(tenant_id, organization_id, principal_kind, principal_id, device_id\)/,
  'baseline DDL must scope presence states by organization_id',
);
assert.match(
  contractControl,
  /pub struct PresenceStateRecord \{[\s\S]*organization_id: String/,
  'PresenceStateRecord must carry organization_id',
);
assert.match(
  contractControl,
  /fn load_state\([\s\S]*organization_id: &str/,
  'PresenceStateStore::load_state must accept organization_id',
);
assert.match(
  read('adapters/postgres-realtime/src/lib.rs'),
  /on conflict \(tenant_id, organization_id, principal_kind, principal_id, device_id\) do update/,
  'postgres presence upsert must use migration 010 conflict key',
);
assert.match(
  read('services/session-gateway/src/presence.rs'),
  /record\.organization_id\.as_str\(\)/,
  'presence runtime must thread organization_id through durable store calls',
);
assert.doesNotMatch(
  read('services/session-gateway/src/presence.rs'),
  /principal_scope_key\([^\)]*"default"/,
  'presence runtime must not hardcode default organization_id in scope keys',
);
assert.match(
  read('services/session-gateway/src/presence_tests.rs'),
  /test_presence_state_store_isolates_organizations_for_same_principal_and_device/,
  'presence runtime must test organization isolation',
);
assert.match(
  contractControl,
  /pub fn realtime_principal_scope_key[\s\S]*principal_kind,\s*\n\s*principal_id,/,
  'realtime principal scope key must use principal_kind before principal_id',
);
assert.match(
  baselineDdl,
  /pk_im_route_bindings PRIMARY KEY \(tenant_id, organization_id, principal_kind, principal_id, device_id\)/,
  'baseline DDL must scope route bindings by organization_id',
);
assert.match(
  read('adapters/postgres-realtime/src/route_store.rs'),
  /on conflict \(tenant_id, organization_id, principal_kind, principal_id, device_id\)/,
  'postgres route binding upsert must use migration 010 conflict key',
);
assert.match(
  read('adapters/postgres-realtime/src/route_store.rs'),
  /route_bound_at_timestamp/,
  'postgres route binding must bind bound_at as chrono timestamps',
);
assert.match(
  read('adapters/postgres-realtime/src/route_store.rs'),
  /session_id\.as_deref\(\)/,
  'postgres route binding must bind nullable session_id as Option<&str>',
);
assert.doesNotMatch(
  read('adapters/local-memory/src/lib.rs').split('impl PresenceStateStore')[1]?.split('impl MemoryTimelineProjectionStore')[0] ?? '',
  /"default"/,
  'memory presence store must not hardcode default organization_id in production paths',
);

const projectionScope = read('services/projection-service/src/scope.rs');
const projectionMigration = baselineDdl;
assert.match(
  projectionScope,
  /pub\(super\) organization_id: String,/,
  'projection client route scope keys must include organization_id',
);
assert.match(
  projectionScope,
  /organization_id: &str,\s*\n\s*principal_kind: &str,\s*\n\s*principal_id: &str,/,
  'projection client route scope keys must order principal_kind before principal_id',
);
assert.match(
  read('services/projection-service/src/client_route_sync.rs'),
  /test_registered_client_routes_isolate_organizations/,
  'projection client route runtime must test organization isolation',
);
assert.match(
  projectionMigration,
  /pk_im_projection_registered_client_routes PRIMARY KEY \(tenant_id, organization_id, principal_kind, principal_id, device_id\)/,
  'migration 011 must scope projection registered client routes by organization_id',
);

assert.match(
  read('crates/im-domain-events/src/lib.rs'),
  /pub organization_id: String/,
  'CommitEnvelope must carry organization_id for tenant/org scoped fanout',
);
assert.match(
  read('services/projection-service/src/scope.rs'),
  /projection_organization_id_for_event/,
  'projection fanout must resolve organization_id from commit envelopes',
);

const conversationRuntime = read(
  'services/sdkwork-comms-conversation-service/src/runtime.rs',
);
assert.match(
  conversationRuntime,
  /pub organization_id: String/,
  'conversation commands must carry organization_id',
);
assert.match(
  conversationRuntime,
  /organization_id_from_auth_context/,
  'conversation commands must resolve organization_id from AppContext auth',
);
assert.match(
  read('services/sdkwork-comms-conversation-service/src/runtime/support.rs'),
  /organization_id: normalize_commit_organization_id/,
  'conversation envelope builders must normalize organization_id',
);
assert.match(
  read('services/sdkwork-comms-conversation-service/src/runtime/handoff.rs'),
  /command\.organization_id\.as_str\(\)/,
  'agent handoff transitions must thread command organization_id',
);
assert.match(
  read('services/sdkwork-comms-conversation-service/src/runtime/support.rs'),
  /conversation_scope_key\([\s\S]*organization_id: &str,\s*\n\s*conversation_id: &str,/,
  'conversation runtime scope keys must include organization_id',
);
assert.match(
  read('services/projection-service/src/scope.rs'),
  /pub\(super\) fn scope_key\([\s\S]*organization_id: &str,\s*\n\s*conversation_id: &str,/,
  'projection conversation scope keys must include organization_id',
);
assert.match(
  read('services/projection-service/src/scope.rs'),
  /test_conversation_scope_keys_isolate_organizations/,
  'projection scope must test conversation organization isolation',
);
assert.doesNotMatch(
  read('services/sdkwork-comms-conversation-service/src/runtime/creation.rs'),
  /organization_id: "default"\.into\(\)/,
  'conversation creation envelopes must not hardcode default organization_id',
);

assert.match(
  read('services/projection-service/src/scope.rs'),
  /test_contact_owner_scope_keys_isolate_organizations/,
  'projection scope must test contact owner organization isolation',
);
assert.match(
  read('services/projection-service/src/contacts.rs'),
  /organization_id: &str,\s*\n\s*owner_user_id: &str,/,
  'projection contacts API must scope reads by organization_id',
);
assert.match(
  read('services/projection-service/src/access.rs'),
  /direct_chat_binding_for_conversation\([\s\S]*organization_id: &str,/,
  'projection direct-chat binding lookup must include organization_id',
);
assert.match(
  projectionMigration,
  /pk_im_projection_contacts PRIMARY KEY \(tenant_id, organization_id, owner_user_id, contact_type, target_user_id\)/,
  'migration 011 must scope projection contacts by organization_id',
);
assert.match(
  projectionMigration,
  /pk_im_projection_direct_chat_bindings PRIMARY KEY \(tenant_id, organization_id, direct_chat_id\)/,
  'migration 011 must scope projection direct-chat bindings by organization_id',
);

assert.doesNotMatch(
  baselineDdl,
  /^CREATE INDEX idx_/m,
  'baseline DDL indexes must be idempotent (CREATE INDEX IF NOT EXISTS)',
);
assert.doesNotMatch(
  baselineDdl,
  /^CREATE UNIQUE INDEX uk_/m,
  'baseline DDL unique indexes must be idempotent (CREATE UNIQUE INDEX IF NOT EXISTS)',
);

console.log('sdkwork-im-realtime-org-scope-standard: ok');
