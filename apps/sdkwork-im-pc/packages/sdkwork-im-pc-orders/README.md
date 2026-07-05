# @sdkwork/im-pc-orders

Capability: im-pc-orders

Thin IM host adapter over canonical `@sdkwork/shop-pc-orders` in `../sdkwork-shop`.

## Ownership

| Concern | Owner |
| --- | --- |
| Orders UI and order/shop SDK integration | `sdkwork-shop` |
| Commerce session bridge | `@sdkwork/im-pc-core` (`commercePcIntegration`) |

Bootstrap: shares `commercePcIntegration` and shop host wiring from `shopPc.ts`.
