# Sdkwork IM — Billing & Metering Architecture

Status: active
Owner: SDKWork maintainers
Updated: 2026-07-03
Specs: API_SPEC.md, IAM_SPEC.md, DATABASE_SPEC.md, CONFIG_SPEC.md

## 1. Purpose

This document defines the billing and metering architecture for the Sdkwork IM platform. It
covers metering event types, usage tracking, billing cycle management, invoice generation,
overage calculation, quota enforcement, and payment integration patterns.

It supplements the commercial pricing model in `docs/product/pricing/README.md` with the
technical implementation required for commercial billing operations.

## 2. Metering Architecture

### 2.1 Design Principles

1. **Accuracy**: Usage is recorded at the point of action, not estimated.
2. **Durability**: Metering events are persisted before the user-facing operation completes.
3. **Idempotency**: Each metering event has a unique ID; duplicates are deduplicated.
4. **Tenant isolation**: All metering is scoped to `(tenant_id, organization_id)`.
5. **Auditability**: Metering events are immutable and included in the audit log.

### 2.2 Metering Pipeline

```text
User Action
  -> Service emits metering event
    -> Kafka / Redis Stream (event buffer)
      -> Metering consumer (aggregation)
        -> PostgreSQL metering_store table (durable)
          -> Daily aggregation job
            -> Billing system (invoice generation)
```

Current implementation: metering events are written directly to PostgreSQL through the
`projection-service` durable journal, ensuring at-least-once delivery. A daily aggregation
job computes per-tenant usage summaries.

## 3. Metering Event Types

### 3.1 Billable Event Catalog

| Event Type | Source Service | Unit | Billable | Description |
| --- | --- | --- | --- | --- |
| `message.sent` | conversation-service | count | Yes | Message created by user |
| `message.delivered` | projection-service | count | No | Delivery confirmation (operational metric) |
| `connection.active` | session-gateway | seconds | Yes | WebSocket connection duration |
| `conversation.created` | conversation-service | count | Yes | New conversation created |
| `storage.used` | drive-app-sdk | bytes | Yes | Media storage consumed |
| `api.call` | im-gateway | count | Yes | API request count (for overage) |
| `push.sent` | push-service | count | Yes | Push notification delivered |
| `search.query` | search-service | count | Yes | Full-text search query |
| `rtc.call.minutes` | im-calls-service | seconds | Yes | Call duration (signaling only) |
| `user.active` | session-gateway | count | Yes | Daily active user |

### 3.2 Event Schema

Each metering event follows the standard SdkWork API response envelope input shape:

```json
{
  "eventId": "uuid-v7",
  "eventType": "message.sent",
  "tenantId": "string",
  "organizationId": "string",
  "userId": "string",
  "timestamp": "ISO 8601 UTC",
  "resourceId": "string",
  "quantity": 1,
  "metadata": {
    "conversationId": "string",
    "messageType": "text | image | file",
    "sizeBytes": 0
  },
  "idempotencyKey": "tenantId|resourceId|eventType|timestamp"
}
```

- `eventId`: UUIDv7 for time-ordered uniqueness.
- `idempotencyKey`: Composite key for deduplication.
- `quantity`: Numeric value for the metered unit (default: 1).
- `metadata`: Event-specific attributes, never containing message content.

## 4. Usage Tracking

### 4.1 Real-Time Tracking

Usage counters are maintained in Redis for real-time quota enforcement:

```text
counter_key = "meter:{tenantId}:{eventType}:{yyyyMM}:{resourceId}"
counter_value = INCR_BY(key, quantity) with TTL of 35 days
```

- Counters are scoped per tenant, event type, and calendar month.
- TTL ensures stale counters are cleaned up after the billing cycle closes.
- Real-time counters are eventually consistent with the durable PostgreSQL store.

### 4.2 Durable Storage

The PostgreSQL `metering_store` table is the durable source of truth:

```sql
CREATE TABLE metering_store (
  event_id        UUID PRIMARY KEY,
  tenant_id       VARCHAR(64) NOT NULL,
  organization_id VARCHAR(64) NOT NULL,
  event_type      VARCHAR(64) NOT NULL,
  user_id         VARCHAR(64),
  resource_id     VARCHAR(128),
  quantity        BIGINT NOT NULL DEFAULT 1,
  occurred_at     TIMESTAMPTZ NOT NULL,
  recorded_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  idempotency_key VARCHAR(256) NOT NULL,
  metadata        JSONB,
  CONSTRAINT uq_metering_idempotency UNIQUE (idempotency_key)
);

CREATE INDEX idx_metering_tenant_month
  ON metering_store (tenant_id, organization_id, event_type, occurred_at);
```

- The `idempotency_key` unique constraint prevents duplicate billing.
- The composite index supports efficient monthly aggregation queries.

### 4.3 Daily Aggregation

A scheduled job runs daily at 01:00 UTC to aggregate usage:

```sql
INSERT INTO metering_daily_summary
  (tenant_id, organization_id, event_type, summary_date, total_quantity, unique_users)
SELECT
  tenant_id,
  organization_id,
  event_type,
  DATE(occurred_at),
  SUM(quantity),
  COUNT(DISTINCT user_id)
FROM metering_store
WHERE occurred_at >= $1 AND occurred_at < $2
GROUP BY tenant_id, organization_id, event_type, DATE(occurred_at)
ON CONFLICT (tenant_id, organization_id, event_type, summary_date)
DO UPDATE SET
  total_quantity = EXCLUDED.total_quantity,
  unique_users = EXCLUDED.unique_users;
```

## 5. Quota Enforcement

### 5.1 Quota Configuration

Each tenant has a quota profile stored in the tenant configuration:

```json
{
  "tenantId": "string",
  "edition": "professional | enterprise",
  "quota": {
    "concurrentUsers": 100,
    "monthlyMessages": 1000000,
    "storageBytes": 107374182400,
    "apiCallsPerMinute": 1000,
    "pushNotificationsPerDay": 10000,
    "searchQueriesPerDay": 5000,
    "rtcCallMinutesPerMonth": 5000
  },
  "overage": {
    "enabled": true,
    "ratePerUnit": {
      "message": 0.0001,
      "storageGB": 0.10,
      "apiCall": 0.00001
    }
  }
}
```

### 5.2 Enforcement Points

| Quota | Enforcement Layer | Behavior on Exceed |
| --- | --- | --- |
| Concurrent users | session-gateway | Reject new WebSocket connections with 429 |
| API calls per minute | im-gateway (rate limiter) | Return 429 with Retry-After header |
| Monthly messages | conversation-service | Accept with overage flag; reject if overage disabled |
| Storage | drive-app-sdk | Reject uploads with 413 |
| Push notifications | push-service | Queue for next cycle or reject with 429 |
| Search queries | search-service | Return 429 with Retry-After header |
| RTC call minutes | im-calls-service | Reject new calls with 402 |

### 5.3 Rate Limiting Architecture

Two-layer rate limiting as defined in `SECURITY_SPEC.md`:

1. **Layer 1 (Per-IP)**: Enforced at the gateway using a sliding window counter in Redis.
   - Default: 100 requests/minute per IP.
   - Burst: 200 requests in the first second.
2. **Layer 2 (Per-tenant)**: Enforced at the application layer using the tenant quota.
   - Configurable per tenant.
   - Returns `429 Too Many Requests` with `Retry-After` header.

Error response follows the standard ProblemDetail format:

```json
{
  "type": "https://docs.sdkwork.com/errors/quota-exceeded",
  "status": 429,
  "title": "Quota exceeded",
  "detail": "Monthly message quota exceeded. Overage billing is enabled.",
  "code": 42901,
  "traceId": "uuid",
  "retryAfter": 3600
}
```

Numeric error code `42901` follows `API_SPEC.md` section 15.3.

## 6. Billing Cycle

### 6.1 Cycle Definition

- **Billing period**: Calendar month, 00:00 UTC on the 1st through 23:59 UTC on the last day.
- **Invoice generation**: 1st of each month at 03:00 UTC (after aggregation completes).
- **Payment due**: 15 days from invoice date (net-15 terms).
- **Grace period**: 7 days after due date before service suspension.

### 6.2 Invoice Components

```text
Invoice = Base subscription
        + Overage charges (messages, storage, API calls)
        + Add-on services (extra concurrent users, premium support)
        - Credits (SLA credits, promotional credits)
        = Total due
```

### 6.3 Invoice Data Source

The billing system reads from the `metering_daily_summary` table:

```sql
SELECT
  event_type,
  SUM(total_quantity) AS monthly_quantity,
  SUM(unique_users) AS monthly_active_users
FROM metering_daily_summary
WHERE tenant_id = $1
  AND summary_date >= DATE_TRUNC('month', $2)
  AND summary_date < DATE_TRUNC('month', $2 + INTERVAL '1 month')
GROUP BY event_type;
```

## 7. Overage Calculation

### 7.1 Message Overage

```text
included = tenant.quota.monthlyMessages
used = SUM(message.sent events for current month)
overage = MAX(0, used - included)
charge = overage * tenant.overage.ratePerUnit.message
```

### 7.2 Storage Overage

```text
included = tenant.quota.storageBytes
used = MAX(storage.used events for current month)  -- last value wins
overage_bytes = MAX(0, used - included)
overage_gb = overage_bytes / 1073741824
charge = overage_gb * tenant.overage.ratePerUnit.storageGB
```

### 7.3 API Call Overage

```text
included = tenant.quota.apiCallsPerMinute * 60 * 24 * 30  -- monthly equivalent
used = SUM(api.call events for current month)
overage = MAX(0, used - included)
charge = overage * tenant.overage.ratePerUnit.apiCall
```

### 7.4 Concurrent User Overage

Concurrent users are measured as peak daily active users:

```text
included = tenant.quota.concurrentUsers
peak_daily = MAX(daily unique_users for current month)
overage = MAX(0, peak_daily - included)
charge = overage * overage_rate_per_user
```

## 8. Payment Integration

### 8.1 Integration Architecture

Payment processing is handled through the SDKWork platform billing system, not directly in the
IM application. The IM platform exposes usage data through:

1. **Metering API**: `GET /im/v3/api/admin/metering/summary` — Returns aggregated usage for a
   tenant within a date range.
2. **Webhook events**: Metering events forwarded to the billing system's webhook endpoint.
3. **Daily export**: S3 export of `metering_daily_summary` for batch billing processing.

### 8.2 Metering API

```http
GET /im/v3/api/admin/metering/summary?tenantId={id}&from={date}&to={date}

Response (SdkWorkApiResponse envelope):
{
  "code": 0,
  "data": {
    "items": [
      {
        "eventType": "message.sent",
        "totalQuantity": 1250000,
        "uniqueUsers": 850,
        "periodStart": "2026-07-01T00:00:00Z",
        "periodEnd": "2026-07-31T23:59:59Z"
      }
    ],
    "pageInfo": {
      "mode": "offset",
      "page": 1,
      "pageSize": 20,
      "total": 10
    }
  },
  "traceId": "uuid"
}
```

This follows the standard `SdkWorkApiResponse` envelope per `API_SPEC.md` section 4.5.

### 8.3 Webhook Event

```json
{
  "eventType": "metering.daily_summary",
  "tenantId": "string",
  "summaryDate": "2026-07-03",
  "usage": {
    "message.sent": 42000,
    "connection.active": 3600000,
    "storage.used": 1073741824,
    "api.call": 150000
  },
  "webhookId": "uuid-v7",
  "timestamp": "2026-07-04T01:05:00Z"
}
```

Webhooks are signed with HMAC-SHA256 using the billing system's shared secret.

## 9. Edition-Specific Billing

### 9.1 Community Edition

- No billing; usage is unlimited but unsupported.
- Metering events are still recorded for upgrade migration.
- No quota enforcement (best-effort only).

### 9.2 Professional Edition

- Monthly subscription fee includes base quota.
- Overage billing is enabled by default.
- Invoice is generated monthly with net-15 payment terms.
- Payment methods: credit card, bank transfer.

### 9.3 Enterprise Edition

- Annual contract with custom quota and pricing.
- Overage is negotiated in the master service agreement (MSA).
- Invoice is generated monthly; payment terms per MSA (typically net-30).
- Dedicated support and custom SLA terms.

## 10. Tenant Lifecycle and Billing

### 10.1 Provisioning

1. **Trial**: 14-day trial with Professional quota. No billing.
2. **Conversion**: Tenant admin selects an edition and enters payment information.
3. **Activation**: Billing system confirms payment and updates tenant quota configuration.
4. **Backfill**: Pre-conversion metering events are included in the first invoice.

### 10.2 Upgrade/Downgrade

- **Upgrade**: Takes effect immediately; quota increased; prorated charge for the current cycle.
- **Downgrade**: Takes effect at the start of the next billing cycle; quota unchanged until then.
- **Cancellation**: Takes effect at the end of the current billing cycle; data retained for 30
  days, then purged.

### 10.3 Suspension

- **Voluntary**: Tenant admin requests suspension; data retained for 90 days.
- **Involuntary**: Non-payment after grace period; data retained for 30 days, then purged.
- **Reactivation**: Billing system confirms payment; quota and data restored.

## 11. Reporting and Analytics

### 11.1 Tenant Usage Dashboard

Available in the admin console at `/console/billing`:

1. **Current cycle usage**: Real-time counters vs. quota with progress bars.
2. **Historical trends**: 12-month usage charts per event type.
3. **Cost breakdown**: Base subscription, overage charges, credits.
4. **Forecast**: Projected usage and cost for the current cycle.

### 11.2 Operational Reports

- **Daily usage report**: Per-tenant usage summary, emailed to tenant admins.
- **Monthly invoice**: PDF invoice with detailed usage breakdown.
- **Overage alert**: Email when usage exceeds 80% and 100% of quota.

## 12. Data Retention for Billing

| Data Type | Retention | Reason |
| --- | --- | --- |
| `metering_store` (raw events) | 13 months | Tax and dispute resolution |
| `metering_daily_summary` | 7 years | Tax and audit compliance |
| Invoices | 7 years | Tax and audit compliance |
| Payment records | 7 years | Tax and audit compliance |
| Tenant quota configuration history | 7 years | Audit trail |

## 13. References

- [docs/product/pricing/README.md](../pricing/README.md) — Commercial pricing model.
- [SLA_SLO.md](../compliance/SLA_SLO.md) — SLA credit policy.
- [COMPLIANCE_FRAMEWORK.md](../compliance/COMPLIANCE_FRAMEWORK.md) — Data retention and audit.
- `../sdkwork-specs/API_SPEC.md` — API response envelope and error codes.
- `../sdkwork-specs/IAM_SPEC.md` — Tenant identity and access management.
- `../sdkwork-specs/DATABASE_SPEC.md` — Database schema standards.
