# @sdkwork/im-h5-user

Reusable legacy user-view compatibility package for the H5 mobile React surface.

- Public exports: package root and declared package.json subpaths; specs/component.spec.json is the machine authority.
- SDK inputs: SDK clients enter through core public exports or explicit injected ports; this package does not create auth transport.
- Configuration: application bootstrap supplies runtime configuration and host capabilities.
- Extensions: compose only through public exports and @sdkwork/im-h5-shell contributions.
- Security: IAM remains the appbase SdkworkIamAuthRoutes runtime; local legacy auth is not activated.
- Verification: run pnpm --dir apps/sdkwork-im-h5 typecheck and the component-spec commands.
- Integration: standalone/cloud variation is resolved by bootstrap, leaving module behavior and visuals identical.
