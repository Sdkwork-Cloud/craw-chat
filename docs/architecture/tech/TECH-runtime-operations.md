> Migrated from `docs/sites/deployment/runtime-operations.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Runtime Operations

Runtime inspection, repair, backup, preview, and restore flows are tied to the packaged server
install contract and explicit `SDKWORK_IM_RUNTIME_DIR` configuration.

## Development

Use `pnpm dev:server` or `pnpm dev` and inspect logs from the orchestrator output. Application
ingress health:

```bash
curl http://127.0.0.1:18079/healthz
```

## Production

Use [Server Lifecycle](/deployment/server-lifecycle) scripts for install verification, service
management, and PostgreSQL-backed storage configuration.

