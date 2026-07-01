# sdkwork-im-gateway-assembly

## Purpose

Assembles SDKWork IM gateway route crates into a single application router for cloud and standalone deployments.

## Owner

SDKWork IM maintainers.

## Allowed Content

- Gateway bootstrap and route inventory (`assembly-manifest.json`, `generated/`)
- Public `assemble_application_router` entrypoints

## Forbidden Content

- Product business handlers outside route crate boundaries
- Parallel dependency composition manifests

## Related Specs

- `../../sdkwork-specs/WEB_BACKEND_SPEC.md`
- `../../sdkwork-specs/APPLICATION_GATEWAY_SPEC.md`

## Verification

```bash
cargo check -p sdkwork-im-gateway-assembly
```
