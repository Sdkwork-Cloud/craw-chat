# @sdkwork/im-pc-community

Capability: im-pc-community

Thin IM host adapter over canonical `@sdkwork/community-pc-community` in `../sdkwork-community`.

## Ownership

| Concern | Owner |
| --- | --- |
| UI, service, OpenAPI, Rust domain | `sdkwork-community` |
| IM toast, avatar, session, language bridge, SDK port injection | `@sdkwork/im-pc-community` (this package) |
| Gateway upstream `/app/v3/api/community/*` | `sdkwork-im-cloud-gateway` |

Bootstrap: `apps/sdkwork-im-pc/src/bootstrap/communityPc.ts` calls `bootstrapCommunityPcForIm()` then `bootstrapImCommunityPcHost()` before render.

Authority: `../sdkwork-community/docs/architecture/tech/TECH-2026-06-06-sdkwork-community-migration-design.md`.

This README is the SDKWork module entrypoint for `@sdkwork/im-pc-community`. The machine-readable component contract is `specs/component.spec.json`; canonical standards are under `../../../../../sdkwork-specs/`.
