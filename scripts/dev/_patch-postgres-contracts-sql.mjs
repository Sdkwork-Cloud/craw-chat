import fs from 'node:fs';
import path from 'node:path';

const filePath = path.join(
  path.resolve(import.meta.dirname, '../..'),
  'crates/im-postgres-realtime-contracts/src/lib.rs',
);
let s = fs.readFileSync(filePath, 'utf8');

const replacements = [
  // device-scoped WHERE (2-param -> 3-param)
  [
    'where tenant_id = $1 and client_route_scope_key = $2',
    'where tenant_id = $1 and organization_id = $2 and client_route_scope_key = $3',
  ],
  [
    `where tenant_id = $1
  and client_route_scope_key = $2`,
    `where tenant_id = $1
  and organization_id = $2
  and client_route_scope_key = $3`,
  ],
  // ON CONFLICT keys
  [
    'on conflict (tenant_id, client_route_scope_key) do update set',
    'on conflict (tenant_id, organization_id, client_route_scope_key) do update set',
  ],
  [
    'on conflict (tenant_id, client_route_scope_key, realtime_seq) do nothing',
    'on conflict (tenant_id, organization_id, client_route_scope_key, realtime_seq) do nothing',
  ],
  [
    'on conflict (tenant_id, principal_kind, principal_id, device_id) do update set',
    'on conflict (tenant_id, organization_id, principal_kind, principal_id, device_id) do update set',
  ],
  // checkpoint load/upsert columns
  [
    `insert into im_realtime_checkpoints (
    tenant_id,
    client_route_scope_key,`,
    `insert into im_realtime_checkpoints (
    tenant_id,
    organization_id,
    client_route_scope_key,`,
  ],
  [
    `) values (
    $1, $2, $3, $4, $5,
    $6, $7, $8, $9, $10, $11, $12, $13
)`,
    `) values (
    $1, $2, $3, $4, $5, $6,
    $7, $8, $9, $10, $11, $12, $13, $14
)`,
  ],
  [
    `insert into im_realtime_device_events (
    tenant_id,
    client_route_scope_key,`,
    `insert into im_realtime_device_events (
    tenant_id,
    organization_id,
    client_route_scope_key,`,
  ],
  [
    `) values (
    $1, $2, $3, $4, $5, $6, $7,
    $8, $9, $10, $11::jsonb, $12, $13, $14, $15
)`,
    `) values (
    $1, $2, $3, $4, $5, $6, $7, $8,
    $9, $10, $11, $12::jsonb, $13, $14, $15, $16
)`,
  ],
  [
    `  and realtime_seq > $3
order by realtime_seq asc
limit $4`,
    `  and realtime_seq > $4
order by realtime_seq asc
limit $5`,
  ],
  [
    `  and realtime_seq <= $3`,
    `  and realtime_seq <= $4`,
  ],
  [
    `insert into im_realtime_subscriptions (
    tenant_id,
    client_route_scope_key,`,
    `insert into im_realtime_subscriptions (
    tenant_id,
    organization_id,
    client_route_scope_key,`,
  ],
  [
    `) values (
    $1, $2, $3, $4, $5, $6::jsonb, $7, $8, $9, $10
)`,
    `) values (
    $1, $2, $3, $4, $5, $6, $7::jsonb, $8, $9, $10, $11
)`,
  ],
  [
    `  and synced_at <= $3`,
    `  and synced_at <= $4`,
  ],
  [
    `insert into im_realtime_disconnect_fences (
    tenant_id,
    principal_kind,`,
    `insert into im_realtime_disconnect_fences (
    tenant_id,
    organization_id,
    principal_kind,`,
  ],
  [
    `) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9::jsonb, $10, $11, $12
)`,
    `) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10::jsonb, $11, $12, $13
)`,
  ],
  [
    `where tenant_id = $1
  and principal_kind = $2
  and principal_id = $3
  and device_id = $4`,
    `where tenant_id = $1
  and organization_id = $2
  and principal_kind = $3
  and principal_id = $4
  and device_id = $5`,
  ],
  [
    `  and fence_token = $5`,
    `  and fence_token = $6`,
  ],
  [
    `  and disconnected_at <= $5`,
    `  and disconnected_at <= $6`,
  ],
  // diagnostics joins
  [
    `  on e.tenant_id = c.tenant_id
 and e.client_route_scope_key = c.client_route_scope_key`,
    `  on e.tenant_id = c.tenant_id
 and e.organization_id = c.organization_id
 and e.client_route_scope_key = c.client_route_scope_key`,
  ],
  [
    `  on window_counts.tenant_id = c.tenant_id
 and window_counts.client_route_scope_key = c.client_route_scope_key`,
    `  on window_counts.tenant_id = c.tenant_id
 and window_counts.organization_id = c.organization_id
 and window_counts.client_route_scope_key = c.client_route_scope_key`,
  ],
  [
    `    group by tenant_id, client_route_scope_key`,
    `    group by tenant_id, organization_id, client_route_scope_key`,
  ],
  [
    `  on c.tenant_id = e.tenant_id
 and c.client_route_scope_key = e.client_route_scope_key`,
    `  on c.tenant_id = e.tenant_id
 and c.organization_id = e.organization_id
 and c.client_route_scope_key = e.client_route_scope_key`,
  ],
  [
    `  on c.tenant_id = e.tenant_id
 and c.client_route_scope_key = e.client_route_scope_key
where c.client_route_scope_key is null`,
    `  on c.tenant_id = e.tenant_id
 and c.organization_id = e.organization_id
 and c.client_route_scope_key = e.client_route_scope_key
where c.client_route_scope_key is null`,
  ],
  [
    `  on s.tenant_id = fs.tenant_id
 and s.client_route_scope_key = fs.client_route_scope_key`,
    `  on s.tenant_id = fs.tenant_id
 and s.organization_id = fs.organization_id
 and s.client_route_scope_key = fs.client_route_scope_key`,
  ],
];

for (const [from, to] of replacements) {
  if (!s.includes(from)) {
    console.warn('WARN missing:', from.slice(0, 60));
  } else {
    s = s.split(from).join(to);
  }
}

// DEVICE_SCOPE_BINDINGS
s = s.replace(
  `const DEVICE_SCOPE_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", "Tenant partition key."),
    binding(
        2,
        "client_route_scope_key",`,
  `const DEVICE_SCOPE_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", "Tenant partition key."),
    binding(2, "organization_id", "&str", "text", "Organization partition key."),
    binding(
        3,
        "client_route_scope_key",`,
);

// UPSERT checkpoint bindings - shift indices
s = s.replace(
  `const UPSERT_REALTIME_CHECKPOINT_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "client_route_scope_key", "String", "text", ""),
    binding(3, "principal_kind", "&str", "text", ""),
    binding(4, "principal_id", "&str", "text", ""),
    binding(5, "device_id", "&str", "text", ""),
    binding(6, "latest_realtime_seq", "u64", "bigint", ""),
    binding(7, "acked_through_seq", "u64", "bigint", ""),
    binding(8, "trimmed_through_seq", "u64", "bigint", ""),
    binding(9, "capacity_trimmed_event_count", "u64", "bigint", ""),
    binding(10, "capacity_trimmed_through_seq", "u64", "bigint", ""),
    binding(
        11,
        "last_capacity_trimmed_at",
        "Option<&str>",
        "timestamptz",
        "Bind RFC3339 UTC timestamp when capacity trim metadata is present.",
    ),
    binding(12, "created_at", "&str", "timestamptz", ""),
    binding(13, "updated_at", "&str", "timestamptz", ""),
];`,
  `const UPSERT_REALTIME_CHECKPOINT_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "client_route_scope_key", "String", "text", ""),
    binding(4, "principal_kind", "&str", "text", ""),
    binding(5, "principal_id", "&str", "text", ""),
    binding(6, "device_id", "&str", "text", ""),
    binding(7, "latest_realtime_seq", "u64", "bigint", ""),
    binding(8, "acked_through_seq", "u64", "bigint", ""),
    binding(9, "trimmed_through_seq", "u64", "bigint", ""),
    binding(10, "capacity_trimmed_event_count", "u64", "bigint", ""),
    binding(11, "capacity_trimmed_through_seq", "u64", "bigint", ""),
    binding(
        12,
        "last_capacity_trimmed_at",
        "Option<&str>",
        "timestamptz",
        "Bind RFC3339 UTC timestamp when capacity trim metadata is present.",
    ),
    binding(13, "created_at", "&str", "timestamptz", ""),
    binding(14, "updated_at", "&str", "timestamptz", ""),
];`,
);

// event bindings
s = s.replace(
  `const UPSERT_REALTIME_CLIENT_ROUTE_EVENT_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "client_route_scope_key", "String", "text", ""),
    binding(3, "realtime_seq", "u64", "bigint", ""),
    binding(4, "principal_kind", "&str", "text", ""),
    binding(5, "principal_id", "&str", "text", ""),
    binding(6, "device_id", "&str", "text", ""),
    binding(7, "scope_type", "&str", "text", ""),
    binding(8, "scope_id", "&str", "text", ""),
    binding(9, "event_type", "&str", "text", ""),
    binding(10, "delivery_class", "&str", "text", ""),
    binding(
        11,
        "payload_json",
        "&str",
        "jsonb",
        "Serialized RealtimeEvent.payload JSON.",
    ),
    binding(12, "payload_hash", "&str", "text", ""),
    binding(13, "occurred_at", "&str", "timestamptz", ""),
    binding(14, "created_at", "&str", "timestamptz", ""),
    binding(15, "retention_until", "Option<&str>", "timestamptz", ""),
];`,
  `const UPSERT_REALTIME_CLIENT_ROUTE_EVENT_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "client_route_scope_key", "String", "text", ""),
    binding(4, "realtime_seq", "u64", "bigint", ""),
    binding(5, "principal_kind", "&str", "text", ""),
    binding(6, "principal_id", "&str", "text", ""),
    binding(7, "device_id", "&str", "text", ""),
    binding(8, "scope_type", "&str", "text", ""),
    binding(9, "scope_id", "&str", "text", ""),
    binding(10, "event_type", "&str", "text", ""),
    binding(11, "delivery_class", "&str", "text", ""),
    binding(
        12,
        "payload_json",
        "&str",
        "jsonb",
        "Serialized RealtimeEvent.payload JSON.",
    ),
    binding(13, "payload_hash", "&str", "text", ""),
    binding(14, "occurred_at", "&str", "timestamptz", ""),
    binding(15, "created_at", "&str", "timestamptz", ""),
    binding(16, "retention_until", "Option<&str>", "timestamptz", ""),
];`,
);

s = s.replace(
  `const LIST_REALTIME_CLIENT_ROUTE_EVENTS_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "client_route_scope_key", "String", "text", ""),
    binding(3, "after_seq", "u64", "bigint", ""),
    binding(4, "limit", "usize", "bigint", ""),
];`,
  `const LIST_REALTIME_CLIENT_ROUTE_EVENTS_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "client_route_scope_key", "String", "text", ""),
    binding(4, "after_seq", "u64", "bigint", ""),
    binding(5, "limit", "usize", "bigint", ""),
];`,
);

s = s.replace(
  `const TRIM_REALTIME_CLIENT_ROUTE_EVENTS_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "client_route_scope_key", "String", "text", ""),
    binding(3, "acked_through_seq", "u64", "bigint", ""),
];`,
  `const TRIM_REALTIME_CLIENT_ROUTE_EVENTS_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "client_route_scope_key", "String", "text", ""),
    binding(4, "acked_through_seq", "u64", "bigint", ""),
];`,
);

fs.writeFileSync(filePath, s);
console.log('postgres contracts SQL patched');
