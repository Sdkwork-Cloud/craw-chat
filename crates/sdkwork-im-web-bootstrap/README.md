# sdkwork-im-web-bootstrap

## Purpose

Shared `sdkwork-web-framework` bootstrap for IM-owned HTTP service processes. Wraps Axum
routers with the standard interceptor chain, canonical IAM dual-token resolver, and
`ImAppContextInjector` domain projection. `WebRequestContext` is the sole HTTP identity authority;
the domain injector projects it into IM `AppContext`. App/backend API tenancy is never reparsed or
overridden. The open-api compatibility path may recover IM-only delegated actor fields from a
verified dual token only when tenant, organization, user, session, and app exactly match the
framework principal.

## Owner

SDKWork IM maintainers.

## Related Specs

- `../../../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`
- `../../../sdkwork-specs/WEB_BACKEND_SPEC.md`

## Verification

```bash
cargo check -p sdkwork-im-web-bootstrap
pnpm test:web-framework-standard
```
