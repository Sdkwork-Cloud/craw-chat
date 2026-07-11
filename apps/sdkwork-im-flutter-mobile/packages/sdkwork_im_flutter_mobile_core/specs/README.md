# sdkwork_im_flutter_mobile_core Specs

This directory is the local spec index for the Flutter mobile core package.

Authority:

- `component.spec.json` is the machine-readable package contract.
- Global rules remain in `../../../../../../sdkwork-specs/`; this directory links to them and does not copy their text.

Package role:

- Owns Flutter mobile runtime primitives: IM SDK client factory, Drive SDK wrapper, Appbase callback bridge, and local session model.
- Does not own screens, widgets, feature workflows, or backend-admin logic.

Verification:

- `flutter analyze` from `apps/sdkwork-im-flutter-mobile`
- `flutter test` from `apps/sdkwork-im-flutter-mobile`
- `node scripts/dev/sdkwork-im-mobile-auth-session-standard.test.mjs` from the repository root
