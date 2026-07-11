# sdkwork_im_flutter_mobile_shell Specs

This directory is the local spec index for the Flutter mobile shell package.

Authority:

- `component.spec.json` is the machine-readable package contract.
- Global rules remain in `../../../../../../sdkwork-specs/`; this directory links to them and does not copy their text.

Package role:

- Owns app shell/scaffold composition primitives.
- Does not own business services, SDK construction, session persistence, or backend-admin logic.

Verification:

- `flutter analyze` from `apps/sdkwork-im-flutter-mobile`
- `flutter test` from `apps/sdkwork-im-flutter-mobile`
