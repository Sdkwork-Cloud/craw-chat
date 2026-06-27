# IM HTTP route manifests

Route manifests for `sdkwork-routes-*` crates and OpenAPI materialization.

Manifest rows must declare:

- `requestContext: WebRequestContext`
- `apiSurface: open-api | app-api | backend-api`

Materialization is performed by `sdks/materialize-im-v3-openapi-boundaries.mjs`, which also writes
`x-sdkwork-request-context` and `x-sdkwork-api-surface` onto authority OpenAPI operations.

Phase 2 work splits inline service routers into `sdkwork-routes-im-*` crates and checks manifests in
`pnpm test:web-framework-standard`.
