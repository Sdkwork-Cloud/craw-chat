# Sdkwork IM PC Playwright E2E

## Scope

Commercial readiness runs four Playwright specs against the production build:

| Spec | Purpose |
| --- | --- |
| `production-shell.spec.ts` | `#root`, document title, no 5xx on static assets |
| `authenticated-shell.spec.ts` | Seeded IAM session keeps users off `/auth/login` |
| `authenticated-chat.spec.ts` | Inbox hydration, message timeline, outbound text send |

Fixtures live under `e2e/fixtures/`:

- `auth.ts` — JWT-shaped IAM session + `AppbaseApiResult` envelope
- `setup-authenticated-page.ts` — session seeding and IAM/IM route mocks
- `im-api.ts` — inbox, timeline, contacts, and post-message mocks

## CI / commercial gate

From repository root:

```bash
node scripts/dev/sdkwork-im-pc-playwright-e2e.test.mjs
```

The wrapper:

1. Requires `apps/sdkwork-im-pc/dist/` from `pnpm build`
2. Starts `dist/server.cjs` on port **4173** (`PLAYWRIGHT_PC_PORT` override supported)
3. Runs `pnpm exec playwright test` with `PLAYWRIGHT_BASE_URL=http://127.0.0.1:4173`

Port 4173 avoids collisions with other local services on `3000`.

## Local development

```bash
cd apps/sdkwork-im-pc
pnpm build
PLAYWRIGHT_BASE_URL=http://127.0.0.1:4173 PORT=4173 node dist/server.cjs
pnpm exec playwright test
```

## Staging boundary

CI uses **mock-based** IAM/IM responses so the gate stays deterministic without a live split-service stack.

For staging-backed runs against `configs/topology/cloud.split-services.staging.env`:

1. Deploy or point to a reachable staging ingress
2. Export `PLAYWRIGHT_BASE_URL` to that ingress
3. Provide a real dual-token session through approved IAM bootstrap (do not commit secrets)
4. Run only the specs that do not depend on route mocks, or replace mocks with staging fixtures in a dedicated branch/profile

Staging Playwright is an operator workflow today; it is not part of the default commercial-readiness gate.
