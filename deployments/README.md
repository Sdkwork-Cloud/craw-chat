# Deployments

## Purpose

This directory owns SDKWork IM deployment descriptors, non-secret configuration templates, container
packaging contracts, observability resources, and service-manager examples.

The default development profile is `standalone.development` through `pnpm dev`. Production server and
cloud profiles use PostgreSQL. Desktop SQLite is an application-owned bounded offline cache and is not a
server persistence alternative.

Cloud Kubernetes source templates are under `kubernetes/cloud/`. They are not directly deployable:
release automation must replace every template tag with a build-produced OCI digest through
`scripts/release/materialize-sdkwork-im-kubernetes.mjs`. See `kubernetes/README.md` for the fail-closed
workflow.

## Owner

SDKWork IM maintainers.

## Allowed Content

- Docker and Kubernetes packaging contracts and non-secret templates.
- systemd, launchd, and Windows service descriptors.
- Observability collectors, alert rules, dashboards, and runbooks.
- Deployment documentation and topology handoff examples.

## Forbidden Content

- Runtime secrets, private keys, local configuration overrides, or credentials.
- Databases, mutable service state, logs, caches, or generated release bundles.
- Kubernetes release output that still uses mutable image tags.
- Fabricated image digests, checksums, SBOM, provenance, signatures, or cluster evidence.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/DEPLOYMENT_SPEC.md`
- `../sdkwork-specs/RELEASE_SPEC.md`
- `../sdkwork-specs/CONFIG_SPEC.md`
- `../sdkwork-specs/SUPPLY_CHAIN_SECURITY_SPEC.md`

## Verification

```bash
pnpm run test:commercial-deployment-contract
pnpm run test:kubernetes-release-materializer
pnpm run test:production-security-standard
pnpm run test:k8s-secret-guard
```
