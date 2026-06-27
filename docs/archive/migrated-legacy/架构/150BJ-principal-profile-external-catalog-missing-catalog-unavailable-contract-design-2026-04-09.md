# 150BJ User-Module External Missing-Catalog Unavailable Contract Design

## Problem

`principal-profile-external-catalog` already had unavailable semantics for unreadable or invalid catalogs, but the missing catalog-path branch still panicked during bootstrap.
That made one provider emit two incompatible failure modes for the same class of configuration error.

## Decision

- Preserve the existing provider selection model.
- Preserve invalid provider-mode fast rejection.
- Introduce an unavailable provider wrapper only for the missing external catalog-path branch.
- Let business routes consume it through the existing `ContractError::Unavailable` path.

## Rationale

- This is the minimum change that restores one consistent failure model inside the external provider path.
- It preserves app boot and observability.
- It avoids silent fallback to local provider, which would hide operator mistakes.

## Boundary

- This design only covers:
  - `sdkwork_im_PRINCIPAL_PROFILE_PROVIDER=external`
  - missing or blank `sdkwork_im_PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH`
- It does not change:
  - invalid provider mode behavior
  - unreadable/invalid catalog file behavior
  - principal-profile provider health route exposure
