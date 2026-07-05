# Component Specs

This directory is the local SDKWork component contract for `@sdkwork/im-pc-devices`.

- Component root: `sdkwork-im/apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices`
- Canonical AIoT product: `sdkwork-aiot/apps/sdkwork-aiot-pc/packages/sdkwork-aiot-pc-console-device`
- Session bridge: `@sdkwork/im-pc-core/sdk/aiotPcIntegration`
- Integration pattern: thin host adapter only; device UI, services, and SDK wiring live in `sdkwork-aiot-pc`

Read `specs/component.spec.json` before changing this component's public exports, runtime entrypoints, or verification commands.

Do not copy root standards into this directory. Link to files under `../../../../../sdkwork-specs/` instead.
