# IM PC Client-Local Database

This module owns the SQLite database opened by the IM PC Tauri host. Its role is
`client-local`; it is not an IM service database and it is never a source of
authorization, membership, delivery, message, or conversation truth.

The store contains four bounded client concerns plus one installation identity
record:

- recently fetched conversation cache;
- recently fetched message cache;
- cache refresh cursors;
- a resumable, idempotent pending-send queue with claim, retry, rejection, and
  quarantine state;
- the exact local scope bound to the database file.

It intentionally does not reproduce the authoritative IM schema. Server state
continues to come from the generated IM app SDK and is revalidated by the IM
service on every send.

## Isolation And Security

One SQLite file and WAL set is selected by a SHA-256 fingerprint over the
canonical lifecycle environment, deployment profile, deployment mode, API
origin, tenant, organization, account, principal kind, and principal id. The
same scope is stored in `im_local_installation` and verified on every open.

Message and queue payloads are encrypted with AES-256-GCM. Each scope has a
separate random key stored in the operating-system credential vault. Tokens,
authorization headers, passwords, API keys, private keys, and credential
objects are rejected before persistence. SQLite files stay under the Tauri
application-private data directory and are never placed beside the executable
or in a shared/network directory.

Initialization is serialized across desktop processes by a per-scope operating
system file lock. A credential-vault key is created only while the scoped store
contains no ciphertext. If persisted ciphertext exists but its key is missing,
malformed, or does not decrypt the scope-bound probe, opening fails closed and
the application never replaces the key or continues writing with a new one.

The pre-launch v1-v3 `offline-im-cache.sqlite` schema did not include all scope
dimensions. It is therefore deleted and rebuilt instead of assigning its rows
to a guessed account or origin. A database created by a newer application is
left untouched and fails closed.

## Lifecycle

Runtime schema authority is
`migrations/sqlite/0004_create_im_pc_client_local_store.up.sql`, embedded with
`include_str!`. The baseline is a review and fresh-install parity snapshot; Rust
tests apply both assets independently and compare their schema inventory.
`contract/schema.yaml` owns the semantic table contract, while
`contract/cache-lifecycle.v1.yaml` owns cache and pending-send behavior.

The cache is evicted by age, row count, and encrypted-byte budget. Pending sends
are bounded by row and byte budgets, claimed through expiring leases, and moved
to bounded quarantine when their payload is invalid or their retry/retention
budget is exhausted. Logout and account switch purge fetched cache rows but
retain encrypted pending sends in the original isolated account scope so a
later login to that exact scope can resume them; another account can never
claim them.

## Verification

From this `src-tauri` directory:

```powershell
node ..\..\..\..\..\..\sdkwork-specs\tools\check-database-framework-standard.mjs --root .
cargo fmt --check
cargo test
```

From `apps/sdkwork-im-pc`:

```powershell
node ../../scripts/dev/run-tsx-cli.mjs scripts/desktop-offline-send-queue-contract.test.ts
pnpm lint
```

Canonical standards:

- `../../../../../../sdkwork-specs/DATABASE_SPEC.md`
- `../../../../../../sdkwork-specs/DATABASE_FRAMEWORK_SPEC.md`
- `../../../../../../sdkwork-specs/SECURITY_SPEC.md`
- `../../../../../../sdkwork-specs/PRIVACY_SPEC.md`
