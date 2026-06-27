# sdkwork-im-web-bootstrap

## Purpose

Shared `sdkwork-web-framework` bootstrap for IM-owned HTTP service processes. Wraps Axum
routers with the standard interceptor chain, IAM resolver, and `ImAppContextInjector` domain
projection.

## Owner

SDKWork IM maintainers.

## Related Specs

- `../../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`
- `../../sdkwork-specs/WEB_BACKEND_SPEC.md`

## Verification

```bash
cargo check -p sdkwork-im-web-bootstrap
pnpm test:web-framework-standard
```
