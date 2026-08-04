# Adapters

`adapters/` holds swappable infrastructure backends.

Current constraints:

- `adapters/local-memory` is the default for `standalone.development` local persistence and interface validation.
- **PostgreSQL is the only currently implemented production persistence path** (`adapters/postgres-journal`, `adapters/social-postgres`, `adapters/postgres-realtime`, `adapters/postgres-rtc-state`). `adapters/journal-redpanda` and `adapters/meta-cockroach` are **planned, not implemented** placeholders — they are not production defaults and must not be claimed as available until a real implementation and conformance tests exist.
- All adapters must follow capability and conformance rules in `docs/鏋舵瀯/04-鎶€鏈€夊瀷涓庡彲鎻掓嫈绛栫暐.md`.
- Domain models and API contracts must not change when backends are swapped.
