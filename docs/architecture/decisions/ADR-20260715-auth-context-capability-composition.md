# ADR-20260715-auth-context-capability-composition

Status: accepted
Owner: sdkwork-im
Date: 2026-07-15
Specs: `IAM_LOGIN_INTEGRATION_SPEC.md`, `APP_COMPOSITION_SPEC.md`, `APP_SDK_INTEGRATION_SPEC.md`, `SECURITY_SPEC.md`

## Context

SDKWork applications compose several independently owned API surfaces. A valid
tenant session is not automatically valid for every capability: group
Knowledgebase spaces are organization-owned, while the canonical bootstrap
session is tenant-scoped with organization id `0`. Allowing a feature to issue
requests before checking that distinction turns a composition error into a
late `401`, `403`, or `404` in the browser.

The server must remain the authorization authority. Client checks are only
request-readiness checks and must use the server-returned `AppContext`, never
hand-built scope headers or token-claim overrides.

## Decision

Every authenticated capability that has a scope stronger than the application
default declares an `authContextRequirements` (or equivalent local capability
contract) with:

- required login scope (`TENANT`, `ORGANIZATION`, or a future explicit scope);
- canonical identifier format and numeric bounds when an organization or
  tenant id is required;
- client preflight behavior (`server-derived-capability-status`);
- server enforcement (`required`);
- a deterministic mismatch behavior (`select-organization`, `reauthenticate`,
  or `capability-unavailable`).

Composition and bootstrap follow this order:

1. IAM establishes a session and returns both tokens plus the trusted
   `AppContext`.
2. The application session bridge stores one context and one TokenManager.
3. The capability service requests a server-derived capability status before
   constructing a mutating operation. It does not parse token claims or invent
   organization headers.
4. If the server reports that the requirement is not met, the service returns a
   typed unavailable state and the UI offers the appropriate
   session/organization action. A normal capability probe must not be modeled as
   a failed mutation.
5. The server repeats validation for every protected operation and remains
   fail-closed. Client state can never grant access.

HTTP failures retain their platform meaning:

- `401`: no valid authenticated session or expired credentials;
- `403`: authenticated context is valid but lacks the required scope or grant;
- `404`: route/resource is absent or intentionally not disclosed;
- `503`: a declared dependency surface is unavailable at runtime.

Gateways and embedded routers must fail readiness when a declared dependency
surface is missing. They must not silently fall back to a neighboring route
prefix, a local mock, or a second auth implementation.

## Alternatives

- Treat organization id `0` as a wildcard. Rejected because it collapses the
  tenant and organization authorization models and can expose cross-organization
  data.
- Add organization headers in each feature. Rejected because the trusted scope
  is token/session-derived and manual headers create split-brain identity.
- Let every capability call the server and map `403` to disabled UI. Rejected
  because it hides composition defects until runtime and creates noisy,
  non-deterministic integrations.

## IM application contract

The IM member capability contract declares `group-knowledgebase` as requiring
an `ORGANIZATION` session and a canonical positive signed-64-bit decimal
organization id. The root tenant-scoped bootstrap session remains valid for
tenant-level IM operations, but it is not advertised as sufficient for this
capability.

The PC Knowledgebase launch service consumes the server's lifecycle/member
status and maps an unavailable scope to a typed UI state; it deliberately does
not infer organization authorization from local token claims. The Rust
conversation service keeps validation at every group Knowledgebase operation
and at the RPC/persistence boundary.

## Consequences

- Tenant-scoped sessions no longer produce browser `403` noise for the group
  Knowledgebase lifecycle probe.
- Organization switching is a session operation, not a feature-specific header
  or query parameter.
- New capability modules must add a context requirement and a contract test
  before exposing their SDK client to feature packages.
- Existing server-side 401/403 behavior remains a security boundary and must be
  covered by backend tests; client tests cover typed status handling and UX
  state only.

## Rollout

1. Phase 1 (this change): declare the group Knowledgebase scope requirement in
   the IM capability contract, preserve server fail-closed validation, and make
   the alignment checker reject contract drift.
2. Phase 2: add a server-derived capability-status projection to conversation
   reads so tenant-scoped sessions receive `scope_required` as normal data
   instead of discovering it through a lifecycle `403`.
3. Phase 3: teach the shared composition resolver to aggregate
   `authContextRequirements` from all dependency modules and validate that the
   application exposes a matching organization/session switch flow.
4. Phase 4: run the composition check across every application root in
   `sdkwork-space` and block release when a declared capability can only fail at
   runtime with an avoidable `401`, `403`, or route-level `404`.

## Verification

The IM checks are:

```bash
pnpm test:group-knowledgebase-launch
pnpm test:agent-group-chat
pnpm exec tsc -p apps/sdkwork-im-pc/tsconfig.app.json --noEmit
```

The workspace-wide rollout should add the same context requirement contract to
each organization-bound capability and make the shared composition verifier
reject a capability that has no preflight and server-enforcement declaration.
