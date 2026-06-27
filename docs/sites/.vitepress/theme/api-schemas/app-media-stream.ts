import {
  arrayField,
  field,
  objectField,
  type ApiSchemaDefinitionMap,
  type ApiSchemaField,
} from "./schema-types";
import { mediaAssetFields, mediaResourceFields, senderFields } from "./common";

const streamSessionFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("streamId", "string", "Stream identifier.", { required: true }),
  field("streamType", "string", "Stream type.", { required: true }),
  field("scopeKind", "string", "Bound scope kind.", { required: true }),
  field("scopeId", "string", "Bound scope identifier.", { required: true }),
  field(
    "durabilityClass",
    "string",
    "Requested durability class. Wire values are transient, durableSession, and eventLog.",
    { required: true },
  ),
  field("orderingScope", "string", "Ordering scope.", { required: true }),
  field("schemaRef", "string | null", "Schema reference for stream payloads."),
  field(
    "state",
    "string",
    "Stream state. Supported values: created, opened, active, checkpointed, completed, aborted, expired.",
    { required: true },
  ),
  field("lastFrameSeq", "uint64", "Latest appended frame sequence.", { required: true }),
  field("lastCheckpointSeq", "uint64 | null", "Latest checkpoint frame sequence."),
  field("resultMessageId", "string | null", "Result message identifier associated with the stream."),
  field("openedAt", "date-time string", "Open timestamp.", { required: true }),
  field("closedAt", "date-time string | null", "Close timestamp."),
  field("expiresAt", "date-time string | null", "Expiry timestamp."),
];

const streamFrameFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("streamId", "string", "Stream identifier.", { required: true }),
  field("streamType", "string", "Stream type.", { required: true }),
  field("scopeKind", "string", "Scope kind.", { required: true }),
  field("scopeId", "string", "Scope identifier.", { required: true }),
  field("frameSeq", "uint64", "Frame sequence number.", { required: true }),
  field("frameType", "string", "Frame type.", { required: true }),
  field("schemaRef", "string | null", "Schema reference for the frame payload."),
  field("encoding", "string", "Frame encoding.", { required: true }),
  field("payload", "string", "Serialized frame payload.", { required: true }),
  objectField("sender", "Sender snapshot.", senderFields, {
    required: true,
    summary: "View nested fields for sender",
  }),
  field("attributes", "Record<string, string>", "Frame attributes.", { required: true }),
  field("occurredAt", "date-time string", "Frame timestamp.", { required: true }),
];

const mediaUploadSessionFields: ApiSchemaField[] = [
  field("assetId", "string", "Media asset identifier.", { required: true }),
  field("storageProvider", "string", "Selected object storage provider plugin.", { required: true }),
  field("bucket", "string", "Object storage bucket.", { required: true }),
  field("objectKey", "string", "Object storage key.", { required: true }),
  field("method", "string", "Presigned upload HTTP method.", { required: true }),
  field("url", "string", "Presigned upload URL.", { required: true }),
  field("headers", "Record<string, string>", "Signed headers required by the upload target.", {
    required: true,
  }),
  field("expiresAt", "date-time string", "Upload session expiry timestamp.", { required: true }),
];

const mediaUploadMutationResponseFields: ApiSchemaField[] = [
  ...mediaAssetFields,
  objectField("upload", "Presigned upload session for direct binary transfer.", mediaUploadSessionFields, {
    summary: "View nested fields for upload",
  }),
  field(
    "requestKey",
    "string",
    "Idempotency proof key for the media mutation request.",
    { required: true },
  ),
  field(
    "deliveryStatus",
    "string",
    "Mutation delivery status. Supported values: applied, replayed.",
    { required: true },
  ),
  field("proofVersion", "string", "Delivery proof contract version.", { required: true }),
];

export const appMediaStreamSchemas: ApiSchemaDefinitionMap = {
  CreateUploadRequest: {
    fields: [
      field("mediaAssetId", "string", "Media asset identifier.", { required: true }),
      field("bucket", "string", "Object storage bucket.", { required: true }),
      field("objectKey", "string | null", "Preferred object storage key."),
      field("expiresInSeconds", "uint32 | null", "Requested presigned upload URL lifetime."),
      objectField("resource", "Media resource payload.", mediaResourceFields, {
        required: true,
        summary: "View nested fields for resource",
      }),
    ],
  },
  CompleteUploadRequest: {
    fields: [
      field("bucket", "string", "Object storage bucket.", { required: true }),
      field("objectKey", "string", "Object storage key.", { required: true }),
      field("storageProvider", "string | null", "Preferred object storage provider plugin."),
      field("url", "string", "Final object URL.", { required: true }),
      field("checksum", "string | null", "Object checksum."),
      field("etag", "string | null", "ETag returned by the object storage upload response."),
    ],
  },
  MediaAsset: {
    fields: mediaAssetFields,
  },
  MediaUploadSession: {
    fields: mediaUploadSessionFields,
  },
  MediaUploadMutationResponse: {
    fields: mediaUploadMutationResponseFields,
  },
  MediaResource: {
    fields: mediaResourceFields,
  },
  MediaDownloadUrlResponse: {
    fields: [
      field("mediaAssetId", "string", "Media asset identifier.", { required: true }),
      field("storageProvider", "string", "Object storage provider plugin.", { required: true }),
      field("downloadUrl", "string", "Signed download URL.", { required: true }),
      field("expiresInSeconds", "uint32", "URL lifetime in seconds.", { required: true }),
    ],
  },
  MediaUploadSessionResponse: {
    fields: [
      field("mediaAssetId", "string", "Media asset identifier.", { required: true }),
      objectField("mediaAsset", "Pending media asset snapshot.", mediaAssetFields, {
        required: true,
        summary: "View nested fields for media asset",
      }),
      field("bucket", "string", "Object storage bucket.", { required: true }),
      field("objectKey", "string", "Resolved object storage key.", { required: true }),
      field("storageProvider", "string", "Resolved object storage provider plugin.", { required: true }),
      field("uploadMethod", "string", "Presigned upload HTTP method.", { required: true }),
      field("uploadUrl", "string", "Presigned upload URL.", { required: true }),
      field("uploadHeaders", "Record<string, string>", "Required headers for the presigned upload.", {
        required: true,
      }),
      field("uploadExpiresInSeconds", "uint32", "Upload URL lifetime in seconds.", { required: true }),
      field("requestKey", "string", "Idempotency proof key for the create-upload request.", {
        required: true,
      }),
      field("deliveryStatus", "string", "Whether the session was newly applied or replayed.", {
        required: true,
      }),
      field("proofVersion", "string", "Delivery-proof schema version.", { required: true }),
    ],
  },
  AttachMediaRequest: {
    fields: [
      field("conversationId", "string", "Target conversation identifier.", { required: true }),
      field("clientMsgId", "string | null", "Client-side idempotency key."),
      field("summary", "string | null", "Message summary."),
      field("text", "string | null", "Optional companion text."),
      field("renderHints", "Record<string, string>", "Client rendering hints."),
    ],
  },
  OpenStreamRequest: {
    fields: [
      field("streamId", "string", "Stream identifier.", { required: true }),
      field("streamType", "string", "Stream type.", { required: true }),
      field("scopeKind", "string", "Bound scope kind.", { required: true }),
      field("scopeId", "string", "Bound scope identifier.", { required: true }),
      field(
        "durabilityClass",
        "string",
        "Requested durability class. Accepted values: transient, durableSession, eventLog.",
        { required: true },
      ),
      field("schemaRef", "string | null", "Schema reference for stream payloads."),
    ],
  },
  AppendStreamFrameRequest: {
    fields: [
      field("frameSeq", "uint64", "Frame sequence number. Must begin at 1 and increase monotonically.", {
        required: true,
      }),
      field("frameType", "string", "Frame type.", { required: true }),
      field("schemaRef", "string | null", "Schema reference for the frame payload."),
      field("encoding", "string", "Frame encoding.", { required: true }),
      field("payload", "string", "Serialized frame payload.", { required: true }),
      field("attributes", "Record<string, string>", "Optional frame attributes."),
    ],
  },
  CheckpointStreamRequest: {
    fields: [field("frameSeq", "uint64", "Frame sequence persisted as the checkpoint.", { required: true })],
  },
  CompleteStreamRequest: {
    fields: [
      field("frameSeq", "uint64", "Final frame sequence.", { required: true }),
      field("resultMessageId", "string | null", "Result message identifier linked to stream completion."),
    ],
  },
  AbortStreamRequest: {
    fields: [
      field("frameSeq", "uint64 | null", "Last known frame sequence when aborting."),
      field("reason", "string | null", "Abort reason."),
    ],
  },
  StreamSession: {
    fields: streamSessionFields,
  },
  StreamFrame: {
    fields: streamFrameFields,
  },
  StreamFrameWindow: {
    fields: [
      arrayField("items", "StreamFrame", "Frames included in the current read window.", streamFrameFields, {
        required: true,
        summary: "View nested fields for items",
      }),
      field("nextAfterFrameSeq", "uint64 | null", "Cursor for the next frame page."),
      field("hasMore", "boolean", "Whether more frames are available.", { required: true }),
    ],
  },
};
