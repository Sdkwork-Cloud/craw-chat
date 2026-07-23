# Audit

<p class="api-page-intro">
  Audit endpoints record audit anchors and expose read and export flows for audit evidence.
</p>

<div class="api-link-list">
  <a href="/api-reference/backend/ops"><code>Backend Ops</code> Runtime diagnostics and cluster views are documented separately</a>
  <a href="/sdk/backend-sdk"><code>Backend SDK</code> Audit flows belong to <code>sdkwork-im-backend-sdk</code></a>
</div>

<a id="record-audit-anchor"></a>
<section class="api-op">

## `POST /backend/v3/api/audit/records`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/audit/records</code>
  <span class="api-op-id">operationId: records.create</span>
</div>

Writes a new audit record.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / audit</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`audit.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201 AuditRecord in data.item`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="RecordAuditAnchor" />

### Response `201`

<ApiSchemaTable schema="AuditRecord" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The audit anchor payload is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `audit.write`. |

</section>
<a id="list-audit-records"></a>
<section class="api-op">

## `GET /backend/v3/api/audit/records`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/audit/records</code>
  <span class="api-op-id">operationId: records.list</span>
</div>

Lists audit records visible to the current principal.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / audit</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`audit.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AuditRecordListResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="AuditRecordListResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `audit.read`. |

</section>
<a id="export-audit-bundle"></a>
<section class="api-op">

## `GET /backend/v3/api/audit/export`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/audit/export</code>
  <span class="api-op-id">operationId: export.retrieve</span>
</div>

Exports an audit bundle containing the visible records at the time of the request.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / audit</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`audit.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AuditExportBundle`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="AuditExportBundle" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `audit.read`. |

</section>
