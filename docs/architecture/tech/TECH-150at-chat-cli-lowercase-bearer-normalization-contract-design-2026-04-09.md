> Migrated from `docs/架构/150AT-chat-cli-lowercase-bearer-normalization-contract-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150AT Chat CLI Lowercase Bearer Normalization Contract Design

## Problem

The CLI already exposed a stable split between header-form output and bare-token output, but provided lowercase bearer input still leaked casing into runtime results.
That meant equivalent caller input could produce different `authorization` and `token` material depending only on prefix case.

## Decision

- Treat `Bearer ` and `bearer ` as the same input contract.
- Canonicalize default output to:
  - `authorization = Bearer <token>`
  - `token = <token>`
- Define `token --token-only` as:
  - `authorization = <token>`
  - `token = <token>`

## Rationale

- This is the minimum change that makes external-token input stable and case-insensitive.
- It keeps scripted callers from having to normalize bearer casing themselves.
- It avoids any larger output-shape or flag migration.

## Boundary

- This design only covers the token command contract.
- It does not redefine wrapper output, HTTP auth behavior, or broader Step 12 compatibility governance.

