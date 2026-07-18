# Sdkwork IM 鈥?Regulatory Compliance Framework

Status: active
Owner: SDKWork maintainers
Updated: 2026-07-03
Specs: PRIVACY_SPEC.md, SECURITY_SPEC.md, IAM_SPEC.md, REGION_SPEC.md

## 1. Purpose

This document defines the regulatory compliance framework for the Sdkwork IM platform. It covers
data protection regulations (GDPR, PIPL), data residency, audit logging, retention policies, data
subject rights, and subprocessor management.

It extends the brief summary in `DATA_PROTECTION.md` with actionable compliance controls and
evidence requirements for commercial deployment.

## 2. Regulatory Scope

### 2.1 Applicable Regulations

| Regulation | Region | Scope | Status |
| --- | --- | --- | --- |
| GDPR | EU/EEA | Personal data of EU residents | Aligned |
| PIPL | China (PRC) | Personal information of China residents | Aligned |
| CCPA/CPRA | California, USA | Personal data of California residents | Aligned |
| LGPD | Brazil | Personal data of Brazil residents | Aligned |
| ISO 27001 | International | Information security management | Roadmap (Phase 2) |
| SOC 2 Type II | International | Service organization controls | Roadmap (Phase 2) |

### 2.2 Compliance Principles

1. **Lawfulness, fairness, transparency**: Data is processed only with a valid legal basis and
   users are informed of processing activities through privacy notices.
2. **Purpose limitation**: Data collected for one purpose is not reused for incompatible purposes.
3. **Data minimization**: Only data necessary for the stated purpose is collected and retained.
4. **Accuracy**: Personal data is kept accurate and up-to-date.
5. **Storage limitation**: Data is retained only as long as necessary, then purged.
6. **Integrity and confidentiality**: Data is protected against unauthorized access, alteration,
   or destruction through technical and organizational measures.
7. **Accountability**: Compliance is demonstrated through documented evidence and audit trails.

## 3. Data Classification and Handling

### 3.1 Data Categories

| Category | Examples | Sensitivity | Encryption | Retention |
| --- | --- | --- | --- | --- |
| Personal identifiers | user ID, email, phone | High | At rest + in transit | Account lifetime + 30 days |
| Authentication data | JWT tokens, passwords (hashed) | Critical | At rest + in transit | Token: session lifetime; Password: until changed |
| Message content | chat text, attachments | High | At rest + in transit | Configurable (default: 365 days) |
| Metadata | timestamps, read receipts, presence | Medium | At rest + in transit | 90 days |
| Tenant configuration | org settings, role catalogs | Medium | At rest | Tenant lifetime |
| Audit logs | security events, access logs | High | At rest + hash chain | 7 years (regulated) |
| Telemetry | traces, metrics, structured logs | Low | In transit | 15 days (metrics), 30 days (logs) |

### 3.2 Encryption

- **In transit**: TLS 1.2+ for all HTTP and WebSocket connections. mTLS for internal RPC.
- **At rest**: PostgreSQL transparent data encryption or disk-level encryption (LUKS/cloud KMS).
- **Field-level**: Credentials and signing keys encrypted with envelope encryption (KMS-managed).
- **Key rotation**: JWT signing keys rotated every 90 days; see
  [RUNBOOK-token-key-rotation](../../runbooks/RUNBOOK-token-key-rotation.md).

## 4. Data Residency

### 4.1 Regional Deployment

| Region | Data Center | Profile | Data Stored |
| --- | --- | --- | --- |
| China (Mainland) | AliCloud / Tencent Cloud | `cloud.production` | All IM data within PRC |
| Europe (EU) | AWS Frankfurt / Azure EU | `cloud.production` | All IM data within EU |
| North America | AWS US-East / Azure US | `cloud.production` | All IM data within US |
| Private/on-prem | Customer data center | `standalone.unified-process.production` | Customer-controlled |

### 4.2 Cross-Border Transfer Controls

- No personal data is transferred across regional boundaries without explicit customer consent.
- PIPL compliance: Data exports from China require a separate data export agreement and security
  assessment per PIPL Article 38.
- GDPR compliance: Data exports from the EU rely on Standard Contractual Clauses (SCCs) or
  adequacy decisions.
- Cross-region replication is opt-in and documented in the tenant configuration.

### 4.3 Configuration

Data residency is enforced through deployment topology configuration in `etc/topology/`.
The `SDKWORK_IM_REGION` environment variable tags all data with its residency region. Migration
between regions requires a formal data migration request and customer consent.

## 5. Data Subject Rights

### 5.1 Supported Rights

| Right | GDPR Article | PIPL Article | Implementation |
| --- | --- | --- | --- |
| Access | Art. 15 | Art. 45 | Export user data via admin API |
| Rectification | Art. 16 | Art. 46 | Profile update via user API |
| Erasure | Art. 17 | Art. 47 | Tenant-scoped purge workflow |
| Portability | Art. 20 | Art. 45 | JSON export via admin API |
| Objection | Art. 21 | Art. 44 | Opt-out via tenant configuration |
| Restriction | Art. 18 | Art. 44 | Account suspension via admin API |
| Automated decision | Art. 22 | N/A | No automated decisions in IM |

### 5.2 Request Handling Workflow

1. **Request receipt**: Data subject or tenant admin submits a request through the support portal.
2. **Identity verification**: Verify the requester's identity through IAM authentication.
3. **Scope identification**: Identify all data associated with the user across services.
4. **Execution**: Execute the request (export, rectify, delete) within 30 days (GDPR) or 15
   working days (PIPL).
5. **Confirmation**: Notify the requester and tenant admin of completion.
6. **Audit record**: Log the request and response in the audit log with tamper-evident hash chain.

### 5.3 Data Export Format

Exports are delivered as a signed JSON bundle containing:

```json
{
  "userId": "string",
  "tenantId": "string",
  "exportDate": "ISO 8601 timestamp",
  "profile": { "displayName": "string", "avatar": "url", "status": "string" },
  "contacts": [{ "userId": "string", "displayName": "string", "addedAt": "timestamp" }],
  "conversations": [{
    "conversationId": "string",
    "type": "direct | group",
    "messages": [{ "messageId": "string", "body": "string", "sentAt": "timestamp" }]
  }],
  "auditEvents": [{ "eventType": "string", "timestamp": "ISO 8601", "metadata": "object" }]
}
```

Implementation path: admin/backend APIs through generated backend SDK surfaces.

## 6. Audit Logging

### 6.1 Auditable Events

| Event Category | Examples | Log Level | Retention |
| --- | --- | --- | --- |
| Authentication | login success/failure, token refresh, logout | INFO/WARN | 7 years |
| Authorization | permission grant/revoke, role change | INFO | 7 years |
| Data access | message read, conversation list, search query | INFO | 365 days |
| Data modification | message send/edit/delete, profile update | INFO | 365 days |
| Configuration | tenant setting change, quota change, feature toggle | INFO | 7 years |
| Security | failed auth attempts, rate limit hit, anomaly detected | WARN/ERROR | 7 years |
| Administrative | user create/delete/suspend, tenant create/delete | INFO | 7 years |

### 6.2 Audit Log Integrity

Audit logs use a tamper-evident hash chain:

```text
entry[n].hash = SHA256(entry[n-1].hash || entry[n].canonical_json)
```

- The genesis hash is seeded from the KMS at tenant creation.
- Any tampering with a historical entry breaks the chain and is detectable by the verification
  tool: `scripts/verify-audit-chain.ts`.
- Audit logs are stored in PostgreSQL with append-only constraints and replicated to an
  independent evidence store (S3 with object lock) daily.

### 6.3 Audit Log Investigation

See [RUNBOOK-audit-log-investigation](../../runbooks/RUNBOOK-audit-log-investigation.md) for
detailed investigation procedures.

## 7. Retention and Purge

### 7.1 Retention Classes

| Data Type | Default Retention | Configurable | Purge Method |
| --- | --- | --- | --- |
| Active messages | 365 days | Yes (tenant setting) | Scheduled purge job |
| Deleted messages | 30 days (soft delete) | No | Hard purge after grace period |
| Conversation metadata | 365 days after last activity | Yes | Scheduled purge job |
| User sessions | 24 hours (active) / 7 days (refresh) | Yes | Automatic expiry |
| Audit logs | 7 years | No (regulated) | Archive after 7 years |
| Telemetry (metrics) | 15 days | Yes | Prometheus retention |
| Telemetry (logs) | 30 days | Yes | Log rotation |
| Media files | 365 days after conversation purge | Yes | SDKWork Drive lifecycle policy |

### 7.2 Purge Workflow

1. **Schedule**: Retention scheduler runs daily at 02:00 UTC via `projection-service`.
2. **Identify**: Select records where `expires_at < NOW()` per tenant retention class.
3. **Soft purge**: Mark records as `purged=true` (recoverable for 7 days).
4. **Hard purge**: Delete records after 7-day recovery window.
5. **Verify**: Run integrity check to confirm purge and update audit log.
6. **Report**: Generate purge summary report for compliance evidence.

### 7.3 Legal Hold

When a legal hold is placed on a tenant:

- All purge operations are suspended for the affected tenant.
- Data is preserved in its current state until the hold is released.
- The hold is recorded in the audit log with the requesting authority and legal reference.
- Only authorized legal administrators can place or release holds.

## 8. Access Control

### 8.1 Authentication

- **User authentication**: OAuth 2.0 / OIDC through SDKWork IAM integration.
- **Service authentication**: mTLS between internal services; JWT for API boundaries.
- **Machine-to-machine**: Client credentials flow with scoped tokens.
- **Token validation**: JWT signature verification enforced in production
  (`SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true`).

### 8.2 Authorization

- **RBAC**: Role-based access control with tenant-scoped roles.
- **Permission scopes**: Fine-grained permissions per resource type (conversation, message, media).
- **Multi-tenant isolation**: Enforced at IAM, database schema, and AppContext projection layers.
- **Tenant isolation verification**: See
  [RUNBOOK-tenant-isolation-verification](../../runbooks/RUNBOOK-tenant-isolation-verification.md).

### 8.3 Privileged Access

- Administrative access requires MFA.
- Break-glass access is logged and reviewed weekly.
- Privileged sessions are time-limited (maximum 4 hours).
- All privileged actions are recorded in the audit log.

## 9. Subprocessor Management

### 9.1 Current Subprocessors

| Subprocessor | Purpose | Data Accessed | Regions |
| --- | --- | --- | --- |
| SDKWork IAM | Authentication and identity | User identifiers, credentials | All |
| SDKWork Drive | Media file storage | File metadata, file content | All |
| SDKWork RTC | Real-time media (calls) | Signaling metadata only | All |
| Google FCM | Push notification delivery | Device token, notification payload | Global |
| PostgreSQL | Primary data store | All IM data | Per-region |
| Redis | Cache and session state | Session tokens, presence | Per-region |

### 9.2 Subprocessor Onboarding

1. **Security assessment**: Vendor completes a security questionnaire (SOC 2 / ISO 27001 evidence).
2. **Data processing agreement**: DPA signed defining data scope, purpose, and retention.
3. **Technical integration**: Integration reviewed for data minimization and encryption.
4. **Documentation**: Subprocessor added to the public list and customer notification sent.
5. **Monitoring**: Annual review of subprocessor security posture.

### 9.3 FCM Data Handling

When `SDKWORK_IM_FCM_CREDENTIALS_PATH` is configured:

- Only delivery status metadata is logged; no message content.
- Device tokens are stored in IAM and not shared with other subprocessors.
- FCM credentials are managed through K8s Secrets or `*_FILE` secret mounts.

## 10. Breach Notification

### 10.1 Breach Detection

- Automated detection through Prometheus alerting (anomaly detection in
  `services/sdkwork-im-cloud-gateway/src/anomaly_detector.rs`).
- Manual reporting through security incident response workflow.
- External notifications from subprocessors.

### 10.2 Notification Timeline

| Authority | Notification Deadline | Condition |
| --- | --- | --- |
| Supervisory authority (GDPR) | 72 hours | Risk to user rights and freedoms |
| Cyberspace Administration (PIPL) | Immediately | Serious security incident |
| Affected data subjects | Without undue delay | High risk to rights and freedoms |
| Customer/tenant admin | 24 hours | Any confirmed breach |

### 10.3 Breach Response

1. **Containment**: Isolate affected systems, revoke compromised credentials.
2. **Assessment**: Determine scope, data types affected, and user count.
3. **Notification**: Notify authorities and affected users per the timeline above.
4. **Remediation**: Fix the vulnerability, patch systems, and restore services.
5. **Documentation**: Record the breach, response, and lessons learned in
   `docs/engineering/reviews/`.
6. **Post-incident review**: Within 72 hours of resolution.

## 11. Privacy by Design

### 11.1 Default Settings

- New tenants default to the most privacy-protective settings.
- Data retention defaults to 365 days (configurable down to 30 days).
- Telemetry redaction is enabled by default; no raw tokens or message content in logs.
- Cross-tenant data sharing is disabled by default.

### 11.2 Data Minimization in Development

- Development and staging environments use synthetic data, not production data.
- Production data access requires explicit approval and is time-limited.
- No production data is used in tests; test fixtures are generated from schemas.

## 12. Compliance Evidence

### 12.1 Evidence Collection

| Evidence Type | Source | Storage | Retention |
| --- | --- | --- | --- |
| Availability reports | Prometheus queries | S3 (compliance bucket) | 7 years |
| Audit logs | PostgreSQL audit table | S3 with object lock | 7 years |
| Incident reports | `docs/engineering/reviews/` | Git repository | Indefinite |
| Penetration test reports | Third-party assessment | Encrypted S3 | 3 years |
| Vulnerability scan results | Security scanning tools | S3 | 1 year |
| Data subject request log | Admin API | PostgreSQL | 7 years |
| Subprocessor assessments | Vendor questionnaires | Encrypted S3 | 3 years |

### 12.2 Customer Audits

Enterprise customers may request:

1. A monthly SLA compliance report (see [SLA_SLO.md](SLA_SLO.md) section 12).
2. An annual summary of audit log activity for their tenant.
3. A data flow diagram showing all subprocessors and data locations.
4. Evidence of security certifications (SOC 2, ISO 27001) when available.

## 13. References

- [DATA_PROTECTION.md](DATA_PROTECTION.md) 鈥?Data protection summary.
- [CUSTOMER_OPERATIONS.md](CUSTOMER_OPERATIONS.md) 鈥?Customer operations guide.
- [SLA_SLO.md](SLA_SLO.md) 鈥?Service level agreements and objectives.
- [docs/runbooks/RUNBOOK-audit-log-investigation.md](../../runbooks/RUNBOOK-audit-log-investigation.md)
- [docs/runbooks/RUNBOOK-tenant-isolation-verification.md](../../runbooks/RUNBOOK-tenant-isolation-verification.md)
- [docs/runbooks/RUNBOOK-token-key-rotation.md](../../runbooks/RUNBOOK-token-key-rotation.md)
- `../sdkwork-specs/PRIVACY_SPEC.md` 鈥?Platform privacy standard.
- `../sdkwork-specs/SECURITY_SPEC.md` 鈥?Platform security standard.
- `../sdkwork-specs/IAM_SPEC.md` 鈥?Identity and access management standard.
