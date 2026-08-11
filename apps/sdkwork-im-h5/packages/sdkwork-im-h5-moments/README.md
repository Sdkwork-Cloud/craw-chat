# sdkwork-im-h5-moments

IM-owned moments (朋友圈) feature package for the H5 app.

## Responsibility

- Moments feed timeline (global feed over the community App API) with offset
  paging (20 per page, "load more" sentinel), publish into a selected circle,
  like/unlike, comments, and delete.
- Consumes the generated Community App SDK port injected by the host bootstrap
  (`bootstrapImMomentsH5Port`); the package never constructs SDK clients or raw
  HTTP and fails closed with `MomentCapabilityUnavailableError` before binding.

## Boundaries

- UI, view models, and orchestration live here (`frontend-feature`).
- Data authority: `sdkwork-community` App API (`/app/v3/api/community/*`),
  gateway upstream owned by `sdkwork-api-im-standalone-gateway`.
- Entry media upload is a deferred capability: the App API exposes no media
  surface today, so moments stay text-only (`images`/`video` are reserved).
- Viewer-scoped like state is kept in session memory because the API exposes
  reaction counts but no per-viewer `isLiked` flag.

## Verification

- `pnpm --dir apps/sdkwork-im-h5 typecheck`
- `node --test apps/sdkwork-im-h5/packages/sdkwork-im-h5-moments/src/services/MomentService.test.ts`
