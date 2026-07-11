# IM Redis Cache Adapter Specs

This directory owns the machine-readable integration contract for
`im-adapters-redis-cache`.

- [component.spec.json](./component.spec.json) declares the adapter ports,
  runtime config keys, canonical SDKWork specs, and verification command.
- Root standards remain authoritative at
  [sdkwork-specs](../../../sdkwork-specs/README.md).

The crate provides Redis-backed cache and coordination adapters. Durable IM
business facts remain owned by PostgreSQL repositories; Redis failures and
timeouts follow the failure mode of the injected service port.
