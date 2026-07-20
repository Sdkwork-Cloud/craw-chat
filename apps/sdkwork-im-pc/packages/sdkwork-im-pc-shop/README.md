# @sdkwork/im-pc-shop

Capability: im-pc-shop

Thin IM host adapter over canonical `@sdkwork/shop-pc-consumer` in `../sdkwork-shop`.

## Ownership

| Concern | Owner |
| --- | --- |
| Shop UI, services, catalog/order SDK integration | `sdkwork-shop` |
| IM toast, session, language bridge, assistant messaging | `@sdkwork/im-pc-shop` (this package) |
| Commerce session bridge | `@sdkwork/im-pc-core` (`commercePcIntegration`) |
| Gateway upstream `/app/v3/api/catalog/*`, `/order/*`, `/shop/*` | `platform.api-gateway` |

Bootstrap: `apps/sdkwork-im-pc/src/bootstrap/shopPc.ts` and `commercePc.ts` run before render.
