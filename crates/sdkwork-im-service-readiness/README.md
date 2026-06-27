# sdkwork-im-service-readiness

## Purpose

Shared dependency-aware readiness probes for Sdkwork IM HTTP services. Centralizes IAM,
IM database, and Redis health checks for gateway `/readyz` and split-service env probes.

## Owner

SDKWork IM maintainers.

## Related Specs

- `../../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`
- `../../sdkwork-specs/WEB_BACKEND_SPEC.md`
- `../../sdkwork-specs/DEPLOYMENT_SPEC.md`

## Verification

```bash
cargo check -p sdkwork-im-service-readiness
cargo test -p sdkwork-im-cloud-gateway --test http_proxy_test
```
