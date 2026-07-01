# im-app-context

Domain: communication  
Capability: im  
Package type: rust-crate  
Status: active

Single-source Rust crate for dual-token AppContext resolution, JWT validation, and Axum request middleware used across IM services and gateways.

## Public API (`src/lib.rs`)

- `resolve_app_context`, `resolve_app_context_for_request`, `resolve_handler_app_context`
- `build_dual_token_headers_for_context`, `DualTokenRequestBuilderExt`
- `inject_app_request_context_middleware`
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
