# Configs

## Purpose

Source-controlled safe configuration templates, schemas, profile examples, and non-secret defaults
for the repository root.

## Owner

SDKWork IM maintainers.

## Allowed Content

- Safe config templates and schemas.
- Development, test, staging, and production example profiles.
- Non-secret defaults used by repository-level tooling.

## Forbidden Content

- Host-local overrides such as `.env.local`, `.env.postgres`, or `*.local.toml`.
- Browser public runtime config for the PC app root, which belongs under
  `apps/sdkwork-im-pc/config/` when introduced.
- Secrets, tokens, private keys, database credentials, Redis credentials, runtime state, logs, or
  caches.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/CONFIG_SPEC.md`
- `../sdkwork-specs/ENVIRONMENT_SPEC.md`
- `../sdkwork-specs/APP_RUNTIME_TOPOLOGY_SPEC.md`

## Topology v2

Runtime profile env files live under [topology/](./topology/README.md). Default development uses
`standalone.split-services.development` (`pnpm dev`).

## Verification

```bash
pnpm test:topology-baggage
pnpm test:sdkwork-workspace-structure-standard
```
