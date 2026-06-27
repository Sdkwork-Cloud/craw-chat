# Server Lifecycle

The formal packaged server contract is centered on the unified `sdkwork-im-cloud-gateway` service and the
canonical binary name `sdkwork-im-server`.

## Canonical Startup Contract

- binary: `sdkwork-im-server`
- package source: `services/sdkwork-im-cloud-gateway`
- startup command: `sdkwork-im-server --config <config-root>/server.yaml`
- default frozen bind in `deployments/templates/server.yaml.example`: `0.0.0.0:18080`

This is the same startup contract used by foreground starts, generated service-manager assets, and
release-bundle planning.

## Primary Command Families

| Script family | Purpose |
| --- | --- |
| `install-server.*` | Create install/config/data/log/run roots and copy canonical config templates |
| `init-config-server.*` | Materialize instance config overlays such as `server.yaml` and `server.env` |
| `init-storage-server.*` | Validate the PostgreSQL storage contract and write a storage report |
| `verify-server.*` | Validate config, storage wiring, and optional release-gate bundle semantic integrity |
| `install-service-server.*` | Render generated `systemd`, `launchd`, and Windows Service wrapper contracts |
| `start-server.*` | Resolve or build the canonical binary, start it, and wait for health |
| `status-server.*` | Report generated service assets, storage report paths, and release-bundle state |
| `stop-server.*` / `restart-server.*` | Control the managed instance lifecycle |

## Install Sequence

1. Run `install-server.*` for the target instance.
2. Run `init-config-server.*` to materialize the instance config set.
3. Configure PostgreSQL credentials and connection settings.
4. Run `init-storage-server.*` to validate the storage contract.
5. Run `verify-server.*` to validate the runtime inputs.
6. Run `install-service-server.*` if you want generated service-manager assets.
7. Run `start-server.*` to launch the unified gateway.

## Config Contract

The frozen server templates currently live under `deployments/templates/`:

- `server.yaml.example`
- `server.env.example`
- `postgresql.yaml.example`

The `server.yaml` contract includes:

- `instance.name`
- `network.bindAddress`
- `publicEndpoints.baseUrl`
- `publicEndpoints.apiBaseUrl`
- `publicEndpoints.websocketBaseUrl`
- `publicEndpoints.docsBaseUrl`
- `runtime.configDir`
- `runtime.dataDir`
- `runtime.logDir`
- `runtime.runDir`
- `storage.postgresqlConfig`

## PostgreSQL Baseline

The packaged server flow freezes PostgreSQL as the storage baseline.

`verify-server.*` and `init-storage-server.*` expect:

- `storage/postgresql.yaml`
- `secrets/postgresql.password`
- `provider: postgresql`
- `passwordFile`
- `migrationMode`

That storage contract is separate from the file-backed defaults used in development topology profiles.
development profile.

## Service-manager Outputs

`install-service-server.*` renders generated assets under `<config-root>/generated/`:

- `sdkwork-im-server.service`
- `com.sdkwork.im.server.plist`
- `SdkworkImServer.xml`
- `install-SdkworkImServer.ps1`
- `uninstall-SdkworkImServer.ps1`

All of them resolve back to the same canonical startup command and config root.

## Unified Gateway Surface

After `start-server.*`, the operator-facing unified gateway surface includes:

- `GET /healthz`
- `GET /readyz`
- `GET /openapi.json`
- `GET /openapi/index.json`
- `GET /openapi/runtime-summary.json`
- `GET /docs`
- `GET /openapi/services/<service-id>.openapi.json`
- `GET /docs/services/<service-id>`

This keeps health, discovery, OpenAPI export, and rendered docs on the same external server port.

## Release-bundle Audit

`verify-server.*`, `status-server.*`, and `plan-release-server.*` can optionally consume the
machine-readable release-gate bundle under
`artifacts/releases/wave-d-2026-04-08/server/release-gate.json`.

That bundle freezes:

- package catalog metadata
- release execution graph
- provenance manifest
- checklist and acceptance-manifest references
- go / no-go gate requirements

`verify-server.*` now validates more than path existence. It also checks that:

- `release-gate.json`, `release-execution.json`, `package-catalog.json`, and `release-provenance.json` agree on bundle identity and canonical startup command
- each platform `acceptance-manifest.json` matches the frozen package IDs, package metadata, service manager, and startup contract
- `release-checklist.md` still references the required `artifact-file-list.txt`, `SHA256SUMS`, `acceptance-manifest.json`, staged artifacts, and go / no-go review flow

When semantic drift is detected, `releaseContracts.contractsValid` becomes `false` even if every referenced file still exists.

`plan-release-server.*` now derives its release-plan surface from the same release-contract helper, so
the selected platform plan and the `contractsValid` flag stay aligned with the semantic audit used by
`verify-server.*` and `status-server.*`.

## Related Pages

- [Deployment Overview](/deployment/index)
- [Production Domain Binding](/deployment/production-domain-binding)
- [CLI and Scripts](/reference/cli-and-scripts)
- [Gateway OpenAPI](/api-reference/gateway-openapi)
