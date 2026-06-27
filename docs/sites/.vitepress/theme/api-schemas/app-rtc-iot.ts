import {
  field,
  objectField,
  type ApiSchemaDefinitionMap,
  type ApiSchemaField,
} from "./schema-types";
import { providerHealthSnapshotFields, senderFields } from "./common";

const rtcSessionFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("rtcSessionId", "string", "RTC session identifier.", { required: true }),
  field("conversationId", "string | null", "Conversation identifier bound to the RTC session."),
  field("rtcMode", "string", "RTC mode.", { required: true }),
  field("initiatorId", "string", "Initiating principal identifier.", { required: true }),
  field("providerPluginId", "string | null", "RTC provider plugin identifier."),
  field("providerSessionId", "string | null", "Provider-side session identifier."),
  field("accessEndpoint", "string | null", "Join endpoint."),
  field("providerRegion", "string | null", "Provider region label."),
  field("state", "string", "RTC session state. Supported values: started, accepted, rejected, ended.", {
    required: true,
  }),
  field("signalingStreamId", "string | null", "Associated signaling stream identifier."),
  field("artifactMessageId", "string | null", "Recording artifact message identifier."),
  field("startedAt", "date-time string", "Start timestamp.", { required: true }),
  field("endedAt", "date-time string | null", "End timestamp."),
];

const rtcSignalEventFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("rtcSessionId", "string", "RTC session identifier.", { required: true }),
  field("conversationId", "string | null", "Conversation identifier bound to the RTC session."),
  field("rtcMode", "string", "RTC mode.", { required: true }),
  field("signalType", "string", "Signal type.", { required: true }),
  field("schemaRef", "string | null", "Schema reference for the signaling payload."),
  field("payload", "string", "Serialized signaling payload.", { required: true }),
  objectField("sender", "Sender snapshot.", senderFields, {
    required: true,
    summary: "View nested fields for sender",
  }),
  field("signalingStreamId", "string | null", "Associated signaling stream identifier."),
  field("occurredAt", "date-time string", "Signal timestamp.", { required: true }),
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

export const appRtcIotSchemas: ApiSchemaDefinitionMap = {
  CreateRtcSessionRequest: {
    fields: [
      field("rtcSessionId", "string", "RTC session identifier.", { required: true }),
      field("conversationId", "string | null", "Conversation identifier to bind."),
      field("rtcMode", "string", "RTC mode.", { required: true }),
    ],
  },
  InviteRtcSessionRequest: {
    fields: [field("signalingStreamId", "string | null", "Associated signaling stream identifier.")],
  },
  UpdateRtcSessionRequest: {
    fields: [field("artifactMessageId", "string | null", "Recording artifact message identifier.")],
  },
  PostRtcSignalRequest: {
    fields: [
      field("signalType", "string", "Signal type.", { required: true }),
      field("schemaRef", "string | null", "Schema reference for the signal payload."),
      field("payload", "string", "Serialized signal payload.", { required: true }),
      field("signalingStreamId", "string | null", "Associated signaling stream identifier."),
    ],
  },
  IssueRtcParticipantCredentialRequest: {
    fields: [field("participantId", "string", "Participant identifier.", { required: true })],
  },
  RtcSession: {
    fields: rtcSessionFields,
  },
  RtcSignalEvent: {
    fields: rtcSignalEventFields,
  },
  RtcParticipantCredential: {
    fields: [
      field("tenantId", "string", "Tenant identifier.", { required: true }),
      field("rtcSessionId", "string", "RTC session identifier.", { required: true }),
      field("participantId", "string", "Participant identifier.", { required: true }),
      field("credential", "string", "Issued participant credential.", { required: true }),
      field("expiresAt", "date-time string", "Credential expiry timestamp.", { required: true }),
    ],
  },
  RtcRecordingArtifact: {
    fields: [
      field("tenantId", "string", "Tenant identifier.", { required: true }),
      field("rtcSessionId", "string", "RTC session identifier.", { required: true }),
      field("bucket", "string", "Object storage bucket.", { required: true }),
      field("objectKey", "string", "Object storage key.", { required: true }),
      field("storageProvider", "string | null", "Object storage provider plugin identifier."),
      field("playbackUrl", "string | null", "Signed playback URL."),
    ],
  },
  RtcCallbackRequest: {
    fields: [
      field("rtcSessionId", "string", "RTC session identifier.", { required: true }),
      field("callbackType", "string", "Provider callback type.", { required: true }),
      field("payloadJson", "string", "Raw provider callback JSON.", { required: true }),
    ],
  },
  RtcCallbackEvent: {
    fields: [
      field("rtcSessionId", "string", "RTC session identifier.", { required: true }),
      field("eventType", "string", "Mapped event type.", { required: true }),
      field("participantId", "string | null", "Related participant identifier."),
      field("payloadJson", "string", "Mapped event payload JSON.", { required: true }),
    ],
  },
  IotProtocolUplinkRequest: {
    fields: [
      field("deviceId", "string | null", "Expected device identifier."),
      field("channel", "string", "Protocol channel.", { required: true }),
      field("payload", "string", "Raw uplink payload.", { required: true }),
    ],
  },
  IotProtocolDownlinkRequest: {
    fields: [
      field("deviceId", "string", "Target device identifier.", { required: true }),
      field("channel", "string", "Protocol channel.", { required: true }),
      field("payloadJson", "string", "Structured downlink payload JSON.", { required: true }),
    ],
  },
  IotProtocolDownlinkResponse: {
    fields: [
      objectField("frame", "Frame written to the device command stream.", streamFrameFields, {
        required: true,
        summary: "View nested fields for frame",
      }),
      field("protocolPayload", "string", "Encoded protocol payload emitted downstream.", {
        required: true,
      }),
    ],
  },
  ProviderHealthSnapshot: {
    fields: providerHealthSnapshotFields,
  },
};
