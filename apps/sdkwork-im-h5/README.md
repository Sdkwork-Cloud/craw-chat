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

IM-owned H5 capability packages live under `packages/sdkwork-im-h5-*`:

- `sdkwork-im-h5-core` - bootstrap, SDK client construction, runtime stores
- `sdkwork-im-h5-commons` - shared UI primitives, locale resources
- `sdkwork-im-h5-shell` - capability module registry, route contribution assembly, mobile navigation
- `sdkwork-im-h5-chat` - conversation and inbox surfaces
- `sdkwork-im-h5-user` - legacy, unmounted mixed user surfaces pending owner resolution
- `sdkwork-im-h5-types` - shared TypeScript type contracts
- IM-owned feature packages: `sdkwork-im-h5-{chat,contacts,channels}`
- Fail-closed packages with unresolved ownership: `sdkwork-im-h5-{ai-writing,approval,
  attendance,calendar,enterprise,recruitment,report,user}`

The application composes reusable mobile React modules from their owning sibling repositories:

- Drive: `@sdkwork/drive-mobile-react-drive`
- Image generation: `@sdkwork/image-mobile-react-generation`
- Music generation: `@sdkwork/music-mobile-react-generation`
- Video generation: `@sdkwork/video-mobile-react-generation`
- Voice generation and summary: `@sdkwork/voice-mobile-react-generation`
- Community: `@sdkwork/community-mobile-react-community`
- Course: `@sdkwork/course-mobile-react-courses`
- AIoT hardware: `@sdkwork/aiot-mobile-react-hardware`
- Knowledgebase: `@sdkwork/knowledgebase-mobile-react-knowledge`
- RTC meeting: `@sdkwork/rtc-mobile-react-meeting`
- Notary: `@sdkwork/notary-h5-notary`
- Orders: `@sdkwork/order-mobile-react-orders`
- Shop: `@sdkwork/shop-mobile-react-shopping`
- Membership: `@sdkwork/membership-mobile-react-subscription`

The corresponding `sdkwork-im-h5-{ai-image,ai-music,ai-video,ai-voice,cloud-drive,community,
course,hardware,knowledge,meeting,notary,orders,shopping,vip}` packages are compatibility adapters only. Each keeps the
historic IM package import stable while re-exporting its canonical owner module. Shared generic
mobile primitives come from `@sdkwork/ui-mobile-react`; IM-specific chat UI remains in
`sdkwork-im-h5-commons`.

`@sdkwork/im-h5-shell` is the composition entrypoint. Its default catalog enables only `chat` and
`notary`, matching the existing release UI. Application variants can select known modules through
`moduleIds` or inject fully declared capability modules through `modules`; routes, lifecycle hooks,
and bottom navigation are derived from that selection. Catalog entries without a composed runtime
remain in `CONTRACT_PENDING_IM_H5_MODULES` and are rejected rather than mounted with local fallbacks.
The application root reads the public, typed `VITE_SDKWORK_IM_H5_MODULES` composition key. Omitting it
preserves the release default; for example, `chat,notary,contacts,drive` builds the currently completed
SDK-backed modules into one H5 variant.

Package presence does not mean the feature is mounted or release-ready. The current H5 shell composes
Chat inbox, Conversation, Workspace Notary, and the Notary workflow routes by default. Contacts and
Cloud Drive are optional composed modules backed by injected owner SDK clients. Organization directory,
Agent lifecycle, QR scanning, Chat RTC media UI, legacy Chat operations, AI Image/Video/Writing/Music,
Voice Synthesis, Voice Summary, Calendar, Approval, Attendance, Reports, Meeting, Channels,
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

# IM-owned H5 service contract tests
pnpm --dir apps/sdkwork-im-h5 exec tsx --test \
  packages/sdkwork-im-h5-chat/src/services/ChatService.test.ts \
  packages/sdkwork-im-h5-chat/src/services/chatRealtimeService.test.ts \
  packages/sdkwork-im-h5-contacts/src/services/ContactService.test.ts \
  packages/sdkwork-im-h5-commons/src/ApiClient.test.ts \
  packages/sdkwork-im-h5-channels/src/services/ChannelService.test.ts \
  packages/sdkwork-im-h5-recruitment/src/services/RecruitmentService.test.ts \
  packages/sdkwork-im-h5-user/src/services/UserServices.test.ts

# Canonical migrated module tests
pnpm --dir apps/sdkwork-im-h5 exec tsx --test \
  ../../../sdkwork-notary/apps/sdkwork-notary-h5/packages/sdkwork-notary-h5-notary/src/services/notaryService.test.ts \
  ../../../sdkwork-order/apps/sdkwork-order-common/packages/sdkwork-order-mobile-react-orders/src/services/OrderService.test.ts \
  ../../../sdkwork-aiot/apps/sdkwork-aiot-shared/packages/sdkwork-aiot-mobile-react-hardware/src/services/HardwareService.test.ts \
  ../../../sdkwork-knowledgebase/apps/sdkwork-knowledgebase-common/packages/sdkwork-knowledgebase-mobile-react-knowledge/src/services/KnowledgeBaseService.test.ts \
  ../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/services/ProductService.test.ts \
  ../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/services/CartService.test.ts \
  ../../../sdkwork-community/apps/sdkwork-community-common/packages/sdkwork-community-mobile-react-community/src/services/CommunityService.test.ts \
  ../../../sdkwork-course/apps/sdkwork-course-common/packages/sdkwork-course-mobile-react-courses/src/services/CourseService.test.ts \
  ../../../sdkwork-image/apps/sdkwork-image-common/packages/sdkwork-image-mobile-react-generation/src/services/AIImageService.test.ts \
  ../../../sdkwork-video/apps/sdkwork-video-common/packages/sdkwork-video-mobile-react-generation/src/services/AIVideoService.test.ts \
  ../../../sdkwork-voice/apps/sdkwork-voice-common/packages/sdkwork-voice-mobile-react-generation/src/services/VoiceSummaryService.test.ts

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
