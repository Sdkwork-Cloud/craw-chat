# SDKWork Specs Structure Alignment Design

## Context

`sdkwork-im` is a mixed SDKWork application repository with Rust services, generated SDK families,
release tooling, and the PC React/Tauri application root at `apps/sdkwork-im-pc`. The repository
now follows the canonical sibling standards under `../sdkwork-specs`, especially
`SDKWORK_WORKSPACE_SPEC.md`, `APPLICATION_SPEC.md`, `APP_PC_ARCHITECTURE_SPEC.md`,
`DEPENDENCY_MANAGEMENT_SPEC.md`, and `TEST_SPEC.md`.

The current tree already has local SDKWORK entrypoints at the repository root, but the new
structure exposes gaps:

- Standard top-level capability directories such as `apis/`, `jobs/`, `plugins/`, `examples/`,
  `configs/`, and `tests/` are missing.
- The repository root has `package.json` with `workspace:*` dependencies but no root
  `pnpm-workspace.yaml`.
- `apps/sdkwork-im-pc` is an independent application root with `sdkwork.app.config.json`, but it
  does not yet have its own `AGENTS.md` or source-controlled `.sdkwork/` dictionary.
- PC console and admin package names still use historical `sdkwork-clawchat-console-*` and
  `sdkwork-clawchat-admin-*` names rather than the target `sdkwork-clawchat-pc-console-*` and
  `sdkwork-clawchat-pc-admin-*` families.
- `.sdkwork/dart/pub-cache` had tracked runtime/cache state under the repository metadata directory.

The workspace also contains existing uncommitted source changes. This alignment must not revert,
rewrite, or reformat unrelated work.

## Decision

Use a compatible standards-alignment pass rather than a breaking rename migration.

This pass will make the SDKWork structure executable and discoverable while preserving existing
package paths and developer commands. It will add standards entrypoints, root layout placeholders,
root pnpm workspace authority, migration documentation, and a static verification command.

## Scope

In scope:

- Add missing standard top-level directory placeholders and README files.
- Add `apps/sdkwork-im-pc` application-root `AGENTS.md`, compatibility shims, and `.sdkwork/`
  metadata files.
- Add root `pnpm-workspace.yaml` as the central pnpm source dependency authority.
- Keep the app-local `apps/sdkwork-im-pc/pnpm-workspace.yaml` as compatibility during migration.
- Add a static SDKWORK workspace structure test.
- Wire the new test into `package.json` and the commercial governance node test catalog.
- Document the historical PC console/admin package naming exception.
- Ignore future `.sdkwork/dart/pub-cache` content and remove the currently tracked cache entries
  from the Git index while preserving local files on disk.

Out of scope:

- Renaming console/admin packages.
- Moving existing root `config/` files.
- Editing generated SDK output.
- Regenerating SDKs.
- Running package install commands that rewrite lockfiles.
- Touching current notary, RTC, or Rust service work already modified in the tree.

## Architecture

The repository root remains the primary SDKWork repository/application root. It owns canonical
project-root capability directories and native build-tool workspace declarations.

`apps/sdkwork-im-pc` remains an independent PC application root. It owns PC-specific bootstrap,
packages, scripts, specs, app manifest, and local `.sdkwork/` metadata. Its package names are
validated in compatibility mode: normalized `sdkwork-clawchat-pc-*` packages are accepted, and
legacy console/admin package names must be explicitly listed as a migration exception.

The new verification script will be deterministic Node.js code under `scripts/`, using filesystem
and YAML text checks only. It will fail fast with actionable messages and avoid broad source scans
that would be expensive or noisy.

## Testing

Focused verification:

- `node scripts/sdkwork-workspace-structure-standard.test.mjs`
- `pnpm run test:sdkwork-workspace-structure-standard`
- `pnpm run test:workflow-commercial-gates`

Broader verification when practical:

- `pnpm run check:commercial-readiness`

The commercial readiness check may still be blocked by unrelated existing changes or capacity
evidence gates; those results must be reported as evidence, not hidden.

## Migration Notes

The historical PC console/admin names are treated as an approved local compatibility exception for
this pass only. New packages should use the PC architecture naming target:

- `sdkwork-clawchat-pc-console-*`
- `sdkwork-clawchat-pc-admin-*`

The `.sdkwork/dart/pub-cache` entries have been removed from Git tracking with `git rm --cached`.
Local cache files remain on disk and are ignored by the repository and `.sdkwork/` ignore rules.
The workspace structure test now fails if these cache entries are tracked again.
