# sdkwork-im-database-pool Specs

`component.spec.json` declares the process-wide IM database pool contract. Every server profile
installs one PostgreSQL lifecycle pool and one bounded compatibility driver pool for the same
redacted identity. Missing configuration, SQLite, and identity mismatch fail before traffic.

The PC client-local cache owns its separate SQLite adapter and contract outside this crate.
