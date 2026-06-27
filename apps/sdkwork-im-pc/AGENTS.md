# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v2 -->

## SDKWORK Soul

Read `../../../sdkwork-specs/SOUL.md` before executing tasks in this application root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this application root:

- `../../../sdkwork-specs/README.md`
- `../../../sdkwork-specs/SOUL.md`
- `../../../sdkwork-specs/AGENTS_SPEC.md`
- `../../../sdkwork-specs/PNPM_SCRIPT_SPEC.md`
- `../../../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md`
- `../../../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../../../sdkwork-specs/NAMING_SPEC.md`

Do not copy root standard text into this application root. If these relative paths do not resolve, stop and report the broken workspace layout.

## Application Identity

Read `sdkwork.app.config.json` only when changing PC application behavior, runtime config, SDK wiring, release metadata, packaging, or app-owned capabilities.

## Local Dictionary Structure

- `AGENTS.md`: local application agent entrypoint and relative SDKWork spec index.
- `CLAUDE.md`, `GEMINI.md`, `CODEX.md`: compatibility shims that point to `AGENTS.md` and must not duplicate rules.
- `sdkwork.app.config.json`: PC application identity and release metadata.
- `.sdkwork/`: application dictionary for local skills, plugins, manifests, and AI workspace metadata.
- `specs/`: local PC application/component contracts and narrowing rules.
- `packages/`: PC React package family for app, console, admin, shared runtime, and desktop host modules.
- `src/`: thin PC application bootstrap, providers, route assembly, and shell entry.
- `scripts/`: app-local build, validation, generation, migration, and development utilities.
- `package.json`: app-surface command manifest; public command names still follow `PNPM_SCRIPT_SPEC.md`.

## Spec Resolution Order

Use dynamic progressive loading:

1. Read this `AGENTS.md` and any nearer component-level `AGENTS.md`.
2. Read `sdkwork.app.config.json` only when app behavior, runtime config, SDK wiring, release, or package identity is touched.
3. Read local `specs/README.md` and `specs/component.spec.json` only when local contracts are relevant.
4. Read local `.sdkwork/README.md`, `.sdkwork/skills/`, and `.sdkwork/plugins/` only when local agent extensions are relevant.
5. Read `../../../sdkwork-specs/README.md`, then only the task-specific root specs.
6. Inspect implementation files after the relevant standards are clear.

## Required Specs By Task Type

- Agent/workflow changes: `../../../sdkwork-specs/SOUL.md`, `../../../sdkwork-specs/AGENTS_SPEC.md`, `../../../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`, `../../../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md`, and `../../../sdkwork-specs/TEST_SPEC.md`.
- Package script changes: `../../../sdkwork-specs/PNPM_SCRIPT_SPEC.md`, `../../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_SPEC.md`, and `../../../sdkwork-specs/TEST_SPEC.md`.
- Any code change: `../../../sdkwork-specs/CODE_STYLE_SPEC.md`, `../../../sdkwork-specs/NAMING_SPEC.md`, plus only the touched language/framework spec.
- TypeScript/Node code: `../../../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md`.
- Frontend/UI code: `../../../sdkwork-specs/FRONTEND_CODE_SPEC.md`, `../../../sdkwork-specs/FRONTEND_SPEC.md`, `../../../sdkwork-specs/UI_ARCHITECTURE_SPEC.md`, and `../../../sdkwork-specs/APP_PC_REACT_UI_SPEC.md`.
- PC application architecture: `../../../sdkwork-specs/APPLICATION_SPEC.md`, `../../../sdkwork-specs/APP_CLIENT_ARCHITECTURE_ALIGNMENT_SPEC.md`, and `../../../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md`.
- Runtime config, SDK wiring, release metadata, and packaging changes must follow the task matrix in `../../../sdkwork-specs/README.md`.

Language-specific specs are on-demand; do not load unrelated specs for unrelated tasks.

## PC Package Naming

Canonical console/admin package naming in this app root:

- Console surface: `sdkwork-im-console-*`
- Admin surface: `sdkwork-im-admin-*`
- Normalized PC targets per `APP_PC_ARCHITECTURE_SPEC.md`: `sdkwork-im-pc-console-*`, `sdkwork-im-pc-admin-*`
- PC-native capabilities: `sdkwork-im-pc-*`

Historical `sdkwork-clawchat-*` names were retired and must not be reintroduced.

## Code Style Rules

Read `../../../sdkwork-specs/CODE_STYLE_SPEC.md` and `../../../sdkwork-specs/NAMING_SPEC.md` before code changes. Root `src/` must stay thin; business pages, services, i18n, state, and route contributions belong in packages. Feature packages use generated SDK clients or approved composed wrappers, not raw HTTP or manual credential headers.

## Build, Test, and Verification

Run commands from this application root unless a command explicitly targets the repository root:

- `pnpm dev`
- `pnpm dev:desktop`
- `pnpm build`
- `pnpm lint`
- `pnpm test:domain-app-sdk-auth-runtime`
- `pnpm test:notary-app-sdk-integration`
- `pnpm test:drive-app-sdk-integration`
- `pnpm test:knowledgebase-app-sdk-integration`
- `pnpm test:qr-scan-standard`

From the repository root, run `pnpm test:sdkwork-workspace-structure-standard`, `pnpm check:pnpm-script-standard`, and `pnpm check:agent-workflow-standard` when changing application-root dictionary, package taxonomy, commands, AGENTS, packaging, or workflow metadata.

## Agent Execution Rules

Use dynamic progressive loading and the convention dictionary before broad source loading. Do not hand-edit generated SDK output. Do not replace generated SDK integration with raw HTTP. Keep changes scoped to the owning package, surface, or app root. Record exact verification commands and important outputs before reporting completion.

## Human Review Rules

Request human review before breaking SDKWork standards, changing public naming, altering security/auth behavior, changing generated SDK ownership, changing production release metadata, or deleting tracked runtime/cache files.
