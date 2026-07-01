# Architecture Decision Records

This directory holds architecture decision records (ADRs) for `sdkwork-im`, following
[`sdkwork-specs/ARCHITECTURE_DECISION_SPEC.md`](../../../sdkwork-specs/ARCHITECTURE_DECISION_SPEC.md).

Topology v2 authority for runtime profiles and ingress: [../../topology-greenfield.md](../../topology-greenfield.md).

Superseded ADRs may retain historical crate or profile names in migration tables; active
implementation uses `sdkwork-im-server`, topology profile ids under `configs/topology/`, and
application ingress `127.0.0.1:18079` for development.

## When an ADR is required

An ADR is required when a change affects any of the following (per
`ARCHITECTURE_DECISION_SPEC.md` §1):

- application root, package family, component boundary, domain boundary, route authority,
  SDK family, data ownership boundary, or runtime topology;
- public API, RPC, SDK, database, security, privacy, deployment, release, or cross-client
  architecture behavior;
- a new framework, platform, storage provider, generated code authority, host adapter
  category, or shared runtime dependency;
- a compatibility exception, migration strategy, release strategy, or quality gate exception;
- a meaningful tradeoff where future readers need to understand why one approach was chosen.

For simple additive work with no long-lived architecture consequence, a short decision
section inside the requirement or implementation plan is enough.

## Record shape

File name: `ADR-YYYYMMDD-<short-title>.md` (kebab-case title).

```md
# ADR-YYYYMMDD-<short-title>

Status: proposed | accepted | superseded | deprecated
Requirement: REQ-YYYY-NNNN        <!-- optional, when a requirement exists -->
Owner: team-or-person
Date: YYYY-MM-DD
Specs: <the SDKWork specs that own the technical rules cited>

## Context
## Decision
## Alternatives
## Consequences
## Verification
## Supersedes / Superseded By
```

## Rules

- ADRs must not bypass root `sdkwork-specs`; exceptions follow `GOVERNANCE_SPEC.md`.
- Each `Decision` section must cite the more specific SDKWork spec that owns the rule.
- Architecture decisions that affect multiple repositories, generated SDK ownership,
  **public naming**, security posture, data ownership, or release compatibility require
  human review before broad implementation.
- Superseded records remain in history and point to their replacement.

## Index

| ADR | Title | Status |
| --- | --- | --- |
| [ADR-20260615-crate-naming-alignment](./ADR-20260615-crate-naming-alignment.md) | Crate naming alignment (`sdkwork-im-*`/`im-*` → `sdkwork-im-*`), batched migration | superseded |
| [ADR-20260615-craw-chat-to-sdkwork-im-rebrand](./ADR-20260615-craw-chat-to-sdkwork-im-rebrand.md) | Craw Chat to SDKWork IM rebrand, product token `im`, naming authority | accepted |
| [ADR-20260617-comms-service-naming-boundaries](./ADR-20260617-comms-service-naming-boundaries.md) | Communication service naming boundaries (`sdkwork-comms-*`) | accepted |
| [ADR-20260619-im-rpc-discovery-integration-deferred](./ADR-20260619-im-rpc-discovery-integration-deferred.md) | IM RPC discovery integration deferred to Phase 2 | accepted |
