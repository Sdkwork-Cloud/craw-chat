# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v2 -->

## SDKWORK Soul

Read `../sdkwork-specs/SOUL.md` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this root:

- `../sdkwork-specs/README.md`
- `../sdkwork-specs/SOUL.md`
- `../sdkwork-specs/AGENTS_SPEC.md`
- `../sdkwork-specs/PNPM_SCRIPT_SPEC.md`
- `../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md`
- `../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../sdkwork-specs/NAMING_SPEC.md`

Do not copy root standard text into this repository. If these relative paths do not resolve, stop and report the broken workspace layout.

## Application Identity

Read `sdkwork.app.config.json` only when changing IM application behavior, runtime config, SDK wiring, release metadata, packaging, app-owned capabilities, or deployment metadata.

## RTC Dependency Boundary

- `sdkwork-im` owns call signaling (`/im/v3/api/calls/*`) and WebSocket call workflow.
- RTC media/provider runtime comes from sibling `../sdkwork-rtc` only (`sdkwork-communication-rtc-service`, `plugins/rtc-*`, `@sdkwork/rtc-sdk`).
- Do not materialize RTC SDK packages under this repository; `sdks/` must not contain `sdkwork-rtc-sdk`.
- Canonical boundary reference: `../sdkwork-rtc/docs/rtc-im-boundary.md`.

## RPC and Discovery Boundary

- RPC contracts live under `apis/rpc/` with generated `sdkwork-im-rpc-sdk`.
- Phase 1 RPC hosts ship as `*-rpc-bin` services (`session-gateway-rpc-bin`, `sdkwork-comms-conversation-rpc-bin`, `sdkwork-comms-conversation-internal-rpc-bin`) through `sdkwork-rpc-framework`; optional registration uses `SDKWORK_IM_DISCOVERY_ENDPOINT`.
- The `sdkwork-discovery` product control plane remains deferred until Phase 2. Phased adoption plan: `docs/architecture/decisions/ADR-20260619-im-rpc-discovery-integration-deferred.md`.
- Until Phase 2 discovery ships, split-deploy routing continues to use static topology env vars in `configs/topology/` and gateway upstream URLs as the primary fallback.

## Local Dictionary Structure

- `AGENTS.md`: repository agent entrypoint and relative SDKWork spec index.
- `CLAUDE.md`, `GEMINI.md`, `CODEX.md`: compatibility shims that point to `AGENTS.md` and must not duplicate rules.
- `sdkwork.app.config.json`: IM application identity, runtime, release, and capability metadata.
- `sdkwork.workflow.json`: GitHub packaging/release workflow manifest governed by `GITHUB_WORKFLOW_SPEC.md`.
- `.github/workflows/package.yml`: thin reusable workflow call only.
- `.sdkwork/`: local skills, plugins, manifests, and AI workspace metadata.
- `specs/`: local application/component contracts and narrowing rules.
- `apis/`: authored OpenAPI and RPC contract authorities.
- `apps/`: runnable application surfaces such as `apps/sdkwork-im-pc/`.
- `crates/`, `services/`, `adapters/`: Rust contracts, runtime services, and provider integrations.
- `sdks/`: SDK families, OpenAPI authorities, route manifests, and generated SDK artifacts.
- `configs/`, `deployments/`, `scripts/`, `tools/`, `docs/`, `tests/`: config templates, deployment descriptors, thin command entrypoints, validators, documentation, and verification assets.
- `package.json`, `Cargo.toml`: language/build manifests.

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Spec Resolution Order

Use dynamic progressive loading:

1. Read this `AGENTS.md` and any nearer component-level `AGENTS.md`.
2. Read `sdkwork.app.config.json` only when app identity, runtime config, SDK wiring, release, packaging, or owned capabilities are touched.
3. Read local `specs/README.md` and `specs/component.spec.json` only when local contracts are relevant.
4. Read local `.sdkwork/README.md`, `.sdkwork/skills/`, and `.sdkwork/plugins/` only when local agent extensions are relevant.
5. Read `../sdkwork-specs/README.md`, then only the task-specific root specs.
6. Inspect implementation files after the dictionary and relevant specs are clear.

Do not load all specs, generated SDKs, or source trees before the task surface is known.

## Required Specs By Task Type

- Agent/workflow changes: `../sdkwork-specs/SOUL.md`, `../sdkwork-specs/AGENTS_SPEC.md`, `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`, `../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md`, and `../sdkwork-specs/TEST_SPEC.md`.
- Package script changes: `../sdkwork-specs/PNPM_SCRIPT_SPEC.md`, `../sdkwork-specs/APP_RUNTIME_TOPOLOGY_SPEC.md`, `../sdkwork-specs/CONFIG_SPEC.md`, and `../sdkwork-specs/TEST_SPEC.md`.
- Any code change: `../sdkwork-specs/CODE_STYLE_SPEC.md`, `../sdkwork-specs/NAMING_SPEC.md`, plus only the touched language/framework spec.
- Build scripts / dev runners / dependency preparation: `../sdkwork-specs/CODE_STYLE_SPEC.md` §7 (Build Source Integrity), `../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md` §5 (Node Script Resilience), `../sdkwork-specs/PNPM_SCRIPT_SPEC.md` §11 (Clean Command Boundary).
- Rust code: `../sdkwork-specs/RUST_CODE_SPEC.md`; add `../sdkwork-specs/RUST_RPC_SPEC.md` when RPC is touched.
- TypeScript/Node code: `../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md`.
- Frontend/UI code: `../sdkwork-specs/FRONTEND_CODE_SPEC.md`, `../sdkwork-specs/TAILWIND_CSS_INTEGRATION_SPEC.md` when Tailwind CSS is touched, `../sdkwork-specs/FRONTEND_SPEC.md`, `../sdkwork-specs/UI_ARCHITECTURE_SPEC.md`, and exactly one detailed UI architecture spec.
- API/SDK/RPC changes: `../sdkwork-specs/API_SPEC.md`, `../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`, `../sdkwork-specs/WEB_BACKEND_SPEC.md`, `../sdkwork-specs/SDK_SPEC.md`, `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`, `../sdkwork-specs/RPC_SPEC.md`, and `../sdkwork-specs/TEST_SPEC.md` as applicable.
- Runtime/deployment/release changes: `../sdkwork-specs/CONFIG_SPEC.md`, `../sdkwork-specs/ENVIRONMENT_SPEC.md`, `../sdkwork-specs/DEPLOYMENT_SPEC.md`, `../sdkwork-specs/RELEASE_SPEC.md`, `../sdkwork-specs/SUPPLY_CHAIN_SECURITY_SPEC.md`, and `../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md`.
- Security/auth changes: `../sdkwork-specs/IAM_SPEC.md`, `../sdkwork-specs/IAM_LOGIN_INTEGRATION_SPEC.md`, `../sdkwork-specs/SECURITY_SPEC.md`, and `../sdkwork-specs/PRIVACY_SPEC.md`.

Language-specific specs are on-demand; do not load Rust, Java, TypeScript, and frontend specs for unrelated tasks.

## Code Style Rules

Read `../sdkwork-specs/CODE_STYLE_SPEC.md` and `../sdkwork-specs/NAMING_SPEC.md` before code changes. Keep contracts, services, adapters, SDKs, UI packages, and release tooling inside their owning boundaries. Generated SDK output is changed only through source contracts, generator inputs, or approved composed facades.

Build scripts, dev runners, and cross-repository dependency preparation tooling under `scripts/dev/` must follow `CODE_STYLE_SPEC.md` §7 (Build Source Integrity And Self-Healing). Build-critical source files (e.g., sibling `sdkwork-ui/sdkwork-ui-pc-react/build/` contract files) must be verified before invoking builds and self-healed from git when missing. `pnpm clean` must not delete git-tracked build-critical source files.

## Build, Test, and Verification

Use canonical root package scripts from `PNPM_SCRIPT_SPEC.md`:

- `pnpm dev`: default PostgreSQL, `unified-process`, `standalone` browser dev workflow.
- `pnpm dev:browser` and `pnpm dev:desktop`: same PostgreSQL standalone defaults for development orchestration.
- `pnpm dev:browser:sqlite` or `pnpm dev:desktop:sqlite`: explicit SQLite development variants.
- `pnpm dev:server`: server-only development path.
- `pnpm build`, `pnpm test`, `pnpm check`, `pnpm verify`, `pnpm clean`: standard root lifecycle commands.
- `pnpm check:pnpm-script-standard`: validate package script standardization.
- `pnpm check:agent-workflow-standard`: validate AGENTS and GitHub packaging workflow standardization.

Run the narrowest relevant check first, then broader verification when API contracts, SDK generation, RPC, persistence, security, packaging, or cross-package boundaries change.

## Agent Execution Rules

Use dynamic progressive loading and the convention dictionary instead of broad context loading. Do not hand-edit generated SDK output unless the source contract is verified. Do not replace generated SDK integration with raw HTTP. Do not preserve retired commands, copied workflow bodies, or legacy local guidance blocks. Record exact verification commands and important outputs before reporting completion.

## HTTP API Response Envelope

All L2+ `app-api`, `backend-api`, and SDKWork-owned business `open-api` HTTP contracts `MUST` follow `API_SPEC.md` section 4.5, section 14, and section 15:

- **Input:** typed request bodies, section 14.1 list/search/command input, `SdkWorkListQuery`, and `q` for free-text search.
- **Success output:** `SdkWorkApiResponse` with `{ "code": 0, "data": <payload>, "traceId": "<server-uuid>" }`.
- **Error output:** HTTP 4xx/5xx `application/problem+json` (`ProblemDetail`) with numeric `code` and `traceId`.
- Success `code` is numeric `int32`; HTTP 2xx JSON bodies `MUST` use `0` only. REST semantics remain on HTTP status (`201`, `202`, etc.).
- Platform error codes are numeric non-zero values per section 15.3 (`40001`, `40101`, `40401`, …).
- Single resource: `data.item`
- Lists: `data.items` + `data.pageInfo` (`PageInfo.mode` is `offset` or `cursor`)
- Commands: `data.accepted` plus optional `resourceId` / `status`
- Async accept (`202`): `data.operationId`, `data.status`, optional `pollUrl`

Vendor compatibility `open-api` routes that mirror upstream tool or provider wire (for example OpenAI `/v1/*`, Claude Code, Codex) `MAY` opt out only when every exempt operation declares `x-sdkwork-wire-protocol: external` and `x-sdkwork-external-protocol-id` per `API_SPEC.md` section 4.5.2. SDKWork-owned business `open-api` operations `MUST NOT` opt out.

Errors `MUST` use HTTP 4xx/5xx with `application/problem+json` (`ProblemDetail`) including required numeric `code` and `traceId`. Business failures `MUST NOT` use HTTP 2xx with non-zero `code`, string wire codes, `success`, or human `message`.

Forbidden legacy envelopes and fields: `PlusApiResult`, `AppbaseApiResult`, `StoreApiResult`, `SdkWorkResponse`, per-domain `*ApiResult`, wire field `requestId`, bare domain DTOs at the HTTP root, and top-level `{ items, pageInfo, traceId }` without `data`.

Handlers `MUST` serialize success and map errors through `sdkwork-web-framework` response mapping. Generated HTTP SDKs (`--standard-profile sdkwork-v3`) unwrap `data` by default and expose typed numeric `ProblemDetail.code` / `traceId` on errors; use `.raw` when the full envelope is required.

Before completing API contract, SDK generation, or frontend service work, run:

```bash
node <sdkwork-specs>/tools/check-api-response-envelope.mjs --workspace <workspace-root>
```

Authority: `sdkwork-specs/API_SPEC.md` section 4.5 and sections 14–16, `SDK_SPEC.md` section 4.2, `FRONTEND_SPEC.md`, `MIGRATION_SPEC.md` section 4.2.

## Human Review Rules

Request human review before breaking SDKWork standards, changing public naming, altering security/auth behavior, changing database migrations or production deployment config, deleting data/files, changing generated SDK ownership, or modifying release/deployment governance. Surface unresolved spec paths, app identity conflicts, component ownership conflicts, and API authority ambiguity instead of guessing.
