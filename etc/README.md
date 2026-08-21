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

## Topology v5

Runtime profile env files live under [topology/](./topology/README.md). Default development uses
`standalone.development` (`pnpm dev`). Process decomposition is selected by the topology profile and
runtime manifests; it is not a public profile-id segment.

## Verification

```bash
pnpm test:topology-baggage
pnpm test:sdkwork-workspace-structure-standard
```

<!-- SDKWORK-DEPLOY-LAYOUT: v1 -->
## Installed Runtime Paths

Authority: `APPLICATION_DEPLOY_LAYOUT_SPEC.md` (`../sdkwork-specs/`).

| Item | Value |
| --- | --- |
| `appId` | `sdkwork-im` |
| `runtimeCode` | `im` |
| Config root | `/etc/sdkwork/im/` |
| Runtime TOML | `/etc/sdkwork/im/config.toml` |
| Secrets | `/etc/sdkwork/im/secrets/` |
| Override | `SDKWORK_IM_CONFIG_FILE` |

Source profiles live under `etc/` (`sdkwork.deployment.config.json` index). Deploy manifest: `deployments/deploy.yaml`. Web data-plane source: `deployments/webserver/` (`SDKWORK_WEBSERVER_SPEC.md` layout v2).

```bash
node ../sdkwork-specs/tools/check-source-config-standard.mjs --root .
node ../sdkwork-specs/tools/check-application-deploy-layout.mjs --root .
node ../sdkwork-specs/tools/check-webserver-toml-standard.mjs --root deployments/webserver
```
<!-- /SDKWORK-DEPLOY-LAYOUT -->


