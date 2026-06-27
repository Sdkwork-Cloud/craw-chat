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

## SDK families

CLI smoke validates application ingress HTTP. Integrations should use generated SDK families:
`sdkwork-im-sdk`, `sdkwork-im-app-sdk`, `sdkwork-im-backend-sdk`, and independent `sdkwork-rtc-sdk`.

## Retired

Legacy local lifecycle wrappers and compose profiles are removed.
