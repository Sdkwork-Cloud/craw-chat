# Adapters

`adapters/` holds swappable infrastructure backends.

Current constraints:

- `adapters/local-memory` is the default for `standalone.development` local persistence and interface validation.
- `adapters/journal-redpanda`, `adapters/meta-cockroach`, `adapters/timeline-scylla` remain the production default stack directories.
- All adapters must follow capability and conformance rules in `docs/鏋舵瀯/04-鎶€鏈€夊瀷涓庡彲鎻掓嫈绛栫暐.md`.
- Domain models and API contracts must not change when backends are swapped.
