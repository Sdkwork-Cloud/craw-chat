# Docs

## Purpose

Repository and application documentation, architecture decisions, runbooks, changelogs, design
notes, and developer guides for Sdkwork IM.

## Owner

SDKWork Chat maintainers.

## Topology v2 (current authority)

| Topic | Entry |
| --- | --- |
| Greenfield plan | [topology-greenfield.md](./topology-greenfield.md) |
| Machine contract | [../specs/topology.spec.json](../specs/topology.spec.json) |
| Profile env files | [../configs/topology/](../configs/topology/) |
| Deployment index | [部署/README.md](./部署/README.md) |
| Compliance pack | [product/compliance/](./product/compliance/) |
| Public docs site | [sites/](./sites/) |

Default development: `pnpm dev` from repository root. Application ingress: `http://127.0.0.1:18079`.

Retired `local-minimal` / `local-default` profiles and lifecycle scripts are documented only in
[topology-greenfield.md](./topology-greenfield.md) and [sites/deployment/](./sites/deployment/).

## Archive indexes

| Directory | Role |
| --- | --- |
| [架构/](./架构/README.md) | 2026-04 architecture design archive |
| [step/](./step/README.md) | Step execution plans |
| [review/](./review/README.md) | Execution cards and retrospectives |
| [architecture/decisions/](./architecture/decisions/README.md) | ADRs |

Archive filenames may retain historical tokens for link stability; active body text uses Topology v2
vocabulary.

## Allowed Content

- Architecture decisions, runbooks, changelogs, design documents, implementation plans, and
  user/developer guides.
- Documentation fixtures and examples that do not contain secrets.

## Forbidden Content

- Runtime data, generated SDK transport output, logs, caches, credentials, secrets, or private
  customer data.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/DOCUMENTATION_SPEC.md`
- `../sdkwork-specs/ARCHITECTURE_DECISION_SPEC.md`

## Verification

```bash
pnpm test:topology-baggage
pnpm test:workflow-commercial-gates
pnpm test:sdkwork-workspace-structure-standard
```

Run documentation-specific checks when available and
`pnpm run test:sdkwork-workspace-structure-standard` after root layout changes.

## Canon Documents

| Document | Path |
| --- | --- |
| Product PRD | [product/prd/PRD.md](product/prd/PRD.md) |
| Technical architecture | [architecture/tech/TECH_ARCHITECTURE.md](architecture/tech/TECH_ARCHITECTURE.md) |

- [docs/product/prd/PRD.md](product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](architecture/tech/TECH_ARCHITECTURE.md)

