import {
  arrayField,
  field,
  type ApiSchemaDefinitionMap,
  type ApiSchemaField,
} from "./schema-types";

const notificationTaskFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("notificationId", "string", "Notification task identifier.", { required: true }),
  field("sourceEventId", "string", "Source event identifier.", { required: true }),
  field("sourceEventType", "string", "Source event type.", { required: true }),
  field("category", "string", "Notification category.", { required: true }),
  field("channel", "string", "Delivery channel.", { required: true }),
  field("recipientId", "string", "Recipient principal identifier.", { required: true }),
  field("status", "string", "Notification state. Supported values: requested, dispatched, failed.", {
    required: true,
  }),
  field("title", "string | null", "Notification title."),
  field("body", "string | null", "Notification body."),
  field("payload", "string | null", "Optional extended payload."),
  field("requestedAt", "date-time string", "Request timestamp.", { required: true }),
  field("dispatchedAt", "date-time string | null", "Dispatch timestamp."),
  field("failureReason", "string | null", "Failure reason."),
];

const automationExecutionFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("principalId", "string", "Principal identifier.", { required: true }),
  field("principalKind", "string", "Principal kind.", { required: true }),
  field("executionId", "string", "Automation execution identifier.", { required: true }),
  field("triggerType", "string", "Trigger type.", { required: true }),
  field("targetKind", "string", "Target kind.", { required: true }),
  field("targetRef", "string", "Target reference.", { required: true }),
  field("inputPayload", "string | null", "Input payload."),
  field("outputPayload", "string | null", "Output payload."),
  field("state", "string", "Execution state. Supported values: requested, running, succeeded, failed.", {
    required: true,
  }),
  field("retryCount", "uint32", "Retry count.", { required: true }),
  field("requestedAt", "date-time string", "Request timestamp.", { required: true }),
  field("completedAt", "date-time string | null", "Completion timestamp."),
  field("failureReason", "string | null", "Failure reason."),
];

const auditRecordFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("recordId", "string", "Audit record identifier.", { required: true }),
  field("aggregateType", "string", "Aggregate type.", { required: true }),
  field("aggregateId", "string", "Aggregate identifier.", { required: true }),
  field("action", "string", "Audit action.", { required: true }),
  field("actorId", "string", "Actor identifier.", { required: true }),
  field("actorKind", "string", "Actor kind.", { required: true }),
  field("actorSessionId", "string | null", "Actor session identifier."),
  field("payload", "string | null", "Additional audit payload."),
  field("recordedAt", "date-time string", "Record timestamp.", { required: true }),
  field("chainPrevHash", "string | null", "Hash pointer to the previous audit record in tenant order."),
  field("chainHash", "string", "Current record chain hash (SHA-256 canonical digest).", { required: true }),
];

export const platformBusinessSchemas: ApiSchemaDefinitionMap = {
  RequestNotification: {
    fields: [
      field("notificationId", "string", "Notification request identifier.", { required: true }),
      field("sourceEventId", "string", "Source event identifier.", { required: true }),
      field("sourceEventType", "string", "Source event type.", { required: true }),
      field("category", "string", "Notification category.", { required: true }),
      field("channel", "string", "Delivery channel.", { required: true }),
      field("recipientId", "string", "Recipient principal identifier.", { required: true }),
      field("title", "string | null", "Notification title."),
      field("body", "string | null", "Notification body."),
      field("payload", "string | null", "Optional extended payload."),
    ],
  },
  NotificationTask: {
    fields: notificationTaskFields,
  },
  NotificationListResponse: {
    fields: [
      arrayField("items", "NotificationTask", "Notification tasks.", notificationTaskFields, {
        required: true,
        summary: "View nested fields for items",
      }),
    ],
  },
  RequestAutomationExecution: {
    fields: [
      field("executionId", "string", "Automation execution identifier.", { required: true }),
      field("triggerType", "string", "Trigger type.", { required: true }),
      field("targetKind", "string", "Target kind.", { required: true }),
      field("targetRef", "string", "Target reference.", { required: true }),
      field("inputPayload", "string | null", "Input payload."),
    ],
  },
  AutomationExecution: {
    fields: automationExecutionFields,
  },
  RecordAuditAnchor: {
    fields: [
      field("recordId", "string", "Audit record identifier.", { required: true }),
      field("aggregateType", "string", "Aggregate type.", { required: true }),
      field("aggregateId", "string", "Aggregate identifier.", { required: true }),
      field("action", "string", "Audit action.", { required: true }),
      field("payload", "string | null", "Additional audit payload."),
    ],
  },
  AuditRecord: {
    fields: auditRecordFields,
  },
  AuditRecordListResponse: {
    fields: [
      arrayField("items", "AuditRecord", "Audit records.", auditRecordFields, {
        required: true,
        summary: "View nested fields for items",
      }),
    ],
  },
  AuditExportBundle: {
    fields: [
      field("tenantId", "string", "Tenant identifier.", { required: true }),
      field("exportedAt", "date-time string", "Export timestamp.", { required: true }),
      field("total", "uint64", "Number of exported records.", { required: true }),
      arrayField("items", "AuditRecord", "Exported audit records.", auditRecordFields, {
        required: true,
        summary: "View nested fields for items",
      }),
      field("chainHeadHash", "string | null", "Latest chain hash at export time."),
      field("chainValid", "boolean", "Whether export records pass hash-chain integrity verification.", {
        required: true,
      }),
    ],
  },
  AuditChainVerification: {
    fields: [
      field("tenantId", "string", "Tenant identifier.", { required: true }),
      field("verifiedAt", "date-time string", "Verification timestamp.", { required: true }),
      field("total", "uint64", "Number of records included in verification.", { required: true }),
      field("chainHeadHash", "string | null", "Latest chain hash at verification time."),
      field("chainValid", "boolean", "Whether the tenant audit chain is currently valid.", {
        required: true,
      }),
    ],
  },
};
