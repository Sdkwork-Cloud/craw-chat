# Admin Storage Contract

This page documents the current `/backend/v3/api/admin/storage/*` contract used by
`apps/sdkwork-im-admin` and the workspace admin sandbox. It is implementation-aligned documentation
for the route surface that exists and is verified in this repository today.

It is part of the backend admin API boundary. Current generated SDK ownership belongs to
`sdkwork-im-backend-sdk`; this route set must not be split into a standalone admin SDK family.

## Current Boundary

| Layer | Current responsibility |
| --- | --- |
| `im-storage-contracts` | Canonical storage schema, scope, validation, redaction, and snapshot contracts |
| `im-storage-runtime` | Save, delete, validate, resolve, audit, and store-backed persistence orchestration |
| `sdkwork-api-product-runtime` | Rust-backed desktop admin sandbox that serves `/backend/v3/api/admin/storage/*` |
| `apps/sdkwork-im-admin/dev/admin-sandbox.mjs` | JavaScript admin sandbox used for frontend verification and local walkthroughs |
| `apps/sdkwork-im-admin` | Operator UI and typed admin API wrapper that consume the contract |

The admin storage route set is therefore a real contract, but its current executable surfaces are
workspace admin runtimes rather than a published control-plane service.

## Access Model

| Concern | Current rule |
| --- | --- |
| Route prefix | `/backend/v3/api/admin/storage/*` |
| Auth | Same SDKWork appbase credential and AppContext projection model used by the rest of `/backend/v3/api/admin/*` |
| Read permission | `admin.storage.read` |
| Write permission | `admin.storage.write` |
| Secret reads | Responses expose `StorageSecretSummaryRecord`, never raw `encryptedSecretPayload` |
| Tenant semantics | Tenant config is a whole-record override; if absent, reads fall back to the global default |
| Sandbox persistence | In-memory by default, or file-backed when `SDKWORK_ADMIN_SANDBOX_STORAGE_FILE` is set |

## Route Catalog

| Method | Route | Purpose | Permission |
| --- | --- | --- | --- |
| `GET` | `/backend/v3/api/admin/storage/providers` | Return provider schemas, supported credential modes, and capability tags | `admin.storage.read` |
| `GET` | `/backend/v3/api/admin/storage/config` | Read the global storage default | `admin.storage.read` |
| `POST` | `/backend/v3/api/admin/storage/config` | Create or update the global storage default | `admin.storage.write` |
| `GET` | `/backend/v3/api/admin/storage/config/tenants/{tenantId}` | Read a tenant override snapshot | `admin.storage.read` |
| `POST` | `/backend/v3/api/admin/storage/config/tenants/{tenantId}` | Create or update a tenant override | `admin.storage.write` |
| `DELETE` | `/backend/v3/api/admin/storage/config/tenants/{tenantId}` | Remove a tenant override so effective reads fall back to global | `admin.storage.write` |
| `GET` | `/backend/v3/api/admin/storage/effective/tenants/{tenantId}` | Resolve the effective tenant storage policy after fallback rules | `admin.storage.read` |
| `POST` | `/backend/v3/api/admin/storage/validate` | Validate the global storage target and return a stage-specific status | `admin.storage.write` |
| `POST` | `/backend/v3/api/admin/storage/validate/tenants/{tenantId}` | Validate the tenant override or inherited effective target | `admin.storage.write` |
| `GET` | `/backend/v3/api/admin/storage/audit` | Return recent storage writes and tenant-override deletions | `admin.storage.read` |

## Core Payload Shapes

| Shape | Used by | Notes |
| --- | --- | --- |
| `StorageProviderSchemaRecord` | `GET /providers` | Enumerates provider families, common fields, credential fields, supported modes, and capabilities |
| `StorageConfigUpsertInput` | `POST /config` and `POST /config/tenants/{tenantId}` | Typed write payload for binding, config, and optional secret rotation |
| `StorageConfigSnapshotRecord` | `GET /config` and `GET /config/tenants/{tenantId}` | Redacted scope snapshot containing binding, config, and secret summary |
| `StorageEffectiveConfigRecord` | `GET /effective/tenants/{tenantId}` | Shows requested scope, resolved scope, binding, config, and secret summary |
| `StorageValidationRecord` | `POST /validate*` | Reports `status`, `stage`, `message`, and provider context |
| `StorageAuditRecord` | `GET /audit` | Records writes and deletes with scope, provider, and timestamp |

## Write Contract

`StorageConfigUpsertInput` keeps the write surface explicit:

```json
{
  "binding": {
    "providerPluginId": "object-storage-aws",
    "enabled": true
  },
  "config": {
    "bucketOrContainer": "global-assets",
    "region": "us-east-1",
    "endpoint": "https://s3.amazonaws.com",
    "publicBaseUrl": "https://cdn.global.example",
    "uploadPrefix": "uploads/",
    "downloadPrefix": "downloads/",
    "providerConfig": {
      "forcePathStyle": false
    }
  },
  "secret": {
    "credentialMode": "access-key-pair",
    "encryptedSecretPayload": "{\"accessKeyId\":\"AKIA...\",\"secretAccessKey\":\"...\"}",
    "secretFingerprint": "fp-global-aws"
  }
}
```

Three behavioral rules matter:

1. Provider-specific common fields and credential fields come from the provider schema instead of a
   fake one-size-fits-all credential form.
2. Secret rotation is explicit. When the provider and credential mode stay the same, operators may
   submit config-only updates and preserve the existing secret.
3. Changing provider or credential mode requires a fresh secret submission so validation and future
   presigned-upload behavior stay aligned with the active provider.

## Read Contract And Redaction

Reads return redacted snapshots. A scope snapshot can show that credentials exist, which
credential mode is active, and which fingerprint identifies the stored secret, but it does not
return the raw secret payload:

```json
{
  "scope": {
    "kind": "global",
    "scopeId": null
  },
  "binding": {
    "scope": {
      "kind": "global",
      "scopeId": null
    },
    "providerPluginId": "object-storage-aws",
    "enabled": true
  },
  "config": {
    "scope": {
      "kind": "global",
      "scopeId": null
    },
    "providerPluginId": "object-storage-aws",
    "bucketOrContainer": "global-assets",
    "region": "us-east-1",
    "endpoint": "https://s3.amazonaws.com",
    "publicBaseUrl": "https://cdn.global.example",
    "uploadPrefix": "uploads/",
    "downloadPrefix": "downloads/",
    "providerConfig": {
      "forcePathStyle": false
    }
  },
  "secret": {
    "scope": {
      "kind": "global",
      "scopeId": null
    },
    "providerPluginId": "object-storage-aws",
    "credentialMode": "access-key-pair",
    "configured": true,
    "secretFingerprint": "fp-global-aws"
  }
}
```

This is the rule future backends and SDK-facing upload helpers must preserve: reads expose posture,
not secret material.

## Effective Resolution

Tenant reads follow one rule only: whole-record override or fallback. There is no field-level merge.

If a tenant override exists, `/backend/v3/api/admin/storage/effective/tenants/{tenantId}` resolves from the
tenant record. If the tenant override is deleted or was never created, the same route resolves from
the global default:

```json
{
  "requestedScope": {
    "kind": "tenant",
    "scopeId": "tenant_northstar"
  },
  "resolvedScope": {
    "kind": "global",
    "scopeId": null
  },
  "binding": {
    "scope": {
      "kind": "global",
      "scopeId": null
    },
    "providerPluginId": "object-storage-aws",
    "enabled": true
  },
  "config": {
    "scope": {
      "kind": "global",
      "scopeId": null
    },
    "providerPluginId": "object-storage-aws",
    "bucketOrContainer": "global-assets",
    "region": "us-east-1",
    "endpoint": "https://s3.amazonaws.com",
    "publicBaseUrl": "https://cdn.global.example",
    "uploadPrefix": null,
    "downloadPrefix": null,
    "providerConfig": {}
  },
  "secret": {
    "scope": {
      "kind": "global",
      "scopeId": null
    },
    "providerPluginId": "object-storage-aws",
    "credentialMode": "access-key-pair",
    "configured": true,
    "secretFingerprint": "fp-global-aws"
  }
}
```

That fallback behavior is what the admin UI surfaces in the Storage page posture rail and effective
resolution preview.

## Validation Contract

Validation endpoints return a `StorageValidationRecord` with two operator-facing dimensions:

| Field | Meaning |
| --- | --- |
| `status` | `healthy`, `degraded`, `invalid`, or `unknown` |
| `stage` | `schema`, `credentials`, `bucket`, `presign`, or `readback` |

Current admin flows use this contract to answer questions such as:

- did the selected provider schema validate
- were required credential fields submitted for the chosen credential mode
- can the selected target reach the bucket or container contract
- is presigned upload issuance likely to work for the current effective target

The current sandbox uses `POST /backend/v3/api/admin/storage/validate` and
`POST /backend/v3/api/admin/storage/validate/tenants/{tenantId}` with an empty JSON object body.

## Sandbox And Promotion Rules

The current repository supports two verified execution modes for the admin storage contract:

| Mode | Current behavior |
| --- | --- |
| In-memory sandbox | `SDKWORK_ADMIN_SANDBOX=1` gives a fast, reset-on-restart contract surface |
| File-backed sandbox | `SDKWORK_ADMIN_SANDBOX=1` plus `SDKWORK_ADMIN_SANDBOX_STORAGE_FILE=/absolute/path/storage-snapshots.json` persists global writes, tenant overrides, and override deletes across restarts |

The file-backed mode is useful for operator walkthroughs and contract testing, but it still does
not promote the route set into a production control-plane API.

## Integration Guidance

If you are extending this contract:

1. Keep request decoding anchored on `StorageConfigUpsertInput`.
2. Keep secret reads redacted through `StorageSecretSummaryRecord`.
3. Keep tenant fallback as whole-record replacement only.
4. Reuse `im-storage-runtime` for validation, effective resolution, audit, and persistence
   orchestration instead of rebuilding provider logic in each backend.
5. Only add a public OpenAPI reference page for storage routes after the same contract is served by
   a non-sandbox backend with a checked-in authority file.
