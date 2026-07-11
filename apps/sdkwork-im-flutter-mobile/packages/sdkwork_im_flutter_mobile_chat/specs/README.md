# sdkwork_im_flutter_mobile_chat Specs

This directory is the local spec index for the Flutter mobile chat capability package.

Authority:

- `component.spec.json` is the machine-readable package contract.
- Global rules remain in `../../../../../../sdkwork-specs/`; this directory links to them and does not copy their text.

Package role:

- Owns chat inbox/conversation screens, chat services, realtime UI coordination, and offline send queue.
- Consumes IM SDK capability through core/composed SDK boundaries.
- Does not construct SDK clients, persist auth/session credentials, or implement backend-admin logic.

Verification:

- `flutter analyze` from `apps/sdkwork-im-flutter-mobile`
- `flutter test` from `apps/sdkwork-im-flutter-mobile`
- `node ../sdkwork-specs/tools/check-pagination.mjs --workspace .` from the repository root when list behavior changes
