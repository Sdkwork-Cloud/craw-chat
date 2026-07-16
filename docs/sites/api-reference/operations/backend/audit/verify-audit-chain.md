# `GET /backend/v3/api/audit/verify`

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
  <code>/backend/v3/api/audit/verify</code>
  <span class="api-op-id">operationId: verifyAuditChain</span>
</div>

Verifies the visible audit hash chain through a fixed audit-sequence high watermark and returns
the verified chain head. Verification uses bounded keyset pages and carries only the previous hash,
expected sequence, count, and current head across pages; it does not load the complete ledger into
memory.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / audit</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`audit.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AuditChainVerification`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="AuditChainVerification" />

The response includes `chainHeadHash` and `chainValid` for operator-side integrity checks.
Validation covers tenant identity, contiguous server sequence numbers, previous-hash linkage,
record hash recomputation, and arrival at the captured chain head. Concurrent appends above the
captured high watermark do not change the result of the in-flight verification.

Verification shares the per-instance bounded audit scan gate configured by
`SDKWORK_IM_AUDIT_MAX_CONCURRENT_SCANS` (default `4`, hard maximum `32`).

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller lacks `audit.read`. |
| `503` | `50301` | The audit store is unavailable. |

</section>
