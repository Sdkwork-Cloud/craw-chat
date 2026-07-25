# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v2 -->

## SDKWORK Soul

Read `../../../sdkwork-specs/SOUL.md` before executing tasks in this application root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards


<!-- SDKWORK-PROGRESSIVE-LOADING: v1 -->
Resolve this standards root once and use it as the global authority for the current task:

- `../../../sdkwork-specs/README.md`
- `../../../sdkwork-specs/SOUL.md`
- `../../../sdkwork-specs/AGENTS_SPEC.md`

Read only the relevant README task-matrix row or navigation heading, then load the selected authority sections.
<!-- /SDKWORK-PROGRESSIVE-LOADING: v1 -->

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

Read `sdkwork.app.config.json` for H5 identity, SDK/API inventory, release metadata, packaging capability, or app-owned capabilities. Read `etc/` for renderer runtime bindings and the parent deployment reference; concrete public domains remain owned by the IM root `etc/`.

## Local Dictionary Structure

- `AGENTS.md`: local application agent entrypoint and relative SDKWork spec index.
- `CLAUDE.md`, `GEMINI.md`, `CODEX.md`: compatibility shims that point to `AGENTS.md` and must not duplicate rules.
- `sdkwork.app.config.json`: H5 application identity and release metadata.
- `.sdkwork/`: application dictionary for local skills, plugins, manifests, and AI workspace metadata.
- `specs/`: local H5 application/component contracts and narrowing rules.
- `packages/`: H5 React package family for app, console, admin, shared runtime, and Capacitor host modules.
- `src/`: thin H5 application bootstrap, providers, route assembly, and shell entry.
- `config/`: typed runtime config examples for browser, host (Capacitor), server, and container targets.
- `bin/`: cross-platform operational scripts for build, install, run, diagnostics, and mobile host helper commands.
- `docs/`: H5 architecture notes, runbooks, release notes, and local decisions.
- `public/`: browser-served static assets only.
- `scripts/`: app-local build, validation, generation, migration, and development utilities.
- `sdks/`: application-root SDK workspaces and generator inputs per `SDK_WORKSPACE_GENERATION_SPEC.md`.
- `tests/`: application-level integration, runtime, route, package-boundary, host-adapter, config, and release verification tests.
- `package.json`: app-surface command manifest; public command names still follow `PNPM_SCRIPT_SPEC.md`.

## Spec Resolution Order


<!-- SDKWORK-PROGRESSIVE-LOADING: v1 -->
Use dynamic progressive loading for the current task: resolve the selected root and task category before reading broad source context.

1. Read this `AGENTS.md` routing material and classify the owned surface.
2. Read `sdkwork.app.config.json`, module `specs/`, repository/application `specs/`, and `.sdkwork/` only when the task reaches the contract each item governs.
3. Locate only the relevant task-matrix row or navigation heading in `../../../sdkwork-specs/README.md`; do not load the full catalog.
4. Read only the task-specific global spec sections selected by that route, then inspect implementation files.
<!-- /SDKWORK-PROGRESSIVE-LOADING: v1 -->

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
- Frontend/UI code: `../../../sdkwork-specs/FRONTEND_CODE_SPEC.md`, `../../../sdkwork-specs/FRONTEND_SPEC.md`, `../../../sdkwork-specs/UI_ARCHITECTURE_SPEC.md`, and `../../../sdkwork-specs/APP_MOBILE_REACT_UI_SPEC.md`.
- H5 application architecture: `../../../sdkwork-specs/APPLICATION_SPEC.md`, `../../../sdkwork-specs/APP_CLIENT_ARCHITECTURE_ALIGNMENT_SPEC.md`, and `../../../sdkwork-specs/APP_H5_ARCHITECTURE_SPEC.md`.
- Runtime config, SDK wiring, release metadata, and packaging changes must follow the task matrix in `../../../sdkwork-specs/README.md`.

Language-specific specs are on-demand; do not load unrelated specs for unrelated tasks.

## H5 Package Naming

Canonical H5 package naming in this app root per `APP_H5_ARCHITECTURE_SPEC.md`:

- Core runtime: `sdkwork-im-h5-core`
- Commons: `sdkwork-im-h5-commons`
- App shell: `sdkwork-im-h5-shell`
- App capability: `sdkwork-im-h5-<capability>`
- Console core: `sdkwork-im-h5-console-core`
- Console shell: `sdkwork-im-h5-console-shell`
- Console capability: `sdkwork-im-h5-console-<capability>`
- Admin core: `sdkwork-im-h5-admin-core`
- Admin shell: `sdkwork-im-h5-admin-shell`
- Admin capability: `sdkwork-im-h5-admin-<capability>`
- Capacitor host: `sdkwork-im-h5-capacitor`

Historical `sdkwork-clawchat-mobile-*` names were retired and must not be reintroduced.

## Group Knowledgebase Boundary

- The chat surface reads authoritative current membership through the generated IM SDK. Only the current group Owner can initialize or retry a group Knowledgebase; after activation, joined non-Guest Owners, Admins, and Members can open it.
- Browser launch uses only an opaque ticket in the standalone Knowledgebase route fragment; Capacitor launch carries only that ticket through the registered deep link to the independent Knowledgebase host process.
- Feature packages use generated SDKs or approved composed facades only. They must not construct raw Knowledgebase HTTP requests, manual credentials, space identifiers, destination URLs, or embedded Knowledgebase Webviews.

## Code Style Rules

Read `../../../sdkwork-specs/CODE_STYLE_SPEC.md` and `../../../sdkwork-specs/NAMING_SPEC.md` before code changes. Root `src/` must stay thin; business screens, services, i18n, state, and route contributions belong in packages. Feature packages use generated SDK clients or approved composed wrappers, not raw HTTP or manual credential headers.

## Build, Test, and Verification


<!-- SDKWORK-VERIFICATION-ROUTING: v1 -->
Choose only the narrowest verification selected by the changed surface. This is not a default full-suite command list.
Run workspace-wide checks only when the change crosses that boundary.
`bootstrap-*`, `align-*`, `sync-*`, `--write`, and other mutating repair commands are not verification defaults; use them only for an explicitly scoped repair, migration, bootstrap, or alignment task and inspect the resulting diff.
<!-- /SDKWORK-VERIFICATION-ROUTING: v1 -->

Run commands from this application root unless a command explicitly targets the repository root:

- `pnpm dev`
- `pnpm dev:browser:postgres:standalone`
- `pnpm dev:browser:cloud`
- `pnpm build`
- `pnpm lint`
- `pnpm test`

From the repository root, run `pnpm test:sdkwork-workspace-structure-standard`, `pnpm check:pnpm-script-standard`, and `pnpm check:agent-workflow-standard` when changing application-root dictionary, package taxonomy, commands, AGENTS, packaging, or workflow metadata.

## Agent Execution Rules


<!-- SDKWORK-PROGRESSIVE-LOADING: v1 -->
Use dynamic progressive loading for the current task; treat indexes and cross-references as discovery, not as a startup bundle.
Keep `../../../sdkwork-specs/SOUL.md` and the task-selected standards authoritative; expand context only when evidence exposes a new contract boundary.
Language-specific specs are on-demand: only the touched language loads `../../../sdkwork-specs/RUST_CODE_SPEC.md`, `../../../sdkwork-specs/JAVA_CODE_SPEC.md`, `../../../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md`, or `../../../sdkwork-specs/FRONTEND_CODE_SPEC.md`.
Package command standardization loads `../../../sdkwork-specs/PNPM_SCRIPT_SPEC.md` only when the current task changes package commands or scripts; GitHub packaging work loads `../../../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md` only when it reaches that workflow boundary.
Do not infer a recursive workspace scan or a broad validation suite from the presence of a path alone.
<!-- /SDKWORK-PROGRESSIVE-LOADING: v1 -->

Use dynamic progressive loading and the convention dictionary before broad source loading. Do not hand-edit generated SDK output. Do not replace generated SDK integration with raw HTTP. Keep changes scoped to the owning package, surface, or app root. Record exact verification commands and important outputs before reporting completion.

## Task-Specific Standards

API work loads `../../../sdkwork-specs/API_SPEC.md` and its validators. List/search work loads `../../../sdkwork-specs/PAGINATION_SPEC.md` and `check-pagination.mjs`. Source configuration work loads `../../../sdkwork-specs/SOURCE_CONFIG_SPEC.md` and `check-source-config-standard.mjs`. Link these authorities instead of copying their normative bodies into `AGENTS.md`.

## Human Review Rules

Request human review before breaking SDKWork standards, changing public naming, altering security/auth behavior, changing generated SDK ownership, changing production release metadata, or deleting tracked runtime/cache files.
