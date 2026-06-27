import fs from 'node:fs';
import path from 'node:path';

const repoRoot = path.resolve(import.meta.dirname, '../..');

function patch(relativePath, pairs) {
  const filePath = path.join(repoRoot, relativePath);
  let content = fs.readFileSync(filePath, 'utf8');
  for (const [from, to] of pairs) {
    if (!content.includes(from)) {
      console.warn(`WARN missing in ${relativePath}: ${from.slice(0, 50)}...`);
      continue;
    }
    content = content.split(from).join(to);
  }
  fs.writeFileSync(filePath, content);
  console.log(`patched ${relativePath}`);
}

const orgBlock =
  '        tenant_id: &str,\n        organization_id: &str,\n        principal_kind: &str,\n        principal_id: &str,\n        device_id: &str,';
const oldBlock =
  '        tenant_id: &str,\n        principal_kind: &str,\n        principal_id: &str,\n        device_id: &str,';

// local-disk shared
patch('adapters/local-disk/src/shared.rs', [
  [
    `pub(super) fn scope_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    scope_key_parts(&[tenant_id, principal_kind, principal_id, device_id])
}`,
    `pub(super) fn scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    im_platform_contracts::realtime_client_route_scope_key(
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        device_id,
    )
}`,
  ],
]);

// local-disk realtime + state (presence uses default org)
for (const rel of [
  'adapters/local-disk/src/realtime.rs',
  'adapters/local-disk/src/state.rs',
]) {
  patch(rel, [
    [oldBlock, orgBlock],
    [
      'scope_key(tenant_id, principal_kind, principal_id, device_id)',
      'scope_key(tenant_id, organization_id, principal_kind, principal_id, device_id)',
    ],
    [
      `record.tenant_id.as_str(),
                        record.principal_kind.as_str(),
                        record.principal_id.as_str(),
                        record.device_id.as_str(),`,
      `record.tenant_id.as_str(),
                        record.organization_id.as_str(),
                        record.principal_kind.as_str(),
                        record.principal_id.as_str(),
                        record.device_id.as_str(),`,
    ],
    [
      `record.tenant_id.as_str(),
                record.principal_kind.as_str(),
                record.principal_id.as_str(),
                record.device_id.as_str(),`,
      `record.tenant_id.as_str(),
                record.organization_id.as_str(),
                record.principal_kind.as_str(),
                record.principal_id.as_str(),
                record.device_id.as_str(),`,
    ],
    [
      `expected.tenant_id.as_str(),
            expected.principal_kind.as_str(),
            expected.principal_id.as_str(),
            expected.device_id.as_str(),`,
      `expected.tenant_id.as_str(),
            expected.organization_id.as_str(),
            expected.principal_kind.as_str(),
            expected.principal_id.as_str(),
            expected.device_id.as_str(),`,
    ],
  ]);
}

patch('adapters/local-disk/src/state.rs', [
  [
    'scope_key(tenant_id, organization_id, principal_kind, principal_id, device_id)',
    'scope_key(tenant_id, "default", principal_kind, principal_id, device_id)',
  ],
  [
    `record.tenant_id.as_str(),
                record.organization_id.as_str(),`,
    `record.tenant_id.as_str(),
                "default",`,
  ],
]);

// redis event store
patch('adapters/redis-cache/src/realtime_event_store.rs', [
  [
    `fn window_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    format!("realtime:window:{tenant_id}:{principal_kind}:{principal_id}:{device_id}")
}`,
    `fn window_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    format!(
        "realtime:window:{tenant_id}:{organization_id}:{principal_kind}:{principal_id}:{device_id}"
    )
}`,
  ],
  [oldBlock, orgBlock],
  [
    'window_key(tenant_id, principal_kind, principal_id, device_id)',
    'window_key(tenant_id, organization_id, principal_kind, principal_id, device_id)',
  ],
  [
    `record.tenant_id.as_str(),
                record.principal_kind.as_str(),
                record.principal_id.as_str(),
                record.device_id.as_str(),`,
    `record.tenant_id.as_str(),
                record.organization_id.as_str(),
                record.principal_kind.as_str(),
                record.principal_id.as_str(),
                record.device_id.as_str(),`,
  ],
]);

// redis checkpoint parse + tests
patch('adapters/redis-cache/src/realtime_checkpoint_store.rs', [
  [
    `fn parse_checkpoint_fields(
    fields: &[String],
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> Result<RealtimeCheckpointRecord, ContractError> {`,
    `fn parse_checkpoint_fields(
    fields: &[String],
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> Result<RealtimeCheckpointRecord, ContractError> {`,
  ],
  [
    `        let record =
            parse_checkpoint_fields(&fields, tenant_id, principal_kind, principal_id, device_id)?;`,
    `        let record = parse_checkpoint_fields(
            &fields,
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        )?;`,
  ],
  [
    `    Ok(RealtimeCheckpointRecord {
        tenant_id: tenant_id.to_owned(),
        principal_kind: principal_kind.to_owned(),`,
    `    Ok(RealtimeCheckpointRecord {
        tenant_id: tenant_id.to_owned(),
        organization_id: organization_id.to_owned(),
        principal_kind: principal_kind.to_owned(),`,
  ],
  [
    `        let k1 = checkpoint_key("tenant:a", "user", "b", "d1");
        let k2 = checkpoint_key("tenant", "user", "a:b", "d1");`,
    `        let k1 = checkpoint_key("tenant:a", "default", "user", "b", "d1");
        let k2 = checkpoint_key("tenant", "default", "user", "a:b", "d1");`,
  ],
  [
    `        let key = checkpoint_key("t1", "user", "u1", "d1");`,
    `        let key = checkpoint_key("t1", "default", "user", "u1", "d1");`,
  ],
  [
    `                ("tenant_id", record.tenant_id.clone()),
                ("principal_kind", record.principal_kind.clone()),`,
    `                ("tenant_id", record.tenant_id.clone()),
                ("organization_id", record.organization_id.clone()),
                ("principal_kind", record.principal_kind.clone()),`,
  ],
]);

console.log('adapter batch patch done');
