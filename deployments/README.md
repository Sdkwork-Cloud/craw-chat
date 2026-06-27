# Deployments

## Purpose

Deployment descriptors, environment topology, packaging handoff files, infrastructure examples, and
deployment runbooks for Sdkwork IM.

## Topology v2

- Machine contract: [../specs/topology.spec.json](../specs/topology.spec.json)
- Profile env files: [../configs/topology/](../configs/topology/)
- Greenfield plan: [../docs/topology-greenfield.md](../docs/topology-greenfield.md)
- Deployment docs: [../docs/部署/README.md](../docs/部署/README.md)

Default development profile: `standalone.split-services.development` via `pnpm dev`.
Application ingress bind: `127.0.0.1:18079` (from profile env, not hardcoded in services).

Retired compose files and `bin/*-local.*` lifecycle scripts are removed; see topology-greenfield
delete list.

## Owner

SDKWork Chat maintainers.

## Allowed Content

- Docker, Kubernetes, systemd, nginx, release handoff, and topology documentation.
- Docker server image: [docker/sdkwork-im-server.Dockerfile](docker/sdkwork-im-server.Dockerfile)
- Kubernetes reference manifests under `kubernetes/` for cloud split-service profiles.
- Deployment examples and non-secret environment templates.
- Runbooks for SaaS, private, local, and packaged deployment modes.

## Forbidden Content

- Live secrets, private keys, local override files, runtime databases, mutable service state, logs,
  caches, or user-private config.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/DEPLOYMENT_SPEC.md`
- `../sdkwork-specs/RELEASE_SPEC.md`
- `../sdkwork-specs/RUNTIME_DIRECTORY_SPEC.md`

## Verification

Run deployment-specific checks and `pnpm run test:sdkwork-workspace-structure-standard` after root
layout changes.
