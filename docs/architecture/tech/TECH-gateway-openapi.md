> Migrated from `docs/sites/api-reference/gateway-openapi.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Gateway OpenAPI

<p class="api-page-intro">
  The unified <code>web-gateway</code> is the external HTTP entrypoint for the current Sdkwork IM
  server runtime. It publishes the aggregate OpenAPI 3.1 document, a machine-readable service
  schema index, and service-specific schema or rendered-doc views on the same operator-facing
  port.
</p>

<div class="api-overview-grid">
  <div class="api-card">
    <h3>Aggregate Contract</h3>
    <p><code>GET /openapi.json</code> returns the unified gateway contract assembled from live upstream service schemas.</p>
  </div>
  <div class="api-card">
    <h3>Service Index</h3>
    <p><code>GET /openapi/index.json</code> returns SDK contract groups, upstream service inventory, registry-owned route entries, and grouped surface summaries for SDK-generator and operator-tool consumption.</p>
  </div>
  <div class="api-card">
    <h3>Runtime Summary</h3>
    <p><code>GET /openapi/runtime-summary.json</code> returns the registry-backed runtime discovery summary used by startup logs, operator tooling, and machine-readable gateway inspection.</p>
  </div>
  <div class="api-card">
    <h3>Service Schemas</h3>
    <p><code>GET /openapi/services/&lt;service-id&gt;.openapi.json</code> proxies the current OpenAPI 3.1 document for an individual upstream service.</p>
  </div>
  <div class="api-card">
    <h3>Rendered Docs</h3>
    <p><code>GET /docs</code> and <code>GET /docs/services/&lt;service-id&gt;</code> render browser-friendly views of the aggregate and service-level contracts.</p>
  </div>
</div>

## Endpoint Matrix

| Surface | Path | Purpose |
| --- | --- | --- |
| Aggregate schema | `GET /openapi.json` | Unified gateway contract assembled from the configured upstream set |
| Schema index | `GET /openapi/index.json` | Machine-readable SDK contract groups, upstream service schemas, and docs routes |
| Runtime summary | `GET /openapi/runtime-summary.json` | Registry-backed runtime discovery summary aligned with startup output |
| Service schema | `GET /openapi/services/<service-id>.openapi.json` | Direct proxy to one upstream operational service schema |
| Aggregate docs | `GET /docs` | Rendered docs view for the unified gateway contract |
| Service docs | `GET /docs/services/<service-id>` | Rendered docs view for an individual upstream service |

## Operational Notes

- The aggregate contract is assembled from live upstream `GET /openapi.json` responses.
- The aggregate contract also publishes gateway-owned discovery operations such as the schema index, runtime summary, and service-schema proxy routes, so `/docs` shows both proxied API surface and gateway discovery surface together.
- The aggregate contract now defines formal OpenAPI response schemas for the gateway discovery surface. Tooling can bind directly against `GatewayOpenapiIndex`, `GatewayRuntimeSummary`, `GatewayRouteSummary`, `GatewaySurfaceGroupSummary`, and related enum schemas under `components.schemas`.
- In strict startup mode, upstream schema fetch failures are treated as blocking errors for aggregate schema generation.
- The unified `sdkwork-im-cloud-gateway` now proxies registry-owned public websocket upgrade routes on the same external port as the HTTP surface. The current IM open-platform example is `GET /im/v3/api/realtime/ws`, but the gateway behavior is no longer hardcoded to that single path.
- The service index now exposes four complementary collections:
  `sdkContracts` for the three public SDKWork OpenAPI contract groups,
  `services` for per-upstream operational service schema and docs inventory,
  `routes` for registry-owned route patterns across all visibilities,
  and `surfaceGroups` for grouped `service + operationGroup` discovery metadata.
- `sdkContracts[*]` is fixed to `im-api`, `app-api`, and `backend-api`, with schema URLs `/im/v3/openapi.json`, `/app/v3/openapi.json`, and `/backend/v3/openapi.json`. These are the SDK-generation contract groups.
- `services[*]` exposes `serviceId`, `contractKind`, `schemaUrl`, `docsUrl`, `visibility`, `routeCount`, `operationGroups`, `sdkTargets`, `protocols`, and `websocketSubprotocols`. Its `contractKind` is `upstreamOperational` because these entries are gateway-hosted upstream service views, not SDKWork SDK groups.
- `routes[*]` exposes `serviceId`, `operationGroup`, `visibility`, `pathPattern`, `methods`, `protocol`, `sdkTargets`, and `websocketSubprotocols`, giving generators the raw gateway ownership map without requiring them to infer it from service aggregates.
- `surfaceGroups[*]` exposes `serviceId`, `operationGroup`, `visibility`, `routeCount`, `sdkTargets`, `protocols`, and `websocketSubprotocols`, which matches the conceptual slices used by docs, SDK partitioning, and startup summaries.
- The runtime summary exposes the same discovery contract that startup output prints: `baseUrl`, `aggregateOpenapiUrl`, `openapiIndexUrl`, `runtimeSummaryUrl`, `docsUrl`, `sdkContracts`, `serviceContracts`, `publicEndpoints`, and `surfaceGroups`.
- `baseUrl` in the runtime summary is resolved from the incoming request authority. When the gateway sits behind a reverse proxy, `X-Forwarded-Proto` and `X-Forwarded-Host` are honored so operator tooling receives externally correct absolute URLs.
- `routeCount` tracks the number of gateway-owned route patterns assigned to a service in the route registry. It is not a count of OpenAPI operations.
- `operationGroups` mirrors the gateway route-owner grouping used for docs and SDK surface partitioning.
- `sdkTargets` tells tooling whether the owned surface belongs to the IM SDK, App SDK, Backend SDK, or no public SDK target.
- `protocols` makes websocket visibility explicit without forcing tooling to infer transport semantics from path naming conventions.
- `websocketSubprotocols` lists the declared websocket subprotocols owned by a service on the unified gateway contract. It is omitted for HTTP-only services.
- `session-gateway` now publishes separate `sessions`, `presence`, and `realtime` operation groups in the index so generated SDKs and operator-facing docs can preserve the same conceptual boundaries as the live service contract.
- The index intentionally does not expose direct upstream health URLs because the unified deployment contract is the single `sdkwork-im-cloud-gateway` port, not the split internal bind set.
- Startup output on the `sdkwork-im-server` process prints the aggregate schema URL, the schema index URL, the runtime summary URL, a separate `SDK Contracts` section for `im-api`, `app-api`, and `backend-api`, an `Upstream Operational Service Contracts` section for service-level schema/docs URLs, the currently public gateway endpoint patterns derived from the route registry, and a `service + operationGroup` surface summary aligned with the runtime-summary/index metadata.

## Related Pages

<div class="api-link-list">
  <a href="/api-reference/index"><code>API Reference</code> API reference landing page</a>
  <a href="/api-reference/im-api"><code>IM Standard API</code> Standardized IM development surface</a>
  <a href="/api-reference/app-api"><code>App API</code> App-business generated HTTP surface</a>
  <a href="/api-reference/backend-api"><code>Backend API</code> Backend management, operator, control, and admin surface</a>
</div>

