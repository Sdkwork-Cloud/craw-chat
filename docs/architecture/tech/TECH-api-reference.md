> Migrated from `docs/api-reference.md` on 2026-06-24.
> Owner: SDKWork maintainers

This document is a navigation index for the canonical HTTP contracts. It does not define paths on its own.

## Authority

| Surface | Prefix | OpenAPI authority | Generated SDK |
| --- | --- | --- | --- |
| IM (open-api) | `/im/v3/api` | `sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml` | `sdkwork-im-sdk` |
| App | `/app/v3/api` | `sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.openapi.yaml` | `sdkwork-im-app-sdk` |
| Backend | `/backend/v3/api` | `sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml` | `sdkwork-im-backend-sdk` |

Rules follow `API_SPEC.md`: new work must not add `/api/v1/*` or `/im/v3/api/interactions/*` trees.

Architecture decision: [ADR-20260617-comms-service-naming-boundaries](../architecture/decisions/ADR-20260617-comms-service-naming-boundaries.md).

## OpenAPI tags (IM plane)

| Tag | Write owner (split deploy) | Path prefix |
| --- | --- | --- |
| `presence` | `session-gateway` | `/im/v3/api/presence/` |
| `realtime` | `session-gateway` | `/im/v3/api/realtime/` |
| `calls` | `im-calls-service` | `/im/v3/api/calls/` |
| `social` | `comms-social-service` (`social-service` legacy alias) | `/im/v3/api/social/` |
| `chat` | `comms-conversation-service` (write), `projection-service` (read) | `/im/v3/api/chat/` |
| `spaces` | `comms-space-service` (`space-service` legacy alias) | `/im/v3/api/spaces/` |
| `streams` | `streaming-service` | `/im/v3/api/streams/` |

Reactions, pins, threads, and conversation settings are **`chat` resources** under
`/im/v3/api/chat/conversations/...` and `/im/v3/api/chat/messages/...`, not a separate interactions namespace.

## Deprecated surfaces

| Legacy name | Status | Replacement |
| --- | --- | --- |
| `contact-service` HTTP routes | Deprecated library scaffold | `social-service` / `comms-social-service` on `/im/v3/api/social/*` |
| `interaction-service` HTTP routes | Deprecated | `chat` tag paths in OpenAPI |
| `/api/v1/contacts/*`, `/api/v1/spaces/*`, `/api/v1/interactions/*` | Non-canonical documentation only | `/im/v3/api/*` per OpenAPI |

## Gateway

- Edge: `sdkwork-im-cloud-gateway` / `sdkwork-im-server`
- Aggregated docs: `/docs`, `/openapi.json`, `/openapi/services/<service-id>.openapi.json`
- Default dev application ingress: `http://127.0.0.1:18079` (`pnpm dev`)

## Regenerate derived SDK inputs

```bash
node sdks/materialize-im-v3-openapi-boundaries.mjs
```

## Legacy note

Older drafts of this file described a proposed `/api/v1` DDD layout. That layout was never the runtime
contract. Use OpenAPI and the ADR above as the only source of truth for path naming.

