# @sdkwork/im-console-shop

Capability: im-console-shop

Console placeholder for shop store administration. Canonical shop commerce UI and SDK integration live in sibling `sdkwork-shop` (`@sdkwork/shop-pc-consumer`, `@sdkwork/shop-pc-orders`).

## Ownership

| Concern | Owner |
| --- | --- |
| End-user shop / orders surfaces | `sdkwork-shop` |
| IM embedded shop tab | `@sdkwork/im-pc-shop` thin adapter |
| Console store admin APIs | `sdkwork-shop` backend (future `@sdkwork/shop-pc-console-*`) |
| This package | Contract-empty placeholder until shop console APIs ship |

The current `ConsoleStores` view renders `ConsoleContractEmptyState` intentionally — no mock store data or raw HTTP.

Authority: sibling `sdkwork-shop` PC packages and IM commerce integration (`commercePcIntegration`, `bootstrapImShopPcHost`).
