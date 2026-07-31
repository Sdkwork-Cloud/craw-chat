# @sdkwork/im-h5-attendance

Reusable IM attendance capability for the H5 mobile React surface.

- Public exports: package root; the exact contract is listed in specs/component.spec.json.
- SDK inputs: no SDK client is constructed here; remote access uses core public SDK exports or injected ports.
- Configuration: host-owned runtime configuration only; this module defines no independent environment authority.
- Extensions: compose exported pages/components through @sdkwork/im-h5-shell module contributions.
- Security: authentication, credentials, permission enforcement, and TokenManager ownership remain in appbase/core bootstrap.
- Verification: run pnpm --dir apps/sdkwork-im-h5 typecheck and the commands in the component spec.
- Integration: standalone and cloud H5 builds share this module; deployment differences stay in bootstrap.
