> Migrated from `docs/架构/150AU-chat-cli-provided-token-claims-boundary-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150AU Chat CLI Provided-Token Claims Boundary Design

## Problem

The CLI returned a `claims` object even when callers supplied an external bearer token through `--bearer-token`.
Those values came from local CLI input, not from the external token itself, so the output overstated what the CLI had actually decoded or verified.

## Decision

- Preserve the existing token command envelope:
  - `source`
  - `authorization`
  - `token`
  - `claims`
- Define generated-token output as:
  - `source = generatedBearerToken`
  - `claims = <local signed claims>`
- Define provided-token output as:
  - `source = providedBearerToken`
  - `claims = null`

## Rationale

- This is the minimum change that restores output honesty without adding a JWT decoding subsystem.
- It keeps generated-token introspection available where the CLI actually knows the claim payload.
- It prevents scripted callers from mistaking local CLI context for decoded external-token claims.

## Boundary

- This design only covers the token command contract.
- It does not add external-token verification, decoding, or validation.

