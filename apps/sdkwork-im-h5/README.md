# Sdkwork IM H5

Sdkwork IM H5 is the mobile browser instant messaging application for the SDKWork IM
product family. It targets the H5 (mobile web) runtime and ships as a standalone
React + Vite renderer consumed by the SDKWork IM deployment profile.

## Application Identity

- **App key**: `sdkwork-im-h5`
- **Runtime family**: browser (H5)
- **Framework**: react-vite
- **Deployment profiles**: `cloud`, `standalone`
- **Manifest**: [`sdkwork.app.config.json`](sdkwork.app.config.json)
- **Deployment config**: [`etc/sdkwork.deployment.config.json`](etc/sdkwork.deployment.config.json)
- **Browser runtime**: [`etc/browser.runtime.json`](etc/browser.runtime.json)

## Workspace Layout

This app lives under `apps/sdkwork-im-h5/` in the `sdkwork-im` monorepo. It depends
on sibling SDK packages (`@sdkwork/im-app-sdk`, `@sdkwork/im-sdk`,
`@sdkwork/drive-app-sdk`, `@sdkwork/iam-app-sdk`) and shared React foundation
packages (`@sdkwork/auth-pc-react`, `@sdkwork/auth-runtime-pc-react`,
`@sdkwork/appbase-pc-react`, `@sdkwork/ui-pc-react`, `@sdkwork/i18n-pc-react`).

H5-native capability packages live under `packages/sdkwork-im-h5-*`:

- `sdkwork-im-h5-core` - bootstrap, SDK client construction, runtime stores
- `sdkwork-im-h5-commons` - shared UI primitives, locale resources
- `sdkwork-im-h5-chat` - conversation and inbox surfaces
- `sdkwork-im-h5-user` - profile, settings, account pages
- `sdkwork-im-h5-types` - shared TypeScript type contracts
- Feature packages: `sdkwork-im-h5-{ai-image,ai-video,ai-voice,ai-writing,
  approval,attendance,calendar,cloud-drive,community,contacts,course,
  enterprise,hardware,knowledge,meeting,notary,orders,recruitment,report,
  shopping,vip,channels}`

## Run Locally

**Prerequisites**: Node.js, pnpm, workspace root dependencies installed.

From the `sdkwork-im` monorepo root:

```bash
# Install workspace dependencies
pnpm install

# Run H5 dev server (PostgreSQL standalone profile)
pnpm dev:browser:postgres:standalone --filter @sdkwork/im-h5

# Or run directly from this app root
pnpm dev
```

The dev server binds to `127.0.0.1:4178` by default. See
[`.env.example`](.env.example) for public runtime environment variables and
[`config/browser/runtime-env.*.example.json`](config/browser/) for per-profile
example configurations.

## Build

```bash
# From monorepo root
pnpm build:browser --filter @sdkwork/im-h5

# Or from this app root
pnpm build:browser
```

Build artifacts are emitted to `dist/` and packaged as
`web-universal-{cloud,standalone}-browser-zip` per the app manifest.

## Verification

Run SDKWork standard checks from the monorepo root:

```bash
# H5 architecture standard
node scripts/dev/sdkwork-im-h5-architecture-standard.test.mjs

# H5 utils standard
node scripts/dev/sdkwork-im-h5-utils-standard.test.mjs

# App manifest standard
node ../sdkwork-specs/tools/check-app-manifest-standard.mjs --root apps/sdkwork-im-h5

# Source config standard
node ../sdkwork-specs/tools/check-source-config-standard.mjs --root apps/sdkwork-im-h5

# Full workspace verification
pnpm verify
```

## Agent Entry

See [`AGENTS.md`](AGENTS.md) for repository agent guidelines, spec resolution
order, package naming, and verification rules for this H5 application.
