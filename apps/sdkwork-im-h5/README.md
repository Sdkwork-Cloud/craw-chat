# Sdkwork IM H5

Sdkwork IM H5 is the mobile browser instant messaging application for the SDKWork IM
product family. It targets the H5 (mobile web) runtime and ships as a standalone
React + Vite renderer consumed by the SDKWork IM deployment profile.
The application is pre-launch and its manifest publication status remains `DRAFT`.

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

Package presence does not mean the feature is mounted or release-ready. The current `ImApp` mounts
Chat inbox, Conversation, Workspace Notary, and the Notary workflow routes. Contacts has a formal
cursor-paged IM SDK service boundary but is not mounted by the root router. Organization directory,
Agent lifecycle, QR scanning, Chat RTC media UI, legacy Chat operations, AI Image/Video/Writing/Music,
Voice Synthesis, Voice Summary, Calendar, Approval, Attendance, Reports, Cloud Drive, Meeting, Channels,
Hardware, Recruitment, local Knowledge CRUD, Shopping, Checkout, Orders, Payments, Vouchers, Refunds,
Fulfillment, Community, Courses, and Enterprise routes are fail-closed until their owner SDK and
permission composition is complete. Legacy User profile, settings, Moments, Characters, Works, voice,
billing, and life-service pages are also fail-closed; browser storage and synthetic records are not
accepted substitutes. The separate legacy User Auth implementation is excluded from release and remains
blocked pending IAM security review; the root app uses the approved appbase IAM runtime instead.
Group Knowledgebase launch remains a separate opaque-ticket integration and is not implemented by the
local Knowledge package.

## Runtime Data Boundaries

- Bootstrap owns the shared IAM `TokenManager` and constructs the IM, Drive, and Notary clients.
- Feature services consume generated SDKs or approved composed facades; raw HTTP and manual auth
  headers are not supported.
- Inbox, Message, Notary, Contact, and social search lists use bounded cursor pagination.
- Realtime Chat consumers share a reference-counted connection and release it after the final lease.
- PostgreSQL is the server persistence authority. SQLite and browser storage are not supported server
  business-state profiles.

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

# H5 service contract tests
pnpm --dir apps/sdkwork-im-h5 exec tsx --test \
  packages/sdkwork-im-h5-chat/src/services/ChatService.test.ts \
  packages/sdkwork-im-h5-chat/src/services/chatRealtimeService.test.ts \
  packages/sdkwork-im-h5-contacts/src/services/ContactService.test.ts \
  packages/sdkwork-im-h5-notary/src/services/notaryService.test.ts \
  packages/sdkwork-im-h5-commons/src/ApiClient.test.ts \
  packages/sdkwork-im-h5-channels/src/services/ChannelService.test.ts \
  packages/sdkwork-im-h5-hardware/src/services/HardwareService.test.ts \
  packages/sdkwork-im-h5-recruitment/src/services/RecruitmentService.test.ts \
  packages/sdkwork-im-h5-knowledge/src/services/KnowledgeBaseService.test.ts \
  packages/sdkwork-im-h5-shopping/src/services/ProductService.test.ts \
  packages/sdkwork-im-h5-shopping/src/services/CartService.test.ts \
  packages/sdkwork-im-h5-orders/src/services/OrderService.test.ts \
  packages/sdkwork-im-h5-community/src/services/CommunityService.test.ts \
  packages/sdkwork-im-h5-course/src/services/CourseService.test.ts \
  packages/sdkwork-im-h5-user/src/services/UserServices.test.ts

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
