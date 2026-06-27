# SDKWork IM Backend SDK OpenAPI Sources

This directory stores the OpenAPI source documents for the `/backend/v3/api` SDK family.
The backend family is the single authority for management-system HTTP APIs, including operator,
control-plane, and admin-console routes.

## Files

- `sdkwork-im-backend-api.openapi.yaml`
  Authority OpenAPI 3.x contract for backend/operator Sdkwork IM APIs.
- `sdkwork-im-backend-api.sdkgen.yaml`
  Generator-compatible derived input.

## Rules

- `/backend/v3/openapi.json` is the live service schema export.
- `SDKWORK_IM_BACKEND_API_OPENAPI_SCHEMA_PATH` can override the runtime schema export source.
- Login, account, tenant, organization, token, and client client-route lifecycle APIs stay outside
  this backend SDK.
- Non-management provider, IoT, RTC, and app-business HTTP APIs stay in the app SDK family.
- `/backend/v3/api/control/*` and `/backend/v3/api/admin/*` are backend modules in this workspace,
  not separate SDK families.
- Generated SDK packages must never edit these files in place.
