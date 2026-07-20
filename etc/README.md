# SDKWork IM Source Configuration

## Purpose

`sdkwork.deployment.config.json` is the root deployment profile index and public origin authority.
The `topology/` files are process-consumable profile instances; gateway TOML files configure the
root cloud API gateway composition. Standalone gateway process templates live with their host at
`crates/sdkwork-api-im-standalone-gateway/etc/`.

## Owner

SDKWork IM maintainers.

## Allowed Content

- Safe config templates and schemas.
- Development, test, staging, and production example profiles.
- Non-secret defaults used by repository-level tooling.

## Forbidden Content

- Host-local overrides such as `.env.local`, `.env.postgres`, or `*.local.toml`.
- Browser renderer config owned by PC/H5 deployable roots, which belongs in each app's `etc/`.
- Secrets, tokens, private keys, database credentials, Redis credentials, runtime state, logs, or
  caches.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/SOURCE_CONFIG_SPEC.md`
- `../sdkwork-specs/CONFIG_SPEC.md`
- `../sdkwork-specs/ENVIRONMENT_SPEC.md`
- `../sdkwork-specs/APP_RUNTIME_TOPOLOGY_SPEC.md`

## Topology v4

Runtime profile env files live under [topology/](./topology/README.md). Default development uses
`standalone.development` (`pnpm dev`). Process decomposition is selected by the topology profile and
runtime manifests; it is not a public profile-id segment.

## Verification

```bash
pnpm test:topology-baggage
pnpm test:sdkwork-workspace-structure-standard
```
