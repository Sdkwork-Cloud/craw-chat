# SDKWork IM App SDK OpenAPI Sources

This directory stores the OpenAPI source documents for the `/app/v3/api` SDK family. The app family
owns app-business and non-management HTTP APIs that are not part of the IM standardized development
API.

## Files

- `sdkwork-im-app-api.openapi.yaml`
  Authority OpenAPI 3.x contract for app developers building instant messaging apps.
- `sdkwork-im-app-api.sdkgen.yaml`
  Generator-compatible derived input.
- `sdkwork-im-app-api.flutter.sdkgen.yaml`
  Flutter-compatible derived input.

## Rules

- `/app/v3/openapi.json` is the live service schema export.
- `SDKWORK_IM_APP_API_OPENAPI_SCHEMA_PATH` can override the runtime schema export source.
- Non-management provider, platform callback, or health HTTP routes belong here when they are
  exposed to app/business integrations.
- AIoT `/app/v3/api/iot/*` routes belong to `sdkwork-aiot` and are consumed through
  `sdkwork-aiot-app-sdk`, not regenerated in this family.
- Management-system, control-plane, and admin-console APIs stay in `sdkwork-im-backend-sdk`.
- Generated SDK packages must never edit these files in place.
