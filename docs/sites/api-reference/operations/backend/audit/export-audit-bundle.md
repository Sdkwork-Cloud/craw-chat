# `GET /backend/v3/api/audit/export`

<p class="api-page-intro">
  Exact request and response contract for <strong>Audit</strong> in the <strong>Backend API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/backend/audit"><code>Audit</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/backend-api"><code>Backend API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/audit/export</code>
  <span class="api-op-id">operationId: exportAuditBundle</span>
</div>

Streams an audit bundle containing the records visible at the request's fixed audit-sequence
high watermark. The service reads the ledger with bounded keyset pages and emits JSON through a
backpressured response body; it does not materialize the tenant's complete audit history in
process memory.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / audit</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`audit.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AuditExportBundle`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="AuditExportBundle" />

The export payload includes `chainHeadHash` and `chainValid` so offline verifiers can detect
tampering before import.

`total`, `chainHeadHash`, and `chainValid` are emitted after the streamed `items` array has been
scanned. Consumers must accept the bundle only after the response body completes and parses as a
complete JSON document. A transport or storage failure after response streaming starts terminates
the body, so a truncated or invalid document is a failed export and must be retried. Concurrent
appends above the captured high watermark belong to a later export.

Each service instance shares a bounded scan gate between export and verification requests. Configure
it with `SDKWORK_IM_AUDIT_MAX_CONCURRENT_SCANS` (default `4`, hard maximum `32`) so slow clients
cannot create unbounded memory or database pressure.

Standalone assembly serves this stream in-process. When an external audit-service upstream is
configured, the cloud gateway forwards the response body incrementally instead of applying its
normal buffered-response size cap.


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller lacks `audit.read`. |
| `503` | `50301` | Export concurrency is saturated or the audit store is unavailable before streaming starts. |

</section>
