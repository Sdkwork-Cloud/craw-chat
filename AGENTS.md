# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v2 -->

## SDKWORK Soul

Read `../sdkwork-specs/SOUL.md` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards


<!-- SDKWORK-PROGRESSIVE-LOADING: v1 -->
Resolve this standards root once and use it as the global authority for the current task:

- `../sdkwork-specs/README.md`
- `../sdkwork-specs/SOUL.md`
- `../sdkwork-specs/AGENTS_SPEC.md`

Read only the relevant README task-matrix row or navigation heading, then load the selected authority sections.
<!-- /SDKWORK-PROGRESSIVE-LOADING: v1 -->

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

Read `sdkwork.app.config.json` for IM identity, registration, SDK/API inventory, release metadata, packaging capability, or app-owned capabilities. Read `etc/` for concrete environment, domain, Base URL, bind, topology, runtime, and deployment values. The app manifest is not runtime configuration authority.

## RTC Dependency Boundary

- `sdkwork-im` owns call signaling (`/im/v3/api/calls/*`) and WebSocket call workflow.
- RTC media/provider runtime comes from sibling `../sdkwork-rtc` only (`sdkwork-communication-rtc-service`, `plugins/rtc-*`, `@sdkwork/rtc-sdk`).
- Do not materialize RTC SDK packages under this repository; `sdks/` must not contain `sdkwork-rtc-sdk`.
- Canonical boundary reference: `../sdkwork-rtc/docs/rtc-im-boundary.md`.

## RPC and Discovery Boundary

- RPC contracts live under `apis/rpc/` with generated `sdkwork-im-rpc-sdk`.
- Phase 1 RPC hosts ship as `*-rpc-bin` services (`session-gateway-rpc-bin`, `sdkwork-comms-conversation-rpc-bin`, `sdkwork-comms-conversation-internal-rpc-bin`) through `sdkwork-rpc-framework`; optional registration uses `SDKWORK_IM_DISCOVERY_ENDPOINT`.
- The `sdkwork-discovery` product control plane remains deferred until Phase 2. Phased adoption plan: `docs/architecture/decisions/ADR-20260619-im-rpc-discovery-integration-deferred.md`.
- Until Phase 2 discovery ships, cloud internal routing continues to use static topology env vars in `etc/topology/` and gateway upstream URLs as the primary fallback.

## Group Knowledgebase Boundary

- IM owns Conversation membership, current-Owner initialization authorization, lifecycle status, and opaque launch-ticket issuance. Knowledgebase owns the one-to-one group-space binding, content, and final ACL enforcement.
- Browser launch carries only the opaque ticket in the standalone Knowledgebase route fragment. Desktop launch carries only that ticket through the registered deep link to the independent Knowledgebase Tauri process; space identifiers, destinations, session tokens, and caller context are never passed by IM.
- Use generated IM SDKs and the generated Knowledgebase RPC SDK or approved composed facades only. Raw HTTP, manual credential headers, and local SDK forks are forbidden; the trusted RPC path requires mTLS, signed caller context, and deployment readiness described in `etc/topology/README.md`.

## Local Dictionary Structure

- `AGENTS.md`: repository agent entrypoint and relative SDKWork spec index.
- `CLAUDE.md`, `GEMINI.md`, `CODEX.md`: compatibility shims that point to `AGENTS.md` and must not duplicate rules.
- `sdkwork.app.config.json`: IM application identity, runtime, release, and capability metadata.
- `etc/`: IM deployment profile index, public origin/API matrices, renderer bootstrap inputs, gateway/service templates, and local DNS examples.
- `sdkwork.workflow.json`: GitHub packaging/release workflow manifest governed by `GITHUB_WORKFLOW_SPEC.md`.
- `.github/workflows/package.yml`: thin reusable workflow call only.
- `.sdkwork/`: local skills, plugins, manifests, and AI workspace metadata.
- `specs/`: local application/component contracts and narrowing rules.
- `apis/`: authored OpenAPI and RPC contract authorities.
- `apps/`: runnable application surfaces such as `apps/sdkwork-im-pc/`.
- `crates/`, `services/`, `adapters/`: Rust contracts, runtime services, and provider integrations.
- `sdks/`: SDK families, OpenAPI authorities, route manifests, and generated SDK artifacts.
- `etc/`, `deployments/`, `scripts/`, `tools/`, `docs/`, `tests/`: source configuration, infrastructure descriptors, thin command entrypoints, validators, documentation, and verification assets.
- `package.json`, `Cargo.toml`: language/build manifests.

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Spec Resolution Order


<!-- SDKWORK-PROGRESSIVE-LOADING: v1 -->
Use dynamic progressive loading for the current task: resolve the selected root and task category before reading broad source context.

1. Read this `AGENTS.md` routing material and classify the owned surface.
2. Read `sdkwork.app.config.json`, module `specs/`, repository/application `specs/`, and `.sdkwork/` only when the task reaches the contract each item governs.
3. Locate only the relevant task-matrix row or navigation heading in `../sdkwork-specs/README.md`; do not load the full catalog.
4. Read only the task-specific global spec sections selected by that route, then inspect implementation files.
<!-- /SDKWORK-PROGRESSIVE-LOADING: v1 -->

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

## Int64 Wire Contract (API_SPEC §13.6)

- OpenAPI `int64` fields and parameters `MUST` be `type: string`, `format: int64`,
  a decimal `pattern` such as `^-?[0-9]+$`, and `x-sdkwork-int64-string: true`.
  `type: integer, format: int64` is a contract violation: generated TypeScript
  SDKs then emit `number`, and browsers silently round ids past
  `Number.MAX_SAFE_INTEGER` (2^53), replaying wrong ids into lookups.
- Rust response DTOs `MUST` serialize `i64` wire fields with
  `#[serde(with = "sdkwork_utils_rust::serde_int64")]` (or `::option`); request
  boundaries parse inbound strings with the same helper.
- Generated TypeScript SDKs keep `int64` as `string`; frontend code `MUST NOT`
  convert ids/snowflake ids/sequence ids to `number` for storage, comparison,
  or submission.
- Verification: `node <sdkwork-specs>/tools/check-api-operation-patterns.mjs --workspace .`

## Code Style Rules

Read `../sdkwork-specs/CODE_STYLE_SPEC.md` and `../sdkwork-specs/NAMING_SPEC.md` before code changes. Keep contracts, services, adapters, SDKs, UI packages, and release tooling inside their owning boundaries. Generated SDK output is changed only through source contracts, generator inputs, or approved composed facades.

Build scripts, dev runners, and cross-repository dependency preparation tooling under `scripts/dev/` must follow `CODE_STYLE_SPEC.md` §7 (Build Source Integrity And Self-Healing). Build-critical source files (e.g., sibling `sdkwork-ui/sdkwork-ui-pc-react/build/` contract files) must be verified before invoking builds and self-healed from git when missing. `pnpm clean` must not delete git-tracked build-critical source files.

## Build, Test, and Verification


<!-- SDKWORK-VERIFICATION-ROUTING: v1 -->
Choose only the narrowest verification selected by the changed surface. This is not a default full-suite command list.
Run workspace-wide checks only when the change crosses that boundary.
`bootstrap-*`, `align-*`, `sync-*`, `--write`, and other mutating repair commands are not verification defaults; use them only for an explicitly scoped repair, migration, bootstrap, or alignment task and inspect the resulting diff.
<!-- /SDKWORK-VERIFICATION-ROUTING: v1 -->

Use canonical root package scripts from `PNPM_SCRIPT_SPEC.md`:

- `pnpm dev`: default PostgreSQL `standalone.development` browser dev workflow.
- `pnpm dev:browser` and `pnpm dev:desktop`: same PostgreSQL standalone defaults for development orchestration.
- `pnpm dev:server`: server-only development path.
- `pnpm build`, `pnpm test`, `pnpm check`, `pnpm verify`, `pnpm clean`: standard root lifecycle commands.
- `pnpm check:pnpm-script-standard`: validate package script standardization.
- `pnpm check:agent-workflow-standard`: validate AGENTS and GitHub packaging workflow standardization.

Run the narrowest relevant check first, then broader verification when API contracts, SDK generation, RPC, persistence, security, packaging, or cross-package boundaries change.

## Agent Execution Rules


<!-- SDKWORK-PROGRESSIVE-LOADING: v1 -->
Use dynamic progressive loading for the current task; treat indexes and cross-references as discovery, not as a startup bundle.
Keep `../sdkwork-specs/SOUL.md` and the task-selected standards authoritative; expand context only when evidence exposes a new contract boundary.
Language-specific specs are on-demand: only the touched language loads `../sdkwork-specs/RUST_CODE_SPEC.md`, `../sdkwork-specs/JAVA_CODE_SPEC.md`, `../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md`, or `../sdkwork-specs/FRONTEND_CODE_SPEC.md`.
Package command standardization loads `../sdkwork-specs/PNPM_SCRIPT_SPEC.md` only when the current task changes package commands or scripts; GitHub packaging work loads `../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md` only when it reaches that workflow boundary.
Do not infer a recursive workspace scan or a broad validation suite from the presence of a path alone.
<!-- /SDKWORK-PROGRESSIVE-LOADING: v1 -->

Use dynamic progressive loading and the convention dictionary instead of broad context loading. Do not hand-edit generated SDK output unless the source contract is verified. Do not replace generated SDK integration with raw HTTP. Do not preserve retired commands, copied workflow bodies, or legacy local guidance blocks. Record exact verification commands and important outputs before reporting completion.

Human Review Rules

Request human review before breaking SDKWork standards, changing public naming, altering security/auth behavior, changing database migrations or production deployment config, deleting data/files, changing generated SDK ownership, or modifying release/deployment governance. Surface unresolved spec paths, app identity conflicts, component ownership conflicts, and API authority ambiguity instead of guessing.

## Task-Specific Standards

API work loads `../sdkwork-specs/API_SPEC.md` and its validators. List/search work loads `../sdkwork-specs/PAGINATION_SPEC.md` and `check-pagination.mjs`. Source configuration work loads `../sdkwork-specs/SOURCE_CONFIG_SPEC.md` and `check-source-config-standard.mjs`. Link these authorities instead of copying their normative bodies into `AGENTS.md`.

## Human Review Rules

Require human review for breaking standards, security exceptions, naming migrations, public contract changes, destructive operations, and changes that affect all repositories or application roots.
