# Sdkwork IM 鈥?Data Protection & Privacy

Status: active  
Owner: SDKWork maintainers  
Updated: 2026-07-21
Specs: PRIVACY_SPEC.md, SECURITY_SPEC.md

## 1. Data Classification

| Class | Examples | Controls |
| --- | --- | --- |
| Tenant metadata | organization ids, role catalogs | RBAC + audit logs |
| Message content | chat bodies, attachments metadata | tenant-scoped storage, retention classes |
| Credentials | JWT signing keys, future provider credentials | secret mounts (`*_FILE`, K8s Secrets) |
| Telemetry | traces, metrics, structured logs | redaction, no raw tokens in logs |

## 2. Retention

- Conversation and projection data honor configured retention classes.
- Automated purge jobs run through postgres-journal retention scheduler.
- Legal hold flows are validated in projection-service retention tests.

## 3. Export and Deletion

Operators should provide:

1. Tenant identifier and organization scope.
2. Export window or full tenant export request.
3. Deletion confirmation with rollback window when legally required.

Implementation path:

- Export: admin/backend APIs through generated backend SDK surfaces.
- Deletion: tenant-scoped purge workflows coordinated with IAM directory state.

## 4. Regional Deployment

- Staging profile: `cloud.staging`
- Production profile: `cloud.production`

Database and object storage residency are customer-controlled through deployment templates under `deployments/templates/`.

## 5. Subprocessors

Push delivery is not currently implemented and Google FCM is not an active IM subprocessor. A
future provider integration requires an approved data-flow review, device-token retention/deletion
contract, payload minimization, secret ownership, log-redaction tests, and an updated subprocessor
notice before activation.
