# ADR-20260617 Comms Service Naming Boundaries

Status: accepted and implemented
Owner: sdkwork-im  
Date: 2026-06-17  
Finalized: 2026-07-16
Specs: DOMAIN_SPEC.md, NAMING_SPEC.md, API_SPEC.md, ARCHITECTURE_DECISION_SPEC.md

## Context

The repository previously had duplicate social and chat HTTP scaffolds whose names and paths did not
match the authored OpenAPI. The final architecture keeps one runtime owner for every HTTP surface and
does not retain compatibility crates for code that was never released.

## Decision

The canonical domain is `communication`, abbreviated as `comms` in service identifiers. The public SDK
family and HTTP stem remain `im` and `/im/v3/api`.

| Capability | Canonical service id | OpenAPI tag | Path authority |
| --- | --- | --- | --- |
| Social graph | `comms-social-service` | `social` | `/im/v3/api/social/*` |
| Spaces and organizations | `comms-space-service` | `spaces` | `/im/v3/api/spaces/*` |
| Conversation writes | `comms-conversation-service` | `chat` | `/im/v3/api/chat/*` write operations |
| Conversation reads | `projection-service` | `chat` | `/im/v3/api/chat/*` read operations |

`social-service` owns the social PostgreSQL handlers and standalone assembly. Reactions, pins, threads,
and conversation settings are chat resources owned by conversation/projection services. There is no
separate contacts or interactions runtime service.

The authored OpenAPI under `sdks/sdkwork-im-sdk/openapi/` is the HTTP source of truth. Handwritten API
documentation is an index only and must not introduce `/api/v1/*` or `/im/v3/api/interactions/*` paths.

## Rejected Alternatives

1. A separate `/im/v3/api/contacts/*` service was rejected because it diverges from the locked `social`
   OpenAPI tag.
2. `/im/v3/api/interactions/*` was rejected because it duplicates the existing `chat` tree and breaks SDK
   parity.
3. Retaining deprecated compatibility crates was rejected because the application has not been released
   and no external consumer requires that debt.

## Consequences

- Gateway routing contains only canonical active owners.
- The workspace, component specs, Docker build inventory, and Kubernetes manifests contain no retired
  process scaffolds.
- Social and chat SDK operations continue to be generated from their OpenAPI authorities.
- PC module capability identifiers follow the canonical chat and social boundaries.

## Verification

```bash
cargo check -p social-service -p space-service -p sdkwork-api-im-standalone-gateway -p sdkwork-comms-conversation-service
cargo test -p sdkwork-api-im-standalone-gateway
pnpm run test:deprecated-service-boundary
node sdks/materialize-im-v3-openapi-boundaries.mjs
```

## Supersedes

This decision supersedes informal `/api/v1` path documentation and all duplicate pre-release service
scaffolds.
