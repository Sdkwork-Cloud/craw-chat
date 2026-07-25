# sdkwork-im-database-host Specs

`component.spec.json` declares the IM database lifecycle host contract. The host accepts only the
process PostgreSQL authority, loads the root `database/` module, and honors manifest lifecycle
flags. It does not keep a SQLite server path or execute undeclared schema repair SQL at startup.

Global `DATABASE_SPEC.md`, `DATABASE_FRAMEWORK_SPEC.md`, and `SECURITY_SPEC.md` remain authoritative.
