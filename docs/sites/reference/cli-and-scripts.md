# CLI and Scripts

## Development commands

| Command | Purpose |
| --- | --- |
| `pnpm dev` | Default PostgreSQL standalone browser development stack |
| `pnpm dev:browser` | Browser development stack |
| `pnpm dev:desktop` | Desktop development stack |
| `pnpm dev:server` | Server-only development stack |

## Packaged server scripts

| Script | Purpose |
| --- | --- |
| `bin/install-server.*` | Build and install `sdkwork-im-server` |
| `bin/init-config-server.*` | Initialize server config root |
| `bin/start-server.*` | Start packaged server |
| `bin/verify-server.*` | Verify server install health |

## Verification tools

| Script | Purpose |
| --- | --- |
| `bin/chat-cli.*` | CLI HTTP verification against application ingress |
| `bin/chat-window.*` | Multi-terminal chat demo windows |
| `pnpm dev` | Start development stack before CLI smoke |
| `tools/smoke/local_stack_smoke.*` | Minimal stack smoke against `http://127.0.0.1:18079` |
| `npm run docs:verify` | Verify the docs site content contract from `docs/sites` |

## SDK families

CLI smoke validates application ingress HTTP. Integrations should use generated SDK families:
`sdkwork-im-sdk`, `sdkwork-im-app-sdk`, `sdkwork-im-backend-sdk`, and independent `sdkwork-rtc-sdk`.

## SDK materialization and verification

OpenAPI SDK boundaries are materialized from the repository root:

```powershell
node .\sdks\materialize-im-v3-openapi-boundaries.mjs
```

Each OpenAPI SDK family also keeps `bin\prepare-openapi-source.mjs` as the family-local source
preparation entrypoint used before generation.

The IM family keeps route authority and derived generator inputs under the SDK family root:

- `sdks\sdkwork-im-sdk\openapi\sdkwork-im-im.sdkgen.yaml`
- `sdks\sdkwork-im-sdk\openapi\sdkwork-im-im.flutter.sdkgen.yaml`
- `sdks\sdkwork-im-app-sdk\openapi\sdkwork-im-app-api.sdkgen.yaml`
- `sdks\sdkwork-im-backend-sdk\openapi\sdkwork-im-backend-api.sdkgen.yaml`

Each SDK family has one metadata source of truth at family-root `sdk-manifest.json`.
Per-family `.sdkwork-assembly.json` is retired and must not be restored. The manifest records
`manifestPath`, `transportPackageName`, `consumerPackageName`, language workspaces, generated
transport paths, dependencies, and release state.

Run the relevant verifier from the repository root:

```powershell
node .\sdks\sdkwork-im-sdk\bin\verify-sdk.mjs
node .\sdks\sdkwork-im-app-sdk\bin\verify-sdk.mjs
node .\sdks\sdkwork-im-backend-sdk\bin\verify-sdk.mjs
node ..\sdkwork-rtc\sdks\sdkwork-rtc-sdk\bin\verify-sdk.mjs
```

Do not use a separate admin or control SDK family. `/backend/v3/api/control/*` and
`/backend/v3/api/admin/*` are backend modules inside `sdkwork-im-backend-sdk`.

## Retired

Legacy local lifecycle wrappers and compose profiles are removed.
