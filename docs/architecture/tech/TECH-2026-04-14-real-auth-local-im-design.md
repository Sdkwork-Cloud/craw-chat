> Migrated from `docs/superpowers/specs/2026-04-14-real-auth-local-im-design.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Real Auth Local IM Design

## Goal

Turn the current local IM stack into a real end-to-end development system:

- real seeded accounts instead of implicit shared-secret auth only
- real login and refresh flows that issue bearer tokens
- chat CLI and GUI that can log in and recover from a stopped local service
- portal `/login` that authenticates against a real backend instead of a mock data source
- local chat, realtime, and RTC flows that keep using the existing public bearer contract after login

## Problem Summary

The current stack has two gaps:

1. chat tools can fail with `Connect` because they assume the local service is already running
2. application login is not real
   - chat surfaces rely on a local HS256 secret and synthesize bearer tokens client-side
   - portal `/login` is backed by a mock data source

This produces a false-ready environment: the transport stack is real, but identity bootstrap is not.

## Chosen Approach

Build a real auth subsystem inside `local-minimal-node` and make every local client consume it.

### Why this approach

- keeps one authority model for chat, realtime, RTC, and portal
- preserves the existing bearer-token verification path in `im-auth-context`
- avoids introducing a second local auth service just for login bootstrap
- lets seeded demo accounts feel realistic without keeping mock login logic in the clients

## Alternatives Considered

### 1. Keep client-side token minting and only add service auto-start

Rejected because it fixes the startup symptom but leaves login fake.

### 2. Add a separate local auth service

Rejected because it adds routing, token, and runtime duplication for a single-node profile.

### 3. Use only portal-side real login and leave chat CLI/GUI on shared-secret auth

Rejected because it would split the local identity story and keep chat/RTC outside the real login path.

## Architecture

### 1. Auth Domain In `local-minimal-node`

Add a dedicated auth module that owns:

- seeded account initialization
- password verification
- refresh-token rotation and revocation
- JWT access-token issuance
- subject/profile bootstrap for portal and chat clients

The auth module will persist state under the runtime dir:

- `.runtime/local-minimal/state/auth-accounts.json`
- `.runtime/local-minimal/state/auth-refresh-sessions.json`

### 2. Seeded Identities

Seed accounts automatically for tenant `100001` during local runtime bootstrap.

Two identity families will exist in the same store:

- IM users
  - used by chat CLI, chat GUI, message send, realtime subscribe, RTC actions
- portal operators
  - used by tenant portal `/login`
  - scoped to operational dashboards rather than end-user chat actions

Seeded accounts must be explicit, visible, and reproducible. The runtime config/docs will publish the default usernames but not store plain-text passwords after initialization.

### 3. Password Storage

Passwords will be stored as salted password hashes, not plain text.

Chosen storage contract:

- PBKDF2-SHA256 with per-account random salt
- persisted algorithm metadata and iteration count
- constant-time verification

This is sufficient for a local commercial-grade dev profile while keeping implementation auditable and deterministic.

### 4. Token Model

Keep two token classes:

- access token
  - HS256 JWT
  - verified by the existing `resolve_public_bearer_auth_context(...)`
  - contains `tenant_id`, subject, `actor_kind`, `sid`, `did`, permissions, issuer, audience, `iat`, `exp`
- refresh token
  - opaque random token
  - stored server-side with expiry, device binding, and revocation metadata
  - rotated on refresh

This keeps the existing service authorization path stable while moving long-lived credentials out of self-contained bearer tokens.

## API Surface

### Auth APIs

Add real public auth endpoints:

- `sdkwork-appbase` login verification
- `sdkwork-appbase` dual-token refresh
- `sdkwork-appbase` IAM context

`sdkwork-appbase` login verification request:

- `tenantId`
- `login`
- `password`
- `clientRouteId` optional
- `sessionId` optional
- `clientKind` optional (`im_user` or `portal_operator`)

`sdkwork-appbase` login verification response:

- `accessToken`
- `refreshToken`
- `expiresAt`
- `user`
- `workspace` when the subject is a portal operator

`sdkwork-appbase` dual-token refresh request:

- `refreshToken`
- `clientRouteId`
- `sessionId`

`sdkwork-appbase` IAM context response:

- normalized subject profile resolved from the bearer token and account store

### Portal Snapshot APIs

Replace the portal mock data source with real HTTP endpoints served by `local-minimal-node`:

- `GET /app/v3/api/portal/home`
- `GET /app/v3/api/portal/access`
- `GET /app/v3/api/portal/workspace`
- `GET /app/v3/api/portal/dashboard`
- `GET /app/v3/api/portal/conversations`
- `GET /app/v3/api/portal/realtime`
- `GET /app/v3/api/portal/media`
- `GET /app/v3/api/portal/automation`
- `GET /app/v3/api/portal/governance`

Public endpoints:

- `home`
- `auth`

Authenticated endpoints:

- `workspace`
- module boards

These endpoints may still shape data into portal-specific view models, but they must be produced by the backend at request time rather than read from frontend mock fixtures.

## Client Changes

### 1. `chat-cli`

Add a real `login` command that returns auth material for user clients.

`chat-cli` will support:

- `login --login <id> --password <secret>`
- outputting access and refresh tokens as JSON
- passing returned bearer tokens into existing commands through `--bearer-token`

The existing shared-secret path stays as an explicit local operator escape hatch, but no longer as the default happy path for seeded accounts.

### 2. Chat GUI / Test Windows

Update `bin/chat-window*.ps1` and `bin/open-chat-test.ps1` so they:

- detect an unavailable local service
- start the local profile automatically when safe
- log in with seeded credentials before sending or polling
- use issued bearer tokens for timeline/send/watch flows

This removes the current failure mode where the GUI opens but the first message send fails with a raw connect error.

### 3. Portal Frontend

Replace the portal mock data source with a real HTTP data source.

Frontend changes:

- `/login` becomes a real form with tenant/login/password fields
- auth store signs in by calling `sdkwork-appbase` login verification
- bootstrap uses the persisted access token against `sdkwork-appbase` IAM context
- board data loads through portal HTTP endpoints
- the default runtime no longer points at `mockPortalDataSource`

## CORS And Local Browser Contract

Because the portal preview runs on a different origin than `local-minimal-node`, the backend must expose a narrow local-browser CORS policy for:

- `http://127.0.0.1:4176`
- `http://localhost:4176`

Allowed headers must include:

- `authorization`
- `content-type`

Allowed methods must include:

- `GET`
- `POST`

## Error Handling

The login surface must fail closed with explicit machine-readable codes:

- `auth_login_invalid`
- `auth_account_disabled`
- `auth_client_kind_invalid`
- `auth_refresh_invalid`
- `auth_refresh_expired`
- `auth_session_revoked`

Client wrappers must translate these into actionable diagnostics instead of leaking raw transport exceptions where recovery is possible.

## Testing Strategy

### Server

Add failing tests first for:

- seeded account initialization
- login success and invalid-password rejection
- refresh rotation and revoked refresh-token rejection
- `sdkwork-appbase` IAM context
- portal snapshot endpoints requiring portal-authenticated bearer tokens

### CLI / GUI

Add failing tests first for:

- `chat-cli login`
- GUI wrapper auto-start when local service is down
- GUI or scripted chat validation using real login rather than direct shared-secret issuance

### Portal

Add failing tests first for:

- login form rendering
- successful real sign-in flow
- bootstrap from persisted access token
- real runtime data source replacing the mock default
- public home/auth and authenticated board fetches

### End-to-End

Verify real local scenarios:

- start local profile
- login seeded IM user
- create conversation
- send message
- watch delivery
- login seeded portal operator
- open portal dashboard
- create RTC session, invite, accept, signal, end

## Non-Goals For This Iteration

- public self-service registration
- external identity providers
- multi-tenant admin provisioning UX
- browser chat product UI beyond the existing local tools and portal

## Success Criteria

The work is complete when all of the following are true:

- a stopped local service no longer produces a dead-end GUI message-send failure
- seeded demo accounts can log in and receive bearer tokens
- chat CLI/GUI use real login for the default flow
- portal `/login` no longer depends on mock auth
- portal data source no longer depends on frontend mock fixtures
- chat, realtime, and RTC flows pass using issued access tokens

