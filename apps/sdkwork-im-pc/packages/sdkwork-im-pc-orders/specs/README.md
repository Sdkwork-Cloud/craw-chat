# Component Specs

This directory is the local SDKWork component contract for `@sdkwork/im-pc-orders`.

- Component root: `sdkwork-im/apps/sdkwork-im-pc/packages/sdkwork-im-pc-orders`
- Canonical Shop orders product: `sdkwork-shop/apps/sdkwork-shop-pc/packages/sdkwork-shop-pc-orders`
- Session bridge: `@sdkwork/im-pc-core/sdk/commercePcIntegration`
- Integration pattern: thin host adapter only; orders UI, services, and SDK wiring live in `sdkwork-shop-pc`

Read `specs/component.spec.json` before changing this component's public exports, runtime entrypoints, or verification commands.

Do not copy root standards into this directory. Link to files under `../../../../../sdkwork-specs/` instead.
