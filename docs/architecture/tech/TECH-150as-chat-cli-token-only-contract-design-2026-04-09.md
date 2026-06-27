> Migrated from `docs/架构/150AS-chat-cli-token-only-contract-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150AS Chat CLI Token-Only Contract Design

## Problem

The CLI exposed a `--token-only` flag, but the returned `authorization` field still kept `Bearer `.
That meant the contract name and runtime behavior diverged.

## Decision

- Preserve the current JSON envelope:
  - `source`
  - `authorization`
  - `token`
  - `claims`
- Preserve default `token` behavior:
  - `authorization = Bearer <token>`
  - `token = <token>`
- Define `token --token-only` as:
  - `authorization = <token>`
  - `token = <token>`

## Rationale

- This is the minimum change that makes the flag semantically real.
- It avoids a larger output-shape migration.
- It keeps scripted callers able to choose between header-form and bare-token material without guessing.

## Source Semantics

- `source = generatedBearerToken`
  - the CLI signed the token from `--public-bearer-secret`
- `source = providedBearerToken`
  - the CLI consumed `--bearer-token`

## Boundary

- This design only covers the token command contract.
- It does not redefine wrapper output, SDK behavior, or broader Step 12 compatibility governance.

