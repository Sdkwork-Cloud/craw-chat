> Migrated from `docs/sites/api-reference/operations/backend/audit/export-audit-bundle.md` on 2026-06-24.
> Owner: SDKWork maintainers

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

Streams an audit bundle containing records through the audit-sequence high watermark captured at
the start of the request. Storage reads use bounded keyset pages and response writes use a bounded,
backpressured channel, so service memory does not grow with tenant audit-history cardinality.

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

The trailing `total`, `chainHeadHash`, and `chainValid` fields are authoritative only after the
entire JSON response completes. Storage or transport failure during streaming produces an
incomplete body that consumers must reject. Per-instance concurrent export work is limited by
the shared `SDKWORK_IM_AUDIT_MAX_CONCURRENT_SCANS` gate (default `4`, maximum `32`).
The cloud gateway preserves streaming for this exact route when audit-service is configured as an
external upstream; other proxied JSON operations retain the normal bounded buffering policy.


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `audit.read`. |
| `503` | `dependency_unavailable` | Export concurrency is saturated or storage is unavailable before streaming starts. |

</section>
