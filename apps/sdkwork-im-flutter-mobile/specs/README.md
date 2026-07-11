# SDKWork IM Flutter Mobile Specs

This directory is the local spec index for the Flutter mobile application root.

Authority:

- `component.spec.json` is the machine-readable application-root contract.
- Global rules remain in `../../../sdkwork-specs/`; this directory links to them and does not copy their text.

Primary contracts:

- Application root: `apps/sdkwork-im-flutter-mobile`
- Runtime family: Flutter mobile app
- Surface: app/user-facing
- SDK boundary: generated/composed IM Flutter SDKs plus appbase IAM session integration through bootstrap.

Verification:

- `flutter analyze`
- `flutter test`
- `node ../../../sdkwork-specs/tools/check-frontend-composition.mjs --root ../..`
- `node ../../../sdkwork-specs/tools/check-application-layering.mjs --root ../..`
