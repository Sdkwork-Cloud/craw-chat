# sdkwork-api-im-standalone-gateway

## Purpose

Canonical standalone `application.public-ingress` process for SDKWork IM. It owns the single HTTP
listener, process-wide Web Framework infrastructure, IM API assembly, IAM bootstrap, approved
embedded dependency assemblies, realtime plane lifecycle, required worker startup, and graceful
drain for the `standalone` deployment profile.

Business routes and domain rules remain in their owning API assembly, route, service, and repository
crates. This gateway composes those executable entrypoints; it does not become another API authority.

## Owner

SDKWork IM maintainers.

## Runtime Entrypoint

- Binary: `sdkwork-api-im-standalone-gateway`
- Source: `src/main.rs`
- IM route authority: `sdkwork-api-im-assembly`
- Bind key: `SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND`
- Development fallback: `127.0.0.1:18079`

Concrete environment, database, Redis, origin, topology, and dependency configuration comes from the
selected source profile under repository `etc/`. Production deployment must not depend on the
development bind fallback.

## Infrastructure And Readiness

`sdkwork-web-bootstrap::service_router` mounts the infrastructure surface exactly once:

- `/healthz`: process liveness
- `/livez`: liveness alias
- `/readyz`: required dependency readiness
- `/metrics`: process metrics

The gateway readiness check composes:

- canonical IM database and configured Redis checks;
- embedded Agents runtime state readiness;
- the registered IM Agent dispatch worker health signal;
- embedded realtime plane dependency and draining state.

Any required failure returns `503`. The framework returns only the canonical client-safe readiness
detail; dependency URLs, credentials, provider failures, SQL errors, and topology detail are not
returned to the caller. Development may omit an unavailable optional embedded dependency only where
the startup policy explicitly permits it; production bootstrap and readiness remain fail closed.

## Security

- Business routes use the standard Web Framework authentication, authorization, request-context,
  tenant-isolation, input, rate-limit, and response protections from their route manifests.
- Infrastructure probes are public framework routes and expose no credentials or business data.
- CORS uses the shared environment policy; production uses explicit origins.
- Secrets and environment-specific values are injected by the selected deployment environment and
  are never authored in this crate.

## Allowed Content

- listener, tracing, metrics, liveness, readiness, and graceful shutdown composition;
- host-neutral API assembly and approved embedded dependency mounting;
- process-level database lifecycle, realtime plane, and required worker bootstrap;
- runtime adapter wiring and host-boundary tests.

## Forbidden Content

- product business handlers, copied route implementations, or a second OpenAPI authority;
- raw HTTP calls to embedded SDKWork dependencies;
- domain repositories, migration SQL, or a second Conversation/Message/Agent Session model;
- a second application HTTP listener or platform cloud gateway implementation;
- secret values or release evidence fabricated from local fixtures.

## Related Specs

- `../../../sdkwork-specs/API_ASSEMBLY_SPEC.md`
- `../../../sdkwork-specs/APPLICATION_GATEWAY_SPEC.md`
- `../../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_SPEC.md`
- `../../../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`
- `../../../sdkwork-specs/WEB_BACKEND_SPEC.md`
- `../../../sdkwork-specs/HEALTH_CHECK_SPEC.md`
- `../../../sdkwork-specs/SECURITY_SPEC.md`
- `../../../sdkwork-specs/RUST_CODE_SPEC.md`
- `../../../sdkwork-specs/TEST_SPEC.md`

## Verification

```bash
cargo test -p sdkwork-api-im-standalone-gateway
pnpm gateway:validate:standalone
pnpm test:web-framework-standard
```
