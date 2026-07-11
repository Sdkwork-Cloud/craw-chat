# IM PostgreSQL Projection Adapter Specs

This directory owns the machine-readable integration contract for
`im-adapters-postgres-projection`.

- [component.spec.json](./component.spec.json) declares the durable projection
  ports, runtime config keys, canonical SDKWork specs, and verification command.
- Root standards remain authoritative in
  [sdkwork-specs](../../../../sdkwork-specs/README.md).

The crate adapts shared PostgreSQL pools to metadata-snapshot and timeline
projection ports. It does not own projection business rules, HTTP routes, or
generated SDK contracts.
