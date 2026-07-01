# local-disk

Local filesystem adapter for development and sandbox persistence.

## Allowed uses

- Social service dev/sandbox journal snapshots when PostgreSQL is not configured.
- Admin sandbox fixtures under `sdkwork-api-product-runtime`.
- Local-only integration tests.

## Production boundary

- **Not** a production file-upload or object-storage authority.
- All application uploads must go through `sdkwork-drive` per `DRIVE_SPEC.md`.
- Do not route chat, community, or media client uploads through this adapter.

## Verification

```bash
cargo test -p im-adapters-local-disk
```
