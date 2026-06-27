import {
  arrayField,
  field,
  objectField,
  type ApiSchemaDefinitionMap,
  type ApiSchemaField,
} from "./schema-types";

export const senderFields: ApiSchemaField[] = [
  field("id", "string", "Sender principal identifier.", { required: true }),
  field(
    "kind",
    "string",
    "Sender principal kind. Common values: user, agent, system, device.",
    {
      required: true,
    },
  ),
  field("memberId", "string | null", "Conversation member identifier for the sender."),
  field("deviceId", "string | null", "Device identifier used to emit the event."),
  field("sessionId", "string | null", "Session identifier used to emit the event."),
  field("metadata", "Record<string, string>", "Additional sender metadata."),
];

export const mediaResourceFields: ApiSchemaField[] = [
  field("id", "uint64 | null", "Numeric resource identifier."),
  field("uuid", "string | null", "Resource UUID."),
  field("url", "string | null", "Public or signed resource URL."),
  field("bytes", "byte[] | null", "Inline raw bytes when the payload is embedded."),
  field("localFile", "string | null", "Local file path used for upload or processing."),
  field("base64", "string | null", "Base64-encoded payload."),
  field(
    "type",
    "string | null",
    "Logical resource type. Common values: image, video, audio, file.",
  ),
  field("mimeType", "string | null", "MIME type."),
  field("size", "uint64 | null", "Resource size in bytes."),
  field("name", "string | null", "Display name or file name."),
  field("extension", "string | null", "File extension."),
  field("tags", "Record<string, string> | null", "Resource tag map."),
  field("metadata", "Record<string, string> | null", "Resource metadata."),
  field("prompt", "string | null", "Prompt text for generated media."),
];

export const contentPartFields: ApiSchemaField[] = [
  field(
    "kind",
    "string",
    "Content part type. Supported values include text, data, media, signal, and stream_ref.",
    {
      required: true,
    },
  ),
  field("text", "string | null", "Inline text when kind is text."),
  field("schemaRef", "string | null", "Schema reference for structured payloads."),
  field("encoding", "string | null", "Encoding for payload or stream fragments, for example json."),
  field("payload", "string | null", "Raw structured payload."),
  field("mediaAssetId", "string | null", "Referenced media asset identifier when kind is media."),
  objectField(
    "resource",
    "Inline media resource object returned for media parts.",
    mediaResourceFields,
    {
      summary: "View nested fields for resource",
    },
  ),
  field("signalType", "string | null", "Signal type when kind is signal."),
  field("streamId", "string | null", "Referenced stream identifier when kind is stream_ref."),
  field("streamType", "string | null", "Referenced stream type when kind is stream_ref."),
  field("state", "string | null", "Current state of the referenced stream."),
];

export const messageBodyFields: ApiSchemaField[] = [
  field("summary", "string | null", "Message summary."),
  arrayField("parts", "ContentPart", "Ordered content parts.", contentPartFields, {
    summary: "View nested fields for parts",
  }),
  field("renderHints", "Record<string, string>", "Client rendering hints."),
];

export const mediaAssetFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("principalId", "string", "Owning principal identifier.", { required: true }),
  field("principalKind", "string", "Owning principal kind.", { required: true }),
  field("mediaAssetId", "string", "Media asset identifier.", { required: true }),
  field("bucket", "string | null", "Object storage bucket."),
  field("objectKey", "string | null", "Object storage key."),
  field("storageProvider", "string | null", "Selected object storage provider plugin."),
  field("checksum", "string | null", "Upload checksum."),
  field(
    "processingState",
    "string",
    "Asset processing state. Supported values: pendingUpload, ready.",
    {
      required: true,
    },
  ),
  objectField("resource", "Media resource payload.", mediaResourceFields, {
    required: true,
    summary: "View nested fields for resource",
  }),
  field("createdAt", "date-time string", "Creation timestamp.", { required: true }),
  field("completedAt", "date-time string | null", "Upload completion timestamp."),
];

export const providerHealthSnapshotFields: ApiSchemaField[] = [
  field("pluginId", "string", "Provider plugin identifier.", { required: true }),
  field("status", "string", "Health state. Current implementations typically return healthy.", {
    required: true,
  }),
  field("checkedAt", "date-time string", "Timestamp of the health probe.", { required: true }),
  field("details", "Record<string, string>", "Provider-specific diagnostics.", {
    required: true,
  }),
];

export const commonApiSchemas: ApiSchemaDefinitionMap = {
  HealthResponse: {
    fields: [
      field("status", "string", "Health status. Fixed to ok.", { required: true }),
      field("service", "string", "Service name.", { required: true }),
      field("profile", "string | null", "Deployment profile when exposed by the service."),
    ],
  },
  ControlPlaneHealthResponse: {
    fields: [
      field("status", "string", "Health status. Fixed to ok.", { required: true }),
      field("service", "string", "Service name.", { required: true }),
    ],
  },
  ApiError: {
    fields: [
      field("code", "string", "Machine-readable error code.", { required: true }),
      field("message", "string", "Human-readable error message.", { required: true }),
      field("status", "string | null", "Optional error status enum returned by some control-plane APIs."),
    ],
  },
  Sender: {
    fields: senderFields,
  },
  ContentPart: {
    fields: contentPartFields,
  },
  MessageBody: {
    fields: messageBodyFields,
  },
  MediaResource: {
    fields: mediaResourceFields,
  },
  MediaAsset: {
    fields: mediaAssetFields,
  },
  ProviderHealthSnapshot: {
    fields: providerHealthSnapshotFields,
  },
};
