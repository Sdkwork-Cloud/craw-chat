# Sdkwork IM — Data Protection & Privacy

Status: active  
Owner: SDKWork maintainers  
Updated: 2026-06-24  
Specs: PRIVACY_SPEC.md, SECURITY_SPEC.md

## 1. Data Classification

| Class | Examples | Controls |
| --- | --- | --- |
| Tenant metadata | organization ids, role catalogs | RBAC + audit logs |
| Message content | chat bodies, attachments metadata | tenant-scoped storage, retention classes |
| Credentials | JWT signing keys, FCM service accounts | secret mounts (`*_FILE`, K8s Secrets) |
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

- Staging profile: `cloud.split-services.staging`
- Production profile: `cloud.split-services.production`

Database and object storage residency are customer-controlled through deployment templates under `deployments/templates/`.

## 5. Subprocessors

Push delivery may invoke Google FCM when `SDKWORK_IM_FCM_CREDENTIALS_PATH` is configured. No message content is logged by the FCM adapter beyond delivery status metadata.
