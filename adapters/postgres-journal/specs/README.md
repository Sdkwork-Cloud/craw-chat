# IM PostgreSQL Journal Adapter Specs

This directory owns the machine-readable integration contract for
`im-adapters-postgres-journal`.

- [component.spec.json](./component.spec.json) declares the durable repository
  ports, typed privileged operation inputs, runtime config keys, canonical SDKWork specs, and
  verification commands.
- Root standards remain authoritative in
  [sdkwork-specs](../../../../sdkwork-specs/README.md).

The crate adapts shared PostgreSQL pools to journal, aggregate, message, outbox,
search, retention, and sequence-allocation ports. It does not own HTTP routes or
generated SDK contracts.

Repository-wide organization isolation and cross-organization operation evidence are owned by
[`specs/organization-isolation.spec.json`](../../../specs/organization-isolation.spec.json); this
module does not copy that inventory.
