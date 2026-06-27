# SDKWork IM Backend API SDK Component Specs

This directory is the local standards index for `sdkwork-im-backend-sdk`.

Root SDKWork standards remain authoritative. Local component specs can narrow or document this component, but they must not contradict [the root standards](../../../../../specs/README.md).

## Component

| Field | Value |
| --- | --- |
| Name | `sdkwork-im-backend-sdk` |
| Type | `sdk-family` |
| Root | `sdkwork-im/sdks/sdkwork-im-backend-sdk` |
| Domain | `communication` |
| Capability | `im` |
| Languages | `csharp, flutter, go, java, kotlin, python, rust, swift, typescript` |
| Status | `standardizing` |

## API Authority

| Field | Value |
| --- | --- |
| Authority | `sdkwork-im-backend-api` |
| Prefix | `/backend/v3/api` |
| Authority OpenAPI | `openapi/sdkwork-im-backend-api.openapi.yaml` |
| Derived OpenAPI | `openapi/sdkwork-im-backend-api.sdkgen.yaml` |

## Contract Manifest

- [component.spec.json](./component.spec.json) is the machine-readable component contract.
- Consumers should integrate through public exports, runtime entrypoints, SDK clients, or adapters declared in the manifest.
- Generated SDK language outputs are represented at their SDK family root instead of duplicating local specs in generated folders.

## Canonical Specs

| Spec | Applies Because |
| --- | --- |
| [API_SPEC.md](../../../../../specs/API_SPEC.md) | HTTP/OpenAPI and generated SDK contract rules. |
| [COMPONENT_SPEC.md](../../../../../specs/COMPONENT_SPEC.md) | Local component specs directory and manifest rules. |
| [CONFIG_SPEC.md](../../../../../specs/CONFIG_SPEC.md) | Runtime configuration, environment, SDK bootstrap, and feature flag rules. |
| [DOCUMENTATION_SPEC.md](../../../../../specs/DOCUMENTATION_SPEC.md) | Module README, examples, ADR, changelog, and runbook rules. |
| [DOMAIN_SPEC.md](../../../../../specs/DOMAIN_SPEC.md) | Canonical domain ownership and naming. |
| [FRONTEND_SPEC.md](../../../../../specs/FRONTEND_SPEC.md) | UI, service, SDK, accessibility, and frontend runtime rules. |
| [GOVERNANCE_SPEC.md](../../../../../specs/GOVERNANCE_SPEC.md) | Standard ownership, exception, compatibility, and migration rules. |
| [I18N_SPEC.md](../../../../../specs/I18N_SPEC.md) | User-facing language, locale, message catalog, and fallback rules. |
| [MODULE_SPEC.md](../../../../../specs/MODULE_SPEC.md) | Reusable package contract and dependency direction. |
| [README.md](../../../../../specs/README.md) | SDKWork root standards entrypoint. |
| [SDK_SPEC.md](../../../../../specs/SDK_SPEC.md) | SDK generation and SDK integration rules. |
| [SDK_WORKSPACE_GENERATION_SPEC.md](../../../../../specs/SDK_WORKSPACE_GENERATION_SPEC.md) | Application-root `sdks/` workspace, SDK family naming, API authority naming, and OpenAPI generation rules. |
| [TEST_SPEC.md](../../../../../specs/TEST_SPEC.md) | Contract, frontend, SDK, security, parity, and documentation verification rules. |

## Public Exports

- Public exports are not declared in the package manifest.

## SDK Clients

- `SdkworkImBackendClient`
- `SdkworkBackendClient` compatibility alias

## SDK Dependencies

The `sdkDependencies` contract is mandatory for this backend SDK family:

- `sdkwork-iam-backend-sdk`: role `appbase-backend-management-capability`, `dependencyMode: consumer-sdk`, `generatedTransportImportPolicy: forbidden`

## Local Extension Specs

- No local extension specs are declared yet.

## Verification

- `node apps/scripts/validate-component-specs.mjs --apps-root apps --json`
