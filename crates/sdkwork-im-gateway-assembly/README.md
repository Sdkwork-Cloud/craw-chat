# sdkwork-im-gateway-assembly

## Purpose

Assembles SDKWork IM application-plane route crates into a single router for standalone and cloud gateway hosts.

## Owner

SDKWork IM maintainers.

## Allowed Content

- Gateway bootstrap and route inventory (`assembly-manifest.json`, `src/generated.rs`)
- Public `assemble_application_router`, `ApplicationAssembly`, and `assembly_route_count` entrypoints
- Regression tests that prove route crates merge without duplicate method/path handlers

## Forbidden Content

- Product business handlers outside route crate boundaries
- Parallel dependency composition manifests
- Standalone or cloud listener process code

## Related Specs

- `../../../sdkwork-specs/COMPONENT_SPEC.md`
- `../../../sdkwork-specs/APPLICATION_GATEWAY_SPEC.md`
- `../../../sdkwork-specs/API_SPEC.md`
- `../../../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`
- `../../../sdkwork-specs/WEB_BACKEND_SPEC.md`
- `../../../sdkwork-specs/RUST_CODE_SPEC.md`

## Verification

```bash
cargo test -p sdkwork-im-gateway-assembly --test route_merge_smoke -- --nocapture
pnpm gateway:assembly:validate
```
