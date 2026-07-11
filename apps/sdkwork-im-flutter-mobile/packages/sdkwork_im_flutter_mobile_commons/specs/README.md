# sdkwork_im_flutter_mobile_commons Specs

This directory is the local spec index for the Flutter mobile commons package.

Authority:

- `component.spec.json` is the machine-readable package contract.
- Global rules remain in `../../../../../../sdkwork-specs/`; this directory links to them and does not copy their text.

Package role:

- Owns domain-neutral formatting and shared UI helper primitives.
- Does not own SDK construction, auth/session storage, business workflows, or backend-admin logic.

Verification:

- `flutter analyze` from `apps/sdkwork-im-flutter-mobile`
- `flutter test` from `apps/sdkwork-im-flutter-mobile`
