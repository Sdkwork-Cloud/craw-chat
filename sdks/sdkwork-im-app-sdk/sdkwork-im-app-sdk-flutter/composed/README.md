# im_app_sdk

Official consumer-facing Flutter package for the app SDK family.

This package is the manual-owned `composed` layer in `sdkwork-im-app-sdk-flutter`. It sits above
and re-exports the generated `im_app_api_generated` transport package.

Use this package first when building app-business capabilities on `/app/v3/api`:

- tenant portal snapshots
- notifications
- automation runtime
- provider health
- RTC provider callbacks and health

Current boundary:

- `im_app_sdk` is consumer-facing and manual-owned.
- `im_app_api_generated` remains generator-owned under `../generated/server-openapi`.
- IM standard capability stays in `im_sdk`; AIoT device capability stays in `sdkwork_aiot_app_sdk`;
  RTC provider-standard capability stays in `rtc_sdk`.

## Usage

```dart
import 'package:im_app_sdk/im_app_sdk.dart';

final sdk = ImAppSdkClient.create(
  baseUrl: 'https://api.example.com',
  authToken: '<auth-token>',
  accessToken: '<access-token>',
);

final workspace = await sdk.portal.workspace();
final notification = await sdk.notification.get('<notification-id>');
```

`ImAppSdkClient` also exposes raw generated route groups (`automationApi`, `notificationsApi`,
`portalApi`, `providerApi`) when direct transport access is needed.

## SDKWork Documentation Contract

Domain: communication
Capability: im-app-sdk
Package type: flutter-package
Status: standard

### Public API

Public exports are declared in `specs/component.spec.json` under `contracts.publicExports`.

### Required SDK Surface

- None declared in `specs/component.spec.json`.

### Configuration

Configuration keys and runtime entrypoints are declared in `specs/component.spec.json`.

### SaaS/Private/Local Behavior

This module follows the canonical standards linked from `specs/component.spec.json`, including deployment and runtime configuration rules where applicable.

### Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module.

### Extension Points

Extension points are limited to declared public exports, runtime entrypoints, SDK clients, events, and config keys.

### Verification

- `powershell -NoProfile -Command "Get-Content specs/component.spec.json -Raw | ConvertFrom-Json | Out-Null"`

### Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
