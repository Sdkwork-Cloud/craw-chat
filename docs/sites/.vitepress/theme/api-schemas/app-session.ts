import {
  arrayField,
  field,
  type ApiSchemaDefinitionMap,
  type ApiSchemaField,
} from "./schema-types";

const presenceClientFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("principalId", "string", "Principal identifier.", { required: true }),
  field("deviceId", "string", "Client route key carried by the `deviceId` wire field.", { required: true }),
  field("platform", "string | null", "Client platform label."),
  field("sessionId", "string | null", "Active realtime session identifier."),
  field("status", "string", "Client route presence state. Supported values: online, offline.", {
    required: true,
  }),
  field("lastSyncSeq", "uint64", "Latest realtime sequence acknowledged by the service.", {
    required: true,
  }),
  field("lastResumeAt", "date-time string | null", "Most recent route recovery timestamp."),
  field("lastSeenAt", "date-time string | null", "Most recent heartbeat timestamp."),
];

const presenceSnapshotFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("principalId", "string", "Principal identifier.", { required: true }),
  field("currentDeviceId", "string | null", "Current active client route key carried by the `currentDeviceId` wire field."),
  arrayField("devices", "PresenceClientView", "Presence snapshots for all known client route keys.", presenceClientFields, {
    required: true,
    summary: "View nested fields for client routes",
  }),
];

const realtimeSubscriptionItemInputFields: ApiSchemaField[] = [
  field("scopeType", "string", "Subscription scope type, for example conversation.", {
    required: true,
  }),
  field("scopeId", "string", "Subscription scope identifier.", { required: true }),
  field("eventTypes", "string[]", "Event types to include. An empty array subscribes to all events in scope."),
];

const realtimeSubscriptionFields: ApiSchemaField[] = [
  field("scopeType", "string", "Resolved subscription scope type.", { required: true }),
  field("scopeId", "string", "Resolved subscription scope identifier.", { required: true }),
  field("eventTypes", "string[]", "Subscribed event type list.", { required: true }),
  field("subscribedAt", "date-time string", "Subscription sync timestamp.", { required: true }),
];

const realtimeEventFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("principalId", "string", "Target principal identifier.", { required: true }),
  field("deviceId", "string", "Target client route key carried by the `deviceId` wire field.", { required: true }),
  field("realtimeSeq", "uint64", "Realtime event sequence number.", { required: true }),
  field("scopeType", "string", "Event scope type.", { required: true }),
  field("scopeId", "string", "Event scope identifier.", { required: true }),
  field("eventType", "string", "Event type.", { required: true }),
  field("deliveryClass", "string", "Delivery class. Current implementations commonly use ephemeral.", {
    required: true,
  }),
  field("payload", "string", "Serialized event payload JSON.", { required: true }),
  field("occurredAt", "date-time string", "Event timestamp.", { required: true }),
];

export const appSessionSchemas: ApiSchemaDefinitionMap = {
  PresenceClientView: {
    fields: presenceClientFields,
  },
  PresenceSnapshotView: {
    fields: presenceSnapshotFields,
  },
  PresenceHeartbeatRequest: {
    fields: [field("deviceId", "string | null", "Target client route key carried by the `deviceId` wire field.")],
  },
  RealtimeSubscriptionItemInput: {
    fields: realtimeSubscriptionItemInputFields,
  },
  SyncRealtimeSubscriptionsRequest: {
    fields: [
      field("deviceId", "string | null", "Target client route key carried by the `deviceId` wire field."),
      arrayField(
        "items",
        "RealtimeSubscriptionItemInput",
        "Requested subscription set. Missing or empty means no active subscriptions.",
        realtimeSubscriptionItemInputFields,
        {
          summary: "View nested fields for items",
        },
      ),
    ],
  },
  RealtimeSubscription: {
    fields: realtimeSubscriptionFields,
  },
  RealtimeSubscriptionSnapshot: {
    fields: [
      field("tenantId", "string", "Tenant identifier.", { required: true }),
      field("principalId", "string", "Principal identifier.", { required: true }),
      field("deviceId", "string", "Client route key carried by the `deviceId` wire field.", { required: true }),
      arrayField("items", "RealtimeSubscription", "Resolved realtime subscriptions for the client route.", realtimeSubscriptionFields, {
        required: true,
        summary: "View nested fields for items",
      }),
      field("syncedAt", "date-time string", "Subscription sync timestamp.", { required: true }),
    ],
  },
  RealtimeEvent: {
    fields: realtimeEventFields,
  },
  RealtimeEventWindow: {
    fields: [
      field("deviceId", "string", "Client route key carried by the `deviceId` wire field.", { required: true }),
      arrayField("items", "RealtimeEvent", "Realtime events in the current fetch window.", realtimeEventFields, {
        required: true,
        summary: "View nested fields for items",
      }),
      field("nextAfterSeq", "uint64 | null", "Cursor to use for the next fetch request."),
      field("hasMore", "boolean", "Whether more events are available.", { required: true }),
      field("ackedThroughSeq", "uint64", "Highest acknowledged event sequence.", { required: true }),
      field("trimmedThroughSeq", "uint64", "Highest event sequence already trimmed from retention.", {
        required: true,
      }),
    ],
  },
  AckRealtimeEventsRequest: {
    fields: [
      field("deviceId", "string | null", "Target client route key carried by the `deviceId` wire field."),
      field("ackedSeq", "uint64", "Highest event sequence confirmed by the client.", {
        required: true,
      }),
    ],
  },
  RealtimeAckState: {
    fields: [
      field("tenantId", "string", "Tenant identifier.", { required: true }),
      field("principalId", "string", "Principal identifier.", { required: true }),
      field("deviceId", "string", "Client route key carried by the `deviceId` wire field.", { required: true }),
      field("ackedThroughSeq", "uint64", "Highest acknowledged event sequence.", { required: true }),
      field("trimmedThroughSeq", "uint64", "Highest trimmed event sequence.", { required: true }),
      field("retainedEventCount", "uint64", "Number of retained events still available in the client route window.", {
        required: true,
      }),
      field("ackedAt", "date-time string", "ACK timestamp.", { required: true }),
    ],
  },
};
