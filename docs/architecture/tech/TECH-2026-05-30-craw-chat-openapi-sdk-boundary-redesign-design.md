> Migrated from `docs/superpowers/specs/2026-05-30-craw-chat-openapi-sdk-boundary-redesign-design.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Sdkwork IM OpenAPI And SDK Boundary Redesign

## Goal

Redesign the Sdkwork IM OpenAPI grouping and SDK family layout so the repository exposes one clear
contract model:

- `sdkwork-im-sdk` remains the only SDK family for IM standardized development APIs under
  `/im/v3/api/*`
- `sdkwork-im-backend-sdk` becomes the only backend and admin SDK family for
  `/backend/v3/api/*`
- `sdkwork-im-app-sdk` becomes the only app-business SDK family for `/app/v3/api/*`
- `sdkwork-rtc-sdk` remains an independent RTC provider-standard SDK family instead of being mixed
  into OpenAPI-generated HTTP SDK families

This redesign must remove the current split where `sdkwork-control-plane-sdk` and
`sdkwork-im-admin-sdk` still exist as independent public SDK families.

## Requested Outcome

The user requirement for this round is explicit:

- `sdkwork-control-plane-sdk` must not remain as an independent SDK family
- `sdkwork-im-admin-sdk` must not remain as an independent SDK family
- all admin-related HTTP APIs must belong to the backend API family
- the backend SDK family must absorb both former control-plane and former admin HTTP surfaces
- APIs other than management-system APIs and IM standardized development APIs must belong to the
  app API family
- `sdkwork-rtc-sdk` must continue as its own independent standard and must not be collapsed into
  app or backend generated SDK groupings

## Current Repository Reality

### Runtime Export Shape

The live local-minimal runtime already exports three OpenAPI discovery surfaces:

- `/im/v3/openapi.json`
- `/app/v3/openapi.json`
- `/backend/v3/openapi.json`

This boundary is already enforced by:

- `services/local-minimal-node/src/node.rs`
- `services/local-minimal-node/tests/openapi_schema_export_test.rs`

So the runtime-level grouping is already close to the desired direction.

### SDK Workspace Drift

The repository currently maintains six SDK families in `sdks/`:

- `sdkwork-im-sdk`
- `sdkwork-im-app-sdk`
- `sdkwork-im-backend-sdk`
- `sdkwork-control-plane-sdk`
- `sdkwork-im-admin-sdk`
- `sdkwork-rtc-sdk`

This is the core drift. The runtime exports three HTTP contracts plus one RTC standard family, but
the workspace layer still models five HTTP-facing families.

### Contract Drift

Current checked-in contract ownership is split across three backend-like families:

- `sdkwork-im-backend-sdk`
  currently documented as `/backend/v3/api/*`, but its family guards still exclude legacy admin
  route fragments and only validate a narrow operator subset
- `sdkwork-control-plane-sdk`
  owns `/backend/v3/api/control/*`
- `sdkwork-im-admin-sdk`
  owns `/backend/v3/api/admin/*`

This makes the backend API boundary ambiguous:

- one runtime discovery endpoint
- three separate checked-in workspace families
- multiple docs pages and verification chains
- consumer ambiguity around which SDK is authoritative for backend-side work

### Documentation And Verification Drift

The docs site, SDK index pages, API reference pages, and validation scripts still teach the older
split:

- control-plane is its own family
- IM admin is its own family
- backend is only partial operator/runtime surface

As a result, repository readers see a different boundary than the actual desired public model.

## Boundary Principles

This redesign freezes the following principles.

### Principle 1: One HTTP Contract Family Per Runtime Surface

Each HTTP contract family must map directly to one exported runtime discovery surface:

- `/im/v3/openapi.json` -> `sdkwork-im-sdk`
- `/app/v3/openapi.json` -> `sdkwork-im-app-sdk`
- `/backend/v3/openapi.json` -> `sdkwork-im-backend-sdk`

No second SDK family may exist for a subset of one of those discovery surfaces.

### Principle 2: Backend Owns All Management And Admin HTTP APIs

All management-facing, operator-facing, control-plane, governance, audit, admin-console, and
backend-side APIs belong to `/backend/v3/api/*` and therefore to `sdkwork-im-backend-sdk`.

This includes:

- `ops`
- `audit`
- `automation` governance that is backend-operated
- `control`
- `admin`

There is no longer a valid independent public SDK family for `/backend/v3/api/control/*` or
`/backend/v3/api/admin/*`.

### Principle 3: IM Standardized Development API Stays Separate

`sdkwork-im-sdk` continues to own the IM standardized development API surface under `/im/v3/api/*`.

This family remains the IM product-standard developer contract and must not absorb:

- app-business APIs
- backend management APIs
- admin-console APIs
- control-plane governance APIs

### Principle 4: App API Owns Non-Management Business APIs

`sdkwork-im-app-sdk` owns app-business APIs under `/app/v3/api/*`.

The app family must absorb business-facing capability that is not part of:

- backend management and admin operations
- IM standardized development contract

This means the app family is the correct home for user-facing or application-facing business APIs
that are not management-system operations.

### Principle 5: RTC SDK Is Not An OpenAPI Family Alias

`sdkwork-rtc-sdk` remains a provider-standard, runtime-standard SDK family.

It must stay independent because it standardizes:

- provider adapters
- runtime surface contracts
- capability negotiation
- signaling/runtime integration rules

It is not a duplicate of app or backend OpenAPI routes, and it must not be renamed or collapsed
into backend or app generated transport workspaces.

## Final Target Family Model

After redesign, the repository-level public SDK family model becomes:

### 1. `sdkwork-im-sdk`

Purpose:

- IM standardized development contract

OpenAPI authority:

- `/im/v3/openapi.json`

Path prefix:

- `/im/v3/api/*`

Examples:

- chat
- realtime
- presence
- client route sessions
- media operations exposed through the IM standard contract
- stream operations exposed through the IM standard contract
- IM-side RTC session and signaling HTTP routes that are part of the standardized IM development
  API

### 2. `sdkwork-im-app-sdk`

Purpose:

- app-business contract for non-management business APIs

OpenAPI authority:

- `/app/v3/openapi.json`

Path prefix:

- `/app/v3/api/*`

Examples:

- portal-facing business APIs
- application-specific business workflows
- tenant/application interaction surfaces that are not management-admin operations

### 3. `sdkwork-im-backend-sdk`

Purpose:

- full backend, operator, governance, and admin contract

OpenAPI authority:

- `/backend/v3/openapi.json`

Path prefix:

- `/backend/v3/api/*`

This family must explicitly include:

- `/backend/v3/api/ops/*`
- `/backend/v3/api/audit/*`
- `/backend/v3/api/automation/*`
- `/backend/v3/api/control/*`
- `/backend/v3/api/admin/*`

### 4. `sdkwork-rtc-sdk`

Purpose:

- RTC provider-standard SDK family

Source of truth:

- RTC assembly metadata, provider catalogs, runtime standards, and provider package contracts

Not an OpenAPI-generated HTTP family.

## Explicit Removals

The following public family identities are removed from the target architecture:

### Remove `sdkwork-control-plane-sdk` As A Public Family

What changes:

- it no longer exists as an independent checked-in SDK family in the repository architecture
- its authority contract is merged into backend authority ownership
- its generated/composed package model is retired as a standalone consumer boundary

What remains conceptually:

- control-plane routes still exist
- control-plane service runtime may still exist
- control-plane route groups still appear in backend docs and generated SDK modules

What disappears:

- separate family-level workspace identity
- separate public package identity
- separate docs landing as a first-class SDK family

### Remove `sdkwork-im-admin-sdk` As A Public Family

What changes:

- it no longer exists as an independent checked-in SDK family in the repository architecture
- its authority contract is merged into backend authority ownership
- its generated/composed package model is retired as a standalone consumer boundary

What remains conceptually:

- admin routes still exist under `/backend/v3/api/admin/*`
- admin docs still exist as backend route groups
- backend SDK modules may still expose admin-focused namespaces

What disappears:

- separate family-level workspace identity
- separate package identity
- separate docs landing as a first-class SDK family

## OpenAPI Grouping Design

### Runtime Discovery Endpoints

The runtime discovery endpoints stay at three HTTP families:

- `/im/v3/openapi.json`
- `/app/v3/openapi.json`
- `/backend/v3/openapi.json`

No new top-level discovery endpoints are introduced.

### IM Contract Grouping

The IM OpenAPI contract remains strictly limited to `/im/v3/api/*`.

It must not include:

- `/app/v3/api/*`
- `/backend/v3/api/*`

### App Contract Grouping

The app OpenAPI contract remains strictly limited to `/app/v3/api/*`.

It must not include:

- `/im/v3/api/*`
- `/backend/v3/api/*`

### Backend Contract Grouping

The backend OpenAPI contract becomes the single checked-in authority for every backend-side route
under `/backend/v3/api/*`.

It must include:

- `ops`
- `audit`
- `automation`
- `control`
- `admin`

It must not exclude `/backend/v3/api/control/*` or `/backend/v3/api/admin/*` at family level.

## SDK Workspace Redesign

### `sdkwork-im-backend-sdk` Becomes The Consolidated Backend Workspace

This workspace becomes the only checked-in backend OpenAPI workspace.

Required outcomes:

- authority OpenAPI includes all backend route groups
- derived sdkgen files include all backend route groups
- verification scripts stop rejecting control/admin path families
- docs stop describing backend as a partial operator-only SDK
- generated packages and composed guidance describe backend as the single management/admin family

### Control And Admin Become Modules, Not Families

Inside `sdkwork-im-backend-sdk`, control-plane and admin remain module-level groupings only.

Examples:

- `backend.control.*`
- `backend.admin.*`
- `backend.ops.*`
- `backend.audit.*`

The exact module naming may vary per language, but the family boundary must not split again.

### `sdkwork-im-app-sdk` Stays Independent

The app workspace remains a standalone OpenAPI-generated family, but its docs must explain more
clearly that it owns non-management business APIs rather than generic "whatever is not IM".

### `sdkwork-im-sdk` Stays Independent

The IM workspace remains the standardized development SDK family for `/im/v3/api/*`.

Its docs must not imply that backend or admin consumers should route through the IM family.

### `sdkwork-rtc-sdk` Stays Independent

The RTC workspace remains a standards-driven SDK family with its own assembly, provider packages,
verification, and documentation.

No backend regrouping may reframe RTC as merely a backend generated package topic.

## Documentation Redesign

### SDK Index

`docs/sites/sdk/index.md` and `sdks/README.md` must be rewritten to present only four public SDK
families:

- `sdkwork-im-sdk`
- `sdkwork-im-app-sdk`
- `sdkwork-im-backend-sdk`
- `sdkwork-rtc-sdk`

The removed families should only appear in migration notes, not as current official choices.

### API Reference Index

`docs/sites/api-reference/index.md` must route readers using this model:

- IM standardized development -> IM API + `sdkwork-im-sdk`
- app-business API -> app API + `sdkwork-im-app-sdk`
- backend/admin/control-plane/management -> backend or platform API + `sdkwork-im-backend-sdk`
- RTC runtime/provider standard -> RTC SDK docs

### Control-Plane Docs

Current control-plane API reference pages remain valuable, but they must be reframed as backend API
subdomains rather than a separate SDK family.

That means:

- route pages may still live under `control-plane/` paths if convenient
- their SDK mapping must point to `sdkwork-im-backend-sdk`
- reader guidance must no longer point to `sdkwork-control-plane-sdk`

### Admin Docs

Current IM admin docs remain valuable, but they must be reframed as backend admin subdomains rather
than a separate SDK family.

That means:

- route pages may still document `/backend/v3/api/admin/*`
- their SDK mapping must point to `sdkwork-im-backend-sdk`
- reader guidance must no longer point to `sdkwork-im-admin-sdk`

## Verification Redesign

### Runtime Export Tests

Existing runtime export tests already validate the three discovery endpoints. They should remain and
continue enforcing:

- IM contract only contains IM paths
- app contract only contains app paths
- backend contract only contains backend paths

### Backend Family Contract Tests

Backend SDK family verification must now assert that the backend family includes representative
paths from:

- `ops`
- `audit`
- `automation`
- `control`
- `admin`

It must stop rejecting control/admin path fragments.

### Remove Legacy Family Verification

Verification scripts, docs tests, and site tests must no longer require:

- `sdkwork-control-plane-sdk`
- `sdkwork-im-admin-sdk`

### Migration Guardrails

Add repository-level guardrails so new docs or scripts do not reintroduce the retired public family
model.

Examples:

- SDK index tests must fail if retired families are listed as current official SDK families
- docs verification must fail if control-plane/admin pages point to retired SDK family pages as the
  authoritative package boundary
- workspace contract tests must fail if backend config continues to exclude `/backend/v3/api/admin`
  or `/backend/v3/api/control`

## Migration Plan

### Phase 1: Freeze The New Boundary In Tests And Documentation

First, make the intended family model explicit in design, docs, and verification:

- update SDK overview and API overview docs
- update verification scripts to treat backend as the consolidated admin/control family
- add or update tests that fail if retired family boundaries are still documented as active

This phase defines truth before moving or deleting workspaces.

### Phase 2: Consolidate Backend Authority Contracts

Move control-plane and IM admin authority ownership into `sdkwork-im-backend-sdk`:

- merge representative backend authority documents
- align family config and required path checks
- ensure backend generated output can cover former control/admin path groups

### Phase 3: Retire Independent Family Entrypoints

Retire `sdkwork-control-plane-sdk` and `sdkwork-im-admin-sdk` as official family entrypoints:

- remove or reduce docs pages to migration notices
- stop listing them in SDK indexes
- stop validating them as official workspaces

Whether the directories are deleted immediately or left temporarily as migration shells is an
implementation detail. Architecturally, they are no longer official families.

### Phase 4: Polish Consumer Guidance

Update all consumer-facing guidance:

- control-plane readers are directed to backend SDK modules
- admin readers are directed to backend SDK modules
- RTC readers are directed to the RTC SDK family
- app readers are directed to app SDK
- IM standard API readers are directed to IM SDK

## Directory Transition Decision

The target architecture does not require preserving deprecated wrapper workspaces.

Recommended implementation decision:

- remove `sdkwork-control-plane-sdk` and `sdkwork-im-admin-sdk` as official workspaces
- if transition shims are needed temporarily, keep them as migration notices only
- do not preserve them as full independently verified SDK families

This prevents long-term drift from returning.

## Risks And Controls

### Risk: Backend Contract Merge Becomes Too Broad Or Vague

Control:

- validate backend family with representative required paths from each backend subgroup
- keep IM and app export-path exclusivity tests intact

### Risk: Documentation Still Teaches Retired Families

Control:

- update site verification to fail on retired family-as-current wording
- rewrite SDK and API index pages first

### Risk: RTC Gets Accidentally Reframed As Backend Or App HTTP Contract

Control:

- explicitly document that RTC is a provider-standard family, not an OpenAPI-generated HTTP family
- preserve independent RTC docs and verification

### Risk: Consumer Code Still Targets Removed Package Names

Control:

- add migration notes mapping old family names to backend modules
- update package guidance everywhere the retired families were previously linked

## Success Criteria

This redesign is complete only when all of the following are true:

- the repository officially documents four public SDK families, not six
- `sdkwork-control-plane-sdk` is no longer treated as an active public SDK family
- `sdkwork-im-admin-sdk` is no longer treated as an active public SDK family
- `sdkwork-im-backend-sdk` is documented and verified as the single backend/admin/control family
- `/backend/v3/api/control/*` and `/backend/v3/api/admin/*` are treated as backend modules, not
  separate SDK families
- `sdkwork-im-sdk` remains the only IM standardized development SDK family
- `sdkwork-im-app-sdk` remains the only app-business OpenAPI SDK family
- `sdkwork-rtc-sdk` remains independently documented and verified
- docs, verification, and runtime contract language all point to the same boundary model

## Decision

The correct repository model is:

- three OpenAPI-generated HTTP contract families:
  `sdkwork-im-sdk`, `sdkwork-im-app-sdk`, and `sdkwork-im-backend-sdk`
- one independent RTC provider-standard family:
  `sdkwork-rtc-sdk`

`sdkwork-control-plane-sdk` and `sdkwork-im-admin-sdk` are retired as independent public SDK
families and their route ownership is absorbed into `sdkwork-im-backend-sdk`.

