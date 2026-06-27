# im_backend_sdk

Official consumer-facing Flutter package for the backend SDK family.

This package is the manual-owned `composed` layer in `sdkwork-im-backend-sdk-flutter`. It sits
above and re-exports the generated `im_backend_api_generated` transport package.

Use this package for backend/operator/control/admin capability on `/backend/v3/api`:

- ops diagnostics and runtime health
- audit export and record surfaces
- automation governance
- control-plane and policy governance
- admin-console APIs

Current boundary:

- `im_backend_sdk` is consumer-facing and manual-owned.
- `im_backend_api_generated` stays generator-owned under `../generated/server-openapi`.
- This package does not own app-business `/app/v3/api` or IM standardized `/im/v3/api` routes.

## Usage

```dart
import 'package:im_backend_sdk/im_backend_sdk.dart';

final sdk = ImBackendSdkClient.create(
  baseUrl: 'https://api.example.com',
  authToken: '<auth-token>',
  accessToken: '<access-token>',
);

final health = await sdk.ops.health();
final protocolRegistry = await sdk.control.protocolRegistry();
```

`ImBackendSdkClient` also exposes raw generated route groups (`opsApi`, `auditApi`, `automationApi`,
`controlApi`, `adminApi`) when direct transport access is required.

## SDKWork Documentation Contract

Domain: communication
Capability: im-backend-sdk
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
