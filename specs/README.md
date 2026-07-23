# SDKWork IM Component Contract

This directory is the local contract index for `sdkwork-im`. The sibling
[`sdkwork-specs`](../../sdkwork-specs/README.md) repository remains the global
authority. Local contracts narrow those standards for IM; they do not copy or
override them.

## Component Identity

| Field | Value |
| --- | --- |
| Component | `sdkwork-im` |
| Domain | `communication` |
| Capability | `chat` |
| Layer role | `runtime-composition` |
| Product surfaces | PC, H5, Flutter mobile, App API, Backend API, Open API, RPC |
| Lifecycle | pre-launch, direct contract correction allowed |

The machine-readable root contract is
[`component.spec.json`](./component.spec.json). Every authored subcomponent has
its own `specs/component.spec.json` with an explicit layer role, public exports,
and composable ports where applicable. Paths in `canonicalSpecs` and permission
composition are resolved relative to the component root.

## Domain Ownership

IM owns communication facts:

- `Conversation -> Message -> Member -> ReadCursor`;
- conversation membership, authorization, invitations, channel/thread semantics;
- visible message history and monotonic per-conversation sequence;
- reactions, pins, read state, presence, routing, realtime delivery, and signaling;
- IM-side Agent assignment intent, dispatch state, reply publication, and opaque
  cross-domain correlation identifiers.

[`sdkwork-agents`](../../sdkwork-agents/specs/README.md) owns Agent execution
facts:

- `Project -> Session -> Turn -> SessionItem -> Interaction`;
- Agent identity and revision, model/provider bindings, inference, tools,
  checkpoints, usage, tasks, and execution audit.

An IM `Message` and an Agents `SessionItem` are different business facts. Neither
is a projection, cache, alias, or replacement for the other. IM may publish an
Agents result as a new visible Message and retain opaque Session/Turn correlation,
but it must not copy the Agents transcript or create another Session/Item model.
There is no dual write between IM and Agents authorities.

The normative dependency and persistence boundary is
[`IM_AGENTS_DEPENDENCY_AND_DATABASE_SPEC.md`](./IM_AGENTS_DEPENDENCY_AND_DATABASE_SPEC.md).
The normalized IM aggregate contract is
[`IM_DOMAIN_AND_PERSISTENCE_SPEC.md`](./IM_DOMAIN_AND_PERSISTENCE_SPEC.md).

## Persistence Authority

PostgreSQL is the production authority. IM owns exactly the tables declared in
the following machine-readable registries:

| Contract | Authority |
| --- | --- |
| [`prefix-registry.json`](../database/contract/prefix-registry.json) | Canonical `im_` prefix and forbidden aliases |
| [`table-registry.json`](../database/contract/table-registry.json) | 60 IM-owned tables, profiles, write owners, and migration provenance |
| [`schema.yaml`](../database/contract/schema.yaml) | Schema registry and migration roots |

Normalized aggregate tables are the only current-state authority. The journal is
immutable audit evidence, and the outbox is a delivery mechanism. Neither is a
read projection. Cache is disposable and cannot be required for correctness.
Ordinary reads and startup must not replay a projector.

IM migrations must not create or write `ai_agent_*`, `studio_*`,
`chat_conversation`, `chat_message`, `ai_coding_session`, or
`im_projection_*` tables. Cross-domain identifiers remain bounded opaque values
without database foreign keys into another product's schema.

## API Authority

Only the following OpenAPI sources define SDKWork IM HTTP operations. Generated
SDK output and generated documentation are consumers, not parallel authorities.

| Surface | Prefix | Operations | Canonical source |
| --- | --- | ---: | --- |
| Open API | `/im/v3/api` | 125 | [`sdkwork-im-im.openapi.yaml`](../apis/open-api/im/sdkwork-im-im.openapi.yaml) |
| App API | `/app/v3/api` | 25 | [`sdkwork-im-app-api.openapi.yaml`](../apis/app-api/communication/sdkwork-im-app-api.openapi.yaml) |
| Backend API | `/backend/v3/api` | 111 | [`sdkwork-im-backend-api.openapi.yaml`](../apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml) |
| Total | - | 261 | [`docs/api-reference.md`](../docs/api-reference.md) |

Requests use verified request context for tenant, organization, and actor scope.
Responses use the SDKWork success/error envelopes. List and search endpoints use
the canonical pagination contract. Consumers integrate through generated SDK
families; handwritten HTTP clients, manual authorization headers, and local SDK
forks are forbidden.

## Composition And Dependencies

The only valid Agent dependency direction is:

```text
sdkwork-im -> sdkwork-agents -> sdkwork-kernel
```

Agents and Kernel do not depend on IM. IM integrates through generated Agents SDK
families, approved public facades, or a declared runtime assembly. It does not
import Agents repositories, SQL, private crates, transport internals, or copied
OpenAPI models.

Frontend composition follows these explicit roles:

| Role | Responsibility |
| --- | --- |
| `frontend-host` | PC/H5/Flutter application or native host boundary |
| `frontend-shell` | Navigation, layout, auth gate, and route assembly |
| `frontend-core` | SDK registry, session/runtime composition, and host contracts |
| `frontend-feature` | One user-facing capability and its state/services/routes |
| `frontend-commons` | Domain-neutral UI primitives and helpers |

Backend crates distinguish `contract`, `backend-domain`, `backend-service`,
`backend-repository`, `backend-provider`, `backend-route`, and runtime composition
roles. SDK families distinguish generated transports from authored facades. The
strict component-port validator rejects missing or invalid declarations.

Shared product dependencies are consumed through their SDK or route-composition
contracts. `sdkwork-utils` is preferred for shared validation, encoding, crypto,
time, and collection helpers; domain rules remain inside the owning IM module.

## Local Contracts

- [`im-app-api-sdk-integration.spec.md`](./im-app-api-sdk-integration.spec.md): API,
  generated SDK, IAM, source-link, and release dependency boundaries.
- [`SDKWORK_APPBASE_IAM_INTEGRATION_SPEC.md`](./SDKWORK_APPBASE_IAM_INTEGRATION_SPEC.md):
  request context and IAM integration.
- [`process-database-pool.spec.json`](./process-database-pool.spec.json): shared
  process-level PostgreSQL pool ownership.
- [`topology.spec.json`](./topology.spec.json): standalone and cloud runtime
  composition.
- [`im-web-ingress-domain.spec.json`](./im-web-ingress-domain.spec.json): PC/H5
  public ingress ownership.
- [`im-member-capability.spec.json`](./im-member-capability.spec.json): member and
  authorization capability boundaries.

## Canonical Standards

- [`DOMAIN_SPEC.md`](../../sdkwork-specs/DOMAIN_SPEC.md)
- [`COMPOSABLE_ARCHITECTURE_SPEC.md`](../../sdkwork-specs/COMPOSABLE_ARCHITECTURE_SPEC.md)
- [`COMPONENT_SPEC.md`](../../sdkwork-specs/COMPONENT_SPEC.md)
- [`DATABASE_SPEC.md`](../../sdkwork-specs/DATABASE_SPEC.md)
- [`SCHEMA_REGISTRY_SPEC.md`](../../sdkwork-specs/SCHEMA_REGISTRY_SPEC.md)
- [`API_SPEC.md`](../../sdkwork-specs/API_SPEC.md)
- [`PAGINATION_SPEC.md`](../../sdkwork-specs/PAGINATION_SPEC.md)
- [`SDK_SPEC.md`](../../sdkwork-specs/SDK_SPEC.md)
- [`APP_SDK_INTEGRATION_SPEC.md`](../../sdkwork-specs/APP_SDK_INTEGRATION_SPEC.md)
- [`DEPENDENCY_MANAGEMENT_SPEC.md`](../../sdkwork-specs/DEPENDENCY_MANAGEMENT_SPEC.md)
- [`DOCUMENTATION_SPEC.md`](../../sdkwork-specs/DOCUMENTATION_SPEC.md)
- [`SECURITY_SPEC.md`](../../sdkwork-specs/SECURITY_SPEC.md)

## Verification

Run the narrow contract gates before broader repository verification:

```powershell
pnpm test:component-spec-consistency
pnpm test:normalized-im-authority-standard
pnpm test:agents-integration-migration
pnpm test:database-naming-standard
pnpm db:validate
pnpm test:apis-authority-standard
pnpm check:api-response-envelope
pnpm check:pagination
pnpm check:app-composition
```

`test:component-spec-consistency` includes strict component layer/port validation
and verifies all canonical standard and permission manifest paths from the real
component root.
