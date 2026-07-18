# Repository Guidelines

Read `../../AGENTS.md` first. This package is the only owner of SDKWork IM H5
Capacitor configuration and generated iOS/Android host projects.

Use `../../../../../sdkwork-specs/APP_H5_ARCHITECTURE_SPEC.md` for the H5 host
boundary, `../../../../../sdkwork-specs/CODE_STYLE_SPEC.md` for build integrity,
and `../../../../../sdkwork-specs/TEST_SPEC.md` for verification.

Do not add business behavior, SDK construction, credentials, or authentication
logic to this package or its generated native projects.

## SDKWORK Soul

Read `../../../../../sdkwork-specs/SOUL.md` before executing tasks. Start with the sections that route the current task; related-spec references are not a startup bundle.

## SDKWORK Standards

<!-- SDKWORK-PROGRESSIVE-LOADING: v1 -->
Resolve this standards root once and use it as the global authority for the current task:

- `../../../../../sdkwork-specs/README.md`
- `../../../../../sdkwork-specs/SOUL.md`
- `../../../../../sdkwork-specs/AGENTS_SPEC.md`

Read only the relevant README task-matrix row or navigation heading, then load the selected authority sections.
<!-- /SDKWORK-PROGRESSIVE-LOADING: v1 -->

## Application Identity

Read `sdkwork.app.config.json` for application identity, registration, SDK/API inventory, release metadata, or app-owned capabilities. Read `etc/` for concrete environment, Base URL, bind, topology, and deployment values; the app manifest is not runtime configuration authority.

## Local Dictionary Structure

Use `AGENTS.md` as the local routing entrypoint; read `.sdkwork/`, `specs/`, `etc/`, and `docs/` only when the current task reaches the workflow, contract, source configuration, or documentation each location governs. Only independently deployable roots own `etc/`.

## Spec Resolution Order

<!-- SDKWORK-PROGRESSIVE-LOADING: v1 -->
Use dynamic progressive loading for the current task: resolve the selected root and task category before reading broad source context.

1. Read this `AGENTS.md` routing material and classify the owned surface.
2. Read `sdkwork.app.config.json`, module `specs/`, repository/application `specs/`, and `.sdkwork/` only when the task reaches the contract each item governs.
3. Locate only the relevant task-matrix row or navigation heading in `../../../../../sdkwork-specs/README.md`; do not load the full catalog.
4. Read only the task-specific global spec sections selected by that route, then inspect implementation files.
<!-- /SDKWORK-PROGRESSIVE-LOADING: v1 -->

## Required Specs By Task Type

Select only the current task authorities from `../../../../../sdkwork-specs/README.md` and `../../../../../sdkwork-specs/AGENTS_SPEC.md`; expand to adjacent specs only when a new contract boundary is reached.

## Code Style Rules

Use `../../../../../sdkwork-specs/CODE_STYLE_SPEC.md` and `../../../../../sdkwork-specs/NAMING_SPEC.md` for authored changes, then load only the language or framework authority touched by the current task.

## Build, Test, and Verification

<!-- SDKWORK-VERIFICATION-ROUTING: v1 -->
Choose only the narrowest verification selected by the changed surface. This is not a default full-suite command list.
Run workspace-wide checks only when the change crosses that boundary.
`bootstrap-*`, `align-*`, `sync-*`, `--write`, and other mutating repair commands are not verification defaults; use them only for an explicitly scoped repair, migration, bootstrap, or alignment task and inspect the resulting diff.
<!-- /SDKWORK-VERIFICATION-ROUTING: v1 -->

## Agent Execution Rules

<!-- SDKWORK-PROGRESSIVE-LOADING: v1 -->
Use dynamic progressive loading for the current task; treat indexes and cross-references as discovery, not as a startup bundle.
Keep `../../../../../sdkwork-specs/SOUL.md` and the task-selected standards authoritative; expand context only when evidence exposes a new contract boundary.
Language-specific specs are on-demand: only the touched language loads `../../../../../sdkwork-specs/RUST_CODE_SPEC.md`, `../../../../../sdkwork-specs/JAVA_CODE_SPEC.md`, `../../../../../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md`, or `../../../../../sdkwork-specs/FRONTEND_CODE_SPEC.md`.
Package command standardization loads `../../../../../sdkwork-specs/PNPM_SCRIPT_SPEC.md` only when the current task changes package commands or scripts; GitHub packaging work loads `../../../../../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md` only when it reaches that workflow boundary.
Do not infer a recursive workspace scan or a broad validation suite from the presence of a path alone.
<!-- /SDKWORK-PROGRESSIVE-LOADING: v1 -->

## Task-Specific Standards

API work loads `../../../../../sdkwork-specs/API_SPEC.md` and its validators. List/search work loads `../../../../../sdkwork-specs/PAGINATION_SPEC.md` and `check-pagination.mjs`. Source configuration work loads `../../../../../sdkwork-specs/SOURCE_CONFIG_SPEC.md` and `check-source-config-standard.mjs`. Link these authorities instead of copying their normative bodies into `AGENTS.md`.

## Human Review Rules

Require human review for breaking standards, security exceptions, naming migrations, public contract changes, destructive operations, and changes that affect all repositories or application roots.