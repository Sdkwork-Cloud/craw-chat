import {
  arrayField,
  field,
  objectField,
  type ApiSchemaDefinitionMap,
  type ApiSchemaField,
} from "./schema-types";
import { contentPartFields } from "./common";

const conversationActorFields: ApiSchemaField[] = [
  field("id", "string", "Principal identifier.", { required: true }),
  field("kind", "string", "Principal kind.", { required: true }),
];

const conversationAgentHandoffFields: ApiSchemaField[] = [
  field("status", "string", "Handoff status. Common values: open, accepted, resolved, closed.", {
    required: true,
  }),
  objectField("source", "Source actor.", conversationActorFields, {
    required: true,
    summary: "View nested fields for source",
  }),
  objectField("target", "Target actor.", conversationActorFields, {
    required: true,
    summary: "View nested fields for target",
  }),
  field("handoffSessionId", "string", "Handoff session identifier.", { required: true }),
  field("handoffReason", "string | null", "Optional handoff reason."),
  field("acceptedAt", "date-time string | null", "Acceptance timestamp."),
  objectField("acceptedBy", "Actor that accepted the handoff.", conversationActorFields, {
    summary: "View nested fields for acceptedBy",
  }),
  field("resolvedAt", "date-time string | null", "Resolution timestamp."),
  objectField("resolvedBy", "Actor that resolved the handoff.", conversationActorFields, {
    summary: "View nested fields for resolvedBy",
  }),
  field("closedAt", "date-time string | null", "Closure timestamp."),
  objectField("closedBy", "Actor that closed the handoff.", conversationActorFields, {
    summary: "View nested fields for closedBy",
  }),
];

const conversationMemberFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("conversationId", "string", "Conversation identifier.", { required: true }),
  field("memberId", "string", "Membership record identifier.", { required: true }),
  field("principalId", "string", "Principal identifier.", { required: true }),
  field("principalKind", "string", "Principal kind.", { required: true }),
  field("role", "string", "Membership role. Supported values: owner, admin, member, guest.", {
    required: true,
  }),
  field("state", "string", "Membership state. Supported values: joined, invited, left, removed.", {
    required: true,
  }),
  field("invitedBy", "string | null", "Inviter principal identifier."),
  field("joinedAt", "date-time string", "Join timestamp.", { required: true }),
  field("removedAt", "date-time string | null", "Removal timestamp."),
  field("attributes", "Record<string, string>", "Membership attributes.", { required: true }),
];

const summarySenderFields: ApiSchemaField[] = [
  field("id", "string", "Sender identifier.", { required: true }),
  field("kind", "string", "Sender kind.", { required: true }),
];

const conversationSummaryFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("conversationId", "string", "Conversation identifier.", { required: true }),
  field("messageCount", "uint64", "Total number of messages.", { required: true }),
  field("lastMessageId", "string | null", "Latest message identifier."),
  field("lastMessageSeq", "uint64", "Latest message sequence number.", { required: true }),
  field("lastSenderId", "string | null", "Latest sender identifier."),
  field("lastSenderKind", "string | null", "Latest sender kind."),
  objectField("lastSender", "Latest sender summary.", summarySenderFields, {
    summary: "View nested fields for lastSender",
  }),
  field("lastSummary", "string | null", "Latest message summary."),
  field("lastMessageAt", "date-time string | null", "Latest message timestamp."),
  objectField("agentHandoff", "Conversation handoff summary.", conversationAgentHandoffFields, {
    summary: "View nested fields for agentHandoff",
  }),
];

const conversationInboxEntryFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("principalId", "string", "Inbox owner identifier.", { required: true }),
  field("memberId", "string", "Membership identifier.", { required: true }),
  field("conversationId", "string", "Conversation identifier.", { required: true }),
  field("conversationType", "string", "Conversation type.", { required: true }),
  field("messageCount", "uint64", "Total number of messages.", { required: true }),
  field("lastMessageId", "string | null", "Latest message identifier."),
  field("lastMessageSeq", "uint64", "Latest message sequence number.", { required: true }),
  field("lastSenderId", "string | null", "Latest sender identifier."),
  field("lastSenderKind", "string | null", "Latest sender kind."),
  field("lastSummary", "string | null", "Latest message summary."),
  field("unreadCount", "uint64", "Unread message count.", { required: true }),
  field("lastActivityAt", "date-time string", "Latest activity timestamp.", { required: true }),
  objectField("agentHandoff", "Conversation handoff summary.", conversationAgentHandoffFields, {
    summary: "View nested fields for agentHandoff",
  }),
];

const handoffActorFields: ApiSchemaField[] = [
  field("id", "string", "Principal identifier.", { required: true }),
  field("kind", "string", "Principal kind.", { required: true }),
];

const agentHandoffStateFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("conversationId", "string", "Conversation identifier.", { required: true }),
  field("status", "string", "Current handoff status.", { required: true }),
  objectField("source", "Source actor.", handoffActorFields, {
    required: true,
    summary: "View nested fields for source",
  }),
  objectField("target", "Target actor.", handoffActorFields, {
    required: true,
    summary: "View nested fields for target",
  }),
  field("handoffSessionId", "string", "Handoff session identifier.", { required: true }),
  field("handoffReason", "string | null", "Optional handoff reason."),
  field("acceptedAt", "date-time string | null", "Acceptance timestamp."),
  objectField("acceptedBy", "Actor that accepted the handoff.", handoffActorFields, {
    summary: "View nested fields for acceptedBy",
  }),
  field("resolvedAt", "date-time string | null", "Resolution timestamp."),
  objectField("resolvedBy", "Actor that resolved the handoff.", handoffActorFields, {
    summary: "View nested fields for resolvedBy",
  }),
  field("closedAt", "date-time string | null", "Closure timestamp."),
  objectField("closedBy", "Actor that closed the handoff.", handoffActorFields, {
    summary: "View nested fields for closedBy",
  }),
];

const conversationReadCursorViewFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("conversationId", "string", "Conversation identifier.", { required: true }),
  field("memberId", "string", "Membership identifier.", { required: true }),
  field("principalId", "string", "Principal identifier.", { required: true }),
  field("readSeq", "uint64", "Highest read message sequence number.", { required: true }),
  field("lastReadMessageId", "string | null", "Last read message identifier."),
  field("updatedAt", "date-time string", "Cursor update timestamp.", { required: true }),
  field("unreadCount", "uint64", "Unread message count.", { required: true }),
];

const timelineViewEntryFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("conversationId", "string", "Conversation identifier.", { required: true }),
  field("messageId", "string", "Message identifier.", { required: true }),
  field("messageSeq", "uint64", "Message sequence number.", { required: true }),
  field("summary", "string | null", "Message summary."),
];

export const appConversationSchemas: ApiSchemaDefinitionMap = {
  ConversationActorView: {
    fields: conversationActorFields,
  },
  ConversationAgentHandoffView: {
    fields: conversationAgentHandoffFields,
  },
  SummarySenderView: {
    fields: summarySenderFields,
  },
  ConversationMember: {
    fields: conversationMemberFields,
  },
  ConversationSummaryView: {
    fields: conversationSummaryFields,
  },
  ConversationInboxEntry: {
    fields: conversationInboxEntryFields,
  },
  InboxResponse: {
    fields: [
      arrayField("items", "ConversationInboxEntry", "Inbox entries visible to the current principal.", conversationInboxEntryFields, {
        required: true,
        summary: "View nested fields for items",
      }),
    ],
  },
  ChangeAgentHandoffStatusView: {
    fields: handoffActorFields,
  },
  AgentHandoffStateView: {
    fields: agentHandoffStateFields,
  },
  CreateConversationRequest: {
    fields: [
      field("conversationId", "string", "Conversation identifier.", { required: true }),
      field(
        "conversationType",
        "string",
        "Conversation type. Common values: group, direct, agent_dialog, agent_handoff, system_channel.",
        { required: true },
      ),
    ],
  },
  CreateAgentDialogRequest: {
    fields: [
      field("conversationId", "string", "Conversation identifier.", { required: true }),
      field("agentId", "string", "Target agent identifier.", { required: true }),
    ],
  },
  CreateAgentHandoffRequest: {
    fields: [
      field("conversationId", "string", "Conversation identifier.", { required: true }),
      field("targetId", "string", "Target principal identifier.", { required: true }),
      field("targetKind", "string", "Target principal kind.", { required: true }),
      field("handoffSessionId", "string", "Handoff session identifier.", { required: true }),
      field("handoffReason", "string | null", "Optional handoff reason."),
    ],
  },
  CreateSystemChannelRequest: {
    fields: [
      field("conversationId", "string", "Conversation identifier.", { required: true }),
      field("subscriberId", "string", "Subscriber principal identifier.", { required: true }),
    ],
  },
  CreateConversationResult: {
    fields: [
      field("conversationId", "string", "Created conversation identifier.", { required: true }),
      field("eventId", "string", "Event identifier emitted by the create operation.", {
        required: true,
      }),
    ],
  },
  ListMembersResponse: {
    fields: [
      arrayField("items", "ConversationMember", "Conversation members.", conversationMemberFields, {
        required: true,
        summary: "View nested fields for items",
      }),
    ],
  },
  AddConversationMemberRequest: {
    fields: [
      field("principalId", "string", "Principal identifier to add.", { required: true }),
      field("principalKind", "string", "Principal kind to add.", { required: true }),
      field("role", "string", "Granted role. Supported values: owner, admin, member, guest.", {
        required: true,
      }),
    ],
  },
  RemoveConversationMemberRequest: {
    fields: [field("memberId", "string", "Membership identifier to remove.", { required: true })],
  },
  TransferConversationOwnerRequest: {
    fields: [field("memberId", "string", "Membership identifier that will become the new owner.", { required: true })],
  },
  TransferConversationOwnerResult: {
    fields: [
      field("eventId", "string", "Ownership transfer event identifier.", { required: true }),
      field("transferredAt", "date-time string", "Ownership transfer timestamp.", {
        required: true,
      }),
      objectField("previousOwner", "Previous owner membership snapshot.", conversationMemberFields, {
        required: true,
        summary: "View nested fields for previousOwner",
      }),
      objectField("newOwner", "New owner membership snapshot.", conversationMemberFields, {
        required: true,
        summary: "View nested fields for newOwner",
      }),
    ],
  },
  ChangeConversationMemberRoleRequest: {
    fields: [
      field("memberId", "string", "Membership identifier to change.", { required: true }),
      field("role", "string", "Target role. Supported values: owner, admin, member, guest.", {
        required: true,
      }),
    ],
  },
  ChangeConversationMemberRoleResult: {
    fields: [
      field("eventId", "string", "Role change event identifier.", { required: true }),
      field("changedAt", "date-time string", "Role change timestamp.", { required: true }),
      objectField("previousMember", "Membership snapshot before the update.", conversationMemberFields, {
        required: true,
        summary: "View nested fields for previousMember",
      }),
      objectField("updatedMember", "Membership snapshot after the update.", conversationMemberFields, {
        required: true,
        summary: "View nested fields for updatedMember",
      }),
    ],
  },
  ConversationReadCursorView: {
    fields: conversationReadCursorViewFields,
  },
  UpdateReadCursorRequest: {
    fields: [
      field("readSeq", "uint64", "Highest message sequence marked as read.", { required: true }),
      field("lastReadMessageId", "string | null", "Last read message identifier."),
    ],
  },
  PostMessageRequest: {
    fields: [
      field("clientMsgId", "string | null", "Client-side idempotency key."),
      field("summary", "string | null", "Message summary."),
      field("text", "string | null", "Convenience plain-text input converted into a text part."),
      arrayField("parts", "ContentPart", "Explicit rich message content parts.", contentPartFields, {
        summary: "View nested fields for parts",
      }),
      field("renderHints", "Record<string, string>", "Client rendering hints."),
    ],
  },
  PostMessageResult: {
    fields: [
      field("messageId", "string", "Message identifier.", { required: true }),
      field("messageSeq", "uint64", "Message sequence number.", { required: true }),
      field("eventId", "string", "Submit event identifier.", { required: true }),
    ],
  },
  EditMessageRequest: {
    fields: [
      field("summary", "string | null", "Updated message summary."),
      field("text", "string | null", "Updated plain-text value."),
      arrayField("parts", "ContentPart", "Updated message content parts.", contentPartFields, {
        summary: "View nested fields for parts",
      }),
      field("renderHints", "Record<string, string>", "Updated client rendering hints."),
    ],
  },
  MessageMutationResult: {
    fields: [
      field("conversationId", "string", "Owning conversation identifier.", { required: true }),
      field("messageId", "string", "Message identifier.", { required: true }),
      field("messageSeq", "uint64", "Message sequence number.", { required: true }),
      field("eventId", "string", "Mutation event identifier.", { required: true }),
    ],
  },
  TimelineViewEntry: {
    fields: timelineViewEntryFields,
  },
  TimelineListResponse: {
    fields: [
      arrayField("items", "TimelineViewEntry", "Timeline entries returned by the projection view.", timelineViewEntryFields, {
        required: true,
        summary: "View nested fields for items",
      }),
    ],
  },
};
