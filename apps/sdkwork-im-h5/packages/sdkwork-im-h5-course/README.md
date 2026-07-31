# @sdkwork/im-h5-course

Reusable course compatibility adapter for the H5 mobile React surface.

- Public exports: package root and any subpaths declared by package.json; the machine contract is specs/component.spec.json.
- SDK inputs: concrete SDK construction is forbidden outside core/bootstrap; features consume core exports or injected ports.
- Configuration: supplied by the application runtime; this package owns no deployment-specific environment values.
- Extensions: public exports can be registered by the shell without importing private source paths.
- Security: appbase owns login and the global TokenManager; server authorization remains authoritative.
- Verification: run pnpm --dir apps/sdkwork-im-h5 typecheck and the component-spec commands.
- Integration: standalone and cloud applications use the same exports and vary only bootstrap topology.
