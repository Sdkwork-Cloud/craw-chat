import fs from 'node:fs';
import path from 'node:path';

const filePath = path.join(
  path.resolve(import.meta.dirname, '../..'),
  'adapters/postgres-realtime/src/lib.rs',
);
let s = fs.readFileSync(filePath, 'utf8');

const sigOld = `        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,`;
const sigNew = `        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,`;
s = s.split(sigOld).join(sigNew);

// scope key helper
s = s.replace(
  `pub fn postgres_realtime_client_route_scope_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    scope_key(&[tenant_id, principal_kind, principal_id, device_id])
}

fn scope_key(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|")
}`,
  `pub fn postgres_realtime_client_route_scope_key(
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
);

// clone organization_id in async closures alongside tenant_id
s = s.replace(
  `        let tenant_id = tenant_id.to_owned();
        let principal_kind = principal_kind.to_owned();`,
  `        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();`,
);

// scope key calls
s = s.replace(
  `postgres_realtime_client_route_scope_key(
                tenant_id.as_str(),
                principal_kind.as_str(),
                principal_id.as_str(),
                device_id.as_str(),
            )`,
  `postgres_realtime_client_route_scope_key(
                tenant_id.as_str(),
                organization_id.as_str(),
                principal_kind.as_str(),
                principal_id.as_str(),
                device_id.as_str(),
            )`,
);

s = s.replace(
  `postgres_realtime_client_route_scope_key(
                    record.tenant_id.as_str(),
                    record.principal_kind.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                )`,
  `postgres_realtime_client_route_scope_key(
                    record.tenant_id.as_str(),
                    record.organization_id.as_str(),
                    record.principal_kind.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                )`,
);

// query bindings with scope key
s = s.replace(
  `&[&tenant_id, &client_route_scope_key]`,
  `&[&tenant_id, &organization_id, &client_route_scope_key]`,
);
s = s.replace(
  `&[&tenant_id, &client_route_scope_key, &cutoff_synced_at]`,
  `&[&tenant_id, &organization_id, &client_route_scope_key, &cutoff_synced_at]`,
);
s = s.replace(
  `&[&tenant_id, &client_route_scope_key, &after_seq, &limit]`,
  `&[&tenant_id, &organization_id, &client_route_scope_key, &after_seq, &limit]`,
);
s = s.replace(
  `&[&tenant_id, &client_route_scope_key, &acked_through_seq_i64]`,
  `&[&tenant_id, &organization_id, &client_route_scope_key, &acked_through_seq_i64]`,
);

// disconnect fence bindings
s = s.replace(
  `&[&tenant_id, &principal_kind, &principal_id, &device_id]`,
  `&[&tenant_id, &organization_id, &principal_kind, &principal_id, &device_id]`,
);
s = s.replace(
  `&[&tenant_id, &principal_kind, &principal_id, &device_id, &record.fence_token]`,
  `&[&tenant_id, &organization_id, &principal_kind, &principal_id, &device_id, &record.fence_token]`,
);
s = s.replace(
  `&[&tenant_id, &principal_kind, &principal_id, &device_id, &cutoff_disconnected_at]`,
  `&[&tenant_id, &organization_id, &principal_kind, &principal_id, &device_id, &cutoff_disconnected_at]`,
);

// upsert checkpoint
s = s.replace(
  `&[
                            &record.tenant_id,
                            &client_route_scope_key,
                            &record.principal_kind,`,
  `&[
                            &record.tenant_id,
                            &record.organization_id,
                            &client_route_scope_key,
                            &record.principal_kind,`,
);

// upsert disconnect fence
s = s.replace(
  `&[
                        &record.tenant_id,
                        &record.principal_kind,
                        &record.principal_id,
                        &record.device_id,`,
  `&[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.principal_kind,
                        &record.principal_id,
                        &record.device_id,`,
);

// row mappers
s = s.replace(
  `    Ok(RealtimeCheckpointRecord {
        tenant_id: row.get("tenant_id"),
        principal_kind: row.get("principal_kind"),`,
  `    Ok(RealtimeCheckpointRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        principal_kind: row.get("principal_kind"),`,
);

s = s.replace(
  `    RealtimeCheckpointRecord {
        tenant_id: record.tenant_id.clone(),
        principal_kind: record.principal_kind.clone(),`,
  `    RealtimeCheckpointRecord {
        tenant_id: record.tenant_id.clone(),
        organization_id: record.organization_id.clone(),
        principal_kind: record.principal_kind.clone(),`,
);

s = s.replace(
  `    Ok(RealtimeSubscriptionRecord {
        tenant_id: row.get("tenant_id"),
        principal_kind: row.get("principal_kind"),`,
  `    Ok(RealtimeSubscriptionRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        principal_kind: row.get("principal_kind"),`,
);

s = s.replace(
  `    Ok(RealtimeDisconnectFenceRecord {
        tenant_id: row.get("tenant_id"),
        principal_kind: row.get("principal_kind"),`,
  `    Ok(RealtimeDisconnectFenceRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        principal_kind: row.get("principal_kind"),`,
);

// matching subscriptions query - add organization_id to bindings if present
s = s.replace(
  `query.tenant_id,
                    query.principal_kind,
                    query.principal_id,`,
  `query.tenant_id,
                    query.organization_id,
                    query.principal_kind,
                    query.principal_id,`,
);

fs.writeFileSync(filePath, s);
console.log('postgres-realtime lib patched');
