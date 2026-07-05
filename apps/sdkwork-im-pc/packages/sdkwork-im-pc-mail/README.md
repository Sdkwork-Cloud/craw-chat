# @sdkwork/im-pc-mail

Capability: im-pc-mail

Thin IM host adapter over canonical `@sdkwork/mail-pc-mail` in `../sdkwork-mail`.

## Ownership

| Concern | Owner |
| --- | --- |
| UI, services, OpenAPI, Rust domain | `sdkwork-mail` |
| IM session bridge into mail IAM session | `@sdkwork/im-pc-core` (`mailPcIntegration`) |
| Gateway upstream `/app/v3/api/mail/*` | `sdkwork-im-cloud-gateway` |

Bootstrap: `apps/sdkwork-im-pc/src/bootstrap/mailPc.ts` calls `bootstrapMailPcForIm()` before render.

Authority: sibling `sdkwork-mail` PC packages (`@sdkwork/mail-pc-core`, `@sdkwork/mail-pc-mail`).
