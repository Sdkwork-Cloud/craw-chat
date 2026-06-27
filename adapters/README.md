# Adapters

`adapters/` holds swappable infrastructure backends.

Current constraints:

- `adapters/local-memory` is the default for `standalone.split-services.development` local persistence and interface validation.
- `adapters/journal-redpanda`, `adapters/meta-cockroach`, `adapters/timeline-scylla` remain the production default stack directories.
- All adapters must follow capability and conformance rules in `docs/架构/04-技术选型与可插拔策略.md`.
- Domain models and API contracts must not change when backends are swapped.
