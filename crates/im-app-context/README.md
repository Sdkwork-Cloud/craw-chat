# im-app-context

Domain: communication  
Capability: im  
Package type: rust-crate  
Status: active

Single-source Rust crate for IM domain `AppContext` projection, internal dual-token utilities,
JWT validation, and trusted orchestration context used across IM services and gateways. HTTP
routers resolve credentials once through `sdkwork-web-framework`; this crate must not install a
parallel request-context middleware.

## Public API (`src/lib.rs`)

- `app_context_from_web_request`, `resolve_web_request_context`, `resolve_app_context_for_request`
- `build_dual_token_headers_for_context`, `DualTokenRequestBuilderExt`
- `allows_header_only_app_context_fallback`, `resolve_web_environment_from_process_env`
- `AppContext`, `AppContextError`, `ResolvedAppContext`, `AppContextSignatureConfig`

Do not add parallel `src/*.rs` module files unless they are wired through `lib.rs` module declarations. The repository enforces this with `pnpm run test:app-context-module-standard`.

## Configuration

See `specs/component.spec.json` and production topology profiles under `configs/topology/`. Production requires tenant-bound JWT signing secrets and forbids the public dev fallback secret.

## Verification

```bash
cargo test -p im-app-context
pnpm run test:app-context-module-standard
pnpm run test:production-security-standard
```
