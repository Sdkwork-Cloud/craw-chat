# sdkwork-im-cloud-gateway

Domain: communication
Capability: chat
Package type: rust-crate
Status: standardizing

This README is the SDKWork module entrypoint for `sdkwork-im-cloud-gateway`. The machine-readable component contract is `specs/component.spec.json`; canonical standards are under `../../../sdkwork-specs/`.

## Public API

- `.`

## Required SDK Surface

- None declared in `specs/component.spec.json`.

## Configuration

Configuration keys, runtime entrypoints, and integration contracts are declared in `specs/component.spec.json`. Shared modules must receive configuration through typed bootstrap or service boundaries rather than reading host-local environment state directly.

### OpenAPI Aggregation

`GET /openapi.json` aggregates live upstream OpenAPI documents from the configured gateway upstreams and merges the gateway discovery endpoints. In standalone or unified-process profiles, some dependency upstreams may intentionally collapse to the same bind as the gateway. The aggregator therefore treats an upstream whose `{baseUrl}/openapi.json` resolves to the gateway's own aggregate endpoint as self-referential and skips it instead of fetching it recursively.

This rule prevents the failure mode where `/openapi.json` recursively calls itself, fans out into repeated upstream fetches, exhausts sockets, and leaves unrelated API requests pending. Direct per-service schema proxy routes under `/openapi/services/{serviceId}.openapi.json` fail closed with `502 Bad Gateway` when the selected service points back to the aggregate endpoint.

Successful aggregate documents are cached in-process for 60 seconds by default and concurrent cache misses are coalesced so only one upstream refresh runs per cache key. Failed upstream refreshes are not cached. Override the TTL with:

```bash
SDKWORK_IM_GATEWAY_OPENAPI_CACHE_TTL_SECS=60
```

Use `cargo test -p sdkwork-im-cloud-gateway --test openapi_index_test -- --nocapture`
to verify self-reference skipping, cache reuse, cache-key stability, concurrent miss
coalescing, and `/healthz` liveness while upstream OpenAPI refreshes are delayed.

### Gateway Protection

Cloud gateway mode applies one per-IP `HybridIpRateLimiter` at the gateway edge. The limiter uses the configured Redis fixed-window backend when available and falls back to the local `DashMap` token bucket when Redis is unavailable and fail-closed mode is not enabled. Probe endpoints are never rate limited:

- `/health`
- `/healthz`
- `/livez`
- `/ready`
- `/readyz`
- `/metrics`

The limiter is not the primary cause of the historical "requests become pending after startup" incident. That incident was caused by recursive OpenAPI aggregation against the gateway's own `/openapi.json`; rate limiting only amplified the socket and request storm once recursion began.

## SaaS/Private/Local Behavior

This component follows the deployment and runtime rules referenced by its `canonicalSpecs` entries. SaaS, private, and local behavior must stay compatible with the relevant SDKWork specs before implementation changes are made.

## Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module. Protected API and SDK access must use the generated SDK or approved service boundary declared in the component contract.

## Extension Points

Extension points are limited to public exports, runtime entrypoints, SDK clients, events, and config keys declared in `specs/component.spec.json`.

## Verification

- `cargo test -p sdkwork-im-cloud-gateway`
- `cargo test -p sdkwork-im-cloud-gateway --test openapi_index_test -- --nocapture`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`. Update that contract before changing public integration behavior.
