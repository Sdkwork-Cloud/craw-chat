import { field, type ApiSchemaDefinitionMap } from "./schema-types";

export const controlPlaneSocialSchemas: ApiSchemaDefinitionMap = {
  BindDirectChatRequest: {
    fields: [
      field("directChatId", "string", "Direct chat aggregate identifier.", {
        required: true,
      }),
      field("eventId", "string", "Event identifier for the binding mutation.", {
        required: true,
      }),
      field("leftActorId", "string", "Left-side actor identifier.", {
        required: true,
      }),
      field("rightActorId", "string", "Right-side actor identifier.", {
        required: true,
      }),
      field("conversationId", "string", "Conversation identifier bound to the direct chat.", {
        required: true,
      }),
      field("boundAt", "date-time string", "Binding timestamp.", {
        required: true,
      }),
    ],
  },
  EstablishExternalConnectionRequest: {
    fields: [
      field("connectionId", "string", "External collaboration connection identifier.", {
        required: true,
      }),
      field("eventId", "string", "Event identifier for the connection mutation.", {
        required: true,
      }),
      field("externalTenantId", "string", "External tenant identifier.", {
        required: true,
      }),
      field("connectionKind", "string", "Connection kind negotiated for the external tenant.", {
        required: true,
      }),
      field("externalOrgName", "string | null", "Optional display name for the external organization."),
      field("establishedAt", "date-time string", "Connection establishment timestamp.", {
        required: true,
      }),
    ],
  },
  BindExternalMemberLinkRequest: {
    fields: [
      field("linkId", "string", "External member link aggregate identifier.", {
        required: true,
      }),
      field("eventId", "string", "Event identifier for the link mutation.", {
        required: true,
      }),
      field("connectionId", "string", "Owning external connection identifier.", {
        required: true,
      }),
      field("localActorId", "string", "Local actor identifier.", {
        required: true,
      }),
      field("localActorKind", "string", "Local actor kind.", {
        required: true,
      }),
      field("externalMemberId", "string", "External member identifier.", {
        required: true,
      }),
      field("externalDisplayName", "string | null", "Optional display name returned by the external system."),
      field("linkedAt", "date-time string", "Link creation timestamp.", {
        required: true,
      }),
    ],
  },
  SubmitFriendRequestRequest: {
    fields: [
      field("requestId", "string", "Friend request aggregate identifier.", {
        required: true,
      }),
      field("eventId", "string", "Event identifier for the friend request mutation.", {
        required: true,
      }),
      field("requesterUserId", "string", "Requesting user identifier.", {
        required: true,
      }),
      field("targetUserId", "string", "Target user identifier.", {
        required: true,
      }),
      field("requestMessage", "string | null", "Optional user-visible invitation message."),
      field("requestedAt", "date-time string", "Friend request timestamp.", {
        required: true,
      }),
    ],
  },
  ActivateFriendshipRequest: {
    fields: [
      field("friendshipId", "string", "Friendship aggregate identifier.", {
        required: true,
      }),
      field("eventId", "string", "Event identifier for the friendship activation.", {
        required: true,
      }),
      field("initiatorUserId", "string", "Initiating user identifier.", {
        required: true,
      }),
      field("peerUserId", "string", "Peer user identifier.", {
        required: true,
      }),
      field("directChatId", "string | null", "Direct chat identifier created or attached to the friendship."),
      field("establishedAt", "date-time string", "Friendship activation timestamp.", {
        required: true,
      }),
    ],
  },
  ApplySharedChannelPolicyRequest: {
    fields: [
      field("policyId", "string", "Shared-channel policy aggregate identifier.", {
        required: true,
      }),
      field("eventId", "string", "Event identifier for the policy mutation.", {
        required: true,
      }),
      field("connectionId", "string", "External connection identifier.", {
        required: true,
      }),
      field("channelId", "string", "Channel identifier governed by the policy.", {
        required: true,
      }),
      field("conversationId", "string | null", "Conversation identifier projected from the shared channel."),
      field("policyVersion", "uint64", "Policy version applied to the shared channel.", {
        required: true,
      }),
      field("historyVisibility", "string", "History-visibility mode applied by the policy.", {
        required: true,
      }),
      field("appliedAt", "date-time string", "Policy application timestamp.", {
        required: true,
      }),
    ],
  },
  BlockUserRequest: {
    fields: [
      field("blockId", "string", "User block aggregate identifier.", {
        required: true,
      }),
      field("eventId", "string", "Event identifier for the block mutation.", {
        required: true,
      }),
      field("blockerUserId", "string", "User initiating the block.", {
        required: true,
      }),
      field("blockedUserId", "string", "Blocked user identifier.", {
        required: true,
      }),
      field("directChatId", "string | null", "Direct chat identifier affected by the block."),
      field("scope", "string", "Block scope.", {
        required: true,
      }),
      field("effectiveAt", "date-time string", "Timestamp when the block becomes effective.", {
        required: true,
      }),
      field("expiresAt", "date-time string | null", "Optional block expiration timestamp."),
    ],
  },
  SocialSharedChannelSyncPendingTargetedClaimRequest: {
    fields: [
      field("requestKeys", "string[]", "Pending shared-channel sync request keys to claim.", {
        required: true,
      }),
    ],
  },
  SocialSharedChannelSyncPendingTargetedReleaseRequest: {
    fields: [
      field("requestKeys", "string[]", "Pending shared-channel sync request keys to release.", {
        required: true,
      }),
    ],
  },
  SocialSharedChannelSyncTargetedRepublishRequest: {
    fields: [
      field("requestKeys", "string[]", "Pending shared-channel sync request keys to republish.", {
        required: true,
      }),
    ],
  },
  SocialSharedChannelSyncDeadLetterTargetedRequeueRequest: {
    fields: [
      field("requestKeys", "string[]", "Dead-letter shared-channel sync request keys to requeue.", {
        required: true,
      }),
    ],
  },
  SocialSharedChannelSyncPendingTargetedTakeoverRequest: {
    fields: [
      field("requestKeys", "string[]", "Pending shared-channel sync request keys to take over.", {
        required: true,
      }),
      field(
        "allowLegacyUntracked",
        "boolean",
        "Allow takeover of legacy entries that are missing tracked ownership metadata.",
      ),
    ],
  },
};
