# Component Specs

This directory is the local SDKWork component contract for `@sdkwork/im-pc-mail`.

- Component root: `sdkwork-im/apps/sdkwork-im-pc/packages/sdkwork-im-pc-mail`
- Canonical Mail product: `sdkwork-mail/apps/sdkwork-mail-pc/packages/sdkwork-mail-pc-mail`
- Session bridge: `@sdkwork/im-pc-core/sdk/mailPcIntegration`
- Integration pattern: thin host adapter only; mail UI, services, and SDK wiring live in `sdkwork-mail-pc`

Read `specs/component.spec.json` before changing this component's public exports, runtime entrypoints, or verification commands.

Do not copy root standards into this directory. Link to files under `../../../../../sdkwork-specs/` instead.
