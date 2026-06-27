# SDKWork IM SDK Component Specs

This directory is the local standards index for `sdkwork-im-sdk`.

Root SDKWork standards remain authoritative. Local component specs can narrow or
document this component, but they must not contradict
[the root standards](../../../sdkwork-specs/README.md).

## Component

| Field | Value |
| --- | --- |
| Name | `sdkwork-im-sdk` |
| Type | `sdk-family` |
| Root | `sdkwork-im/sdks/sdkwork-im-sdk` |
| Domain | `communication` |
| Capability | `im` |
| Languages | `csharp, flutter, go, java, kotlin, python, rust, swift, typescript` |
| Status | `standardizing` |

## API Authority

| Field | Value |
| --- | --- |
| Authority | `sdkwork-im-open-api` |
| Prefix | `/im/v3/api` |
| Authority OpenAPI | `openapi/sdkwork-im-im.openapi.yaml` |
| Derived OpenAPI | `openapi/sdkwork-im-im.sdkgen.yaml`, `openapi/sdkwork-im-im.flutter.sdkgen.yaml` |

## Contract Manifest

- [component.spec.json](./component.spec.json) is the machine-readable component contract.
- Consumers integrate through `@sdkwork/im-sdk`, generated SDK clients, or documented adapters only.
- Generated transport output is represented at the SDK family root instead of duplicating local specs in generated folders.

## Canonical Specs

| Spec | Applies Because |
| --- | --- |
| [API_SPEC.md](../../../sdkwork-specs/API_SPEC.md) | HTTP/OpenAPI and generated SDK contract rules. |
| [COMPONENT_SPEC.md](../../../sdkwork-specs/COMPONENT_SPEC.md) | Local component specs directory and manifest rules. |
| [CONFIG_SPEC.md](../../../sdkwork-specs/CONFIG_SPEC.md) | Runtime configuration, environment, SDK bootstrap, and feature flag rules. |
| [DOCUMENTATION_SPEC.md](../../../sdkwork-specs/DOCUMENTATION_SPEC.md) | Module README, examples, ADR, changelog, and runbook rules. |
| [DOMAIN_SPEC.md](../../../sdkwork-specs/DOMAIN_SPEC.md) | Canonical domain ownership and naming. |
| [FRONTEND_SPEC.md](../../../sdkwork-specs/FRONTEND_SPEC.md) | UI, service, SDK, accessibility, and frontend runtime rules. |
| [GOVERNANCE_SPEC.md](../../../sdkwork-specs/GOVERNANCE_SPEC.md) | Standard ownership, exception, compatibility, and migration rules. |
| [MODULE_SPEC.md](../../../sdkwork-specs/MODULE_SPEC.md) | Reusable package contract and dependency direction. |
| [README.md](../../../sdkwork-specs/README.md) | SDKWork root standards entrypoint. |
| [SDK_SPEC.md](../../../sdkwork-specs/SDK_SPEC.md) | SDK generation and SDK integration rules. |
| [SDK_WORKSPACE_GENERATION_SPEC.md](../../../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md) | Application-root `sdks/` workspace, SDK family naming, API authority naming, and OpenAPI generation rules. |
| [TEST_SPEC.md](../../../sdkwork-specs/TEST_SPEC.md) | Contract, frontend, SDK, security, parity, and documentation verification rules. |

## Public Exports

- `@sdkwork/im-sdk`

## SDK Clients

- `SdkworkImClient` generated transport client.
- `ImSdkClient` TypeScript composed facade for app-facing IM behavior.

## SDK Dependencies

The `sdkDependencies` contract is explicit and empty for this owner-only IM SDK family.

## Local Extension Specs

- No local extension specs are declared yet.

## Verification

- `node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs`
- `node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language typescript` for narrow TypeScript facade work.
