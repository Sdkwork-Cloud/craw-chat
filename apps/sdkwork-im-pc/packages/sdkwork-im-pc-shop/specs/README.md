# Component Specs

This directory is the local SDKWork component contract for `@sdkwork/im-pc-shop`.

- Component root: `sdkwork-im/apps/sdkwork-im-pc/packages/sdkwork-im-pc-shop`
- Canonical Shop product: `sdkwork-shop/apps/sdkwork-shop-pc/packages/sdkwork-shop-pc-consumer`
- Session bridge: `@sdkwork/im-pc-core/sdk/commercePcIntegration` and `bootstrapImShopPcHost`
- Integration pattern: thin host adapter only; shop UI, services, and SDK wiring live in `sdkwork-shop-pc`

Read `specs/component.spec.json` before changing this component's public exports, runtime entrypoints, or verification commands.

Do not copy root standards into this directory. Link to files under `../../../../../sdkwork-specs/` instead.
