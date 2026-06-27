#!/usr/bin/env node
import { mkdirSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadGeneratorYaml } from '../../workspace-sdk-generator-root-shared.mjs';
import { applySdkworkV3OpenApiStandard } from '../../workspace-openapi-v3-standard.mjs';

const workspaceRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const outputPath = path.join(workspaceRoot, 'openapi', 'sdkwork-im-im.openapi.yaml');
const apiPrefix = '/im/v3/api';

const ref = (name) => ({ $ref: `#/components/schemas/${name}` });
const arrayOf = (schema) => ({ type: 'array', items: schema });
const stringSchema = (extra = {}) => ({ type: 'string', ...extra });
const boolSchema = () => ({ type: 'boolean' });
const intSchema = (extra = {}) => ({ type: 'integer', format: 'int64', ...extra });
const int32Schema = (extra = {}) => ({ type: 'integer', format: 'int32', ...extra });
const sequenceSchema = (extra = {}) => int32Schema({ minimum: 0, ...extra });
const objectSchema = (properties, required = [], extra = {}) => ({
  type: 'object',
  additionalProperties: false,
  properties,
  ...(required.length > 0 ? { required } : {}),
  ...extra,
});
const mapSchema = () => ({ type: 'object', additionalProperties: true });
const nullable = (schema) => ({ ...schema, nullable: true });

function parameter(name, location, schema, extra = {}) {
  return {
    name,
    in: location,
    required: location === 'path',
    schema,
    ...extra,
  };
}

function okResponse(schemaName, description = 'OK') {
  return {
    description,
    content: {
      'application/json': {
        schema: ref(schemaName),
      },
    },
  };
}

function errorResponses(statuses = ['400', '401', '403', '404']) {
  return Object.fromEntries(statuses.map((status) => [status, { description: `HTTP ${status} problem` }]));
}

function operation({
  tag,
  operationId,
  summary,
  parameters = [],
  request,
  response = 'AckResponse',
  statuses,
}) {
  return {
    tags: [tag],
    operationId,
    summary,
    ...(parameters.length > 0 ? { parameters } : {}),
    ...(request
      ? {
          requestBody: {
            required: true,
            content: {
              'application/json': {
                schema: ref(request),
              },
            },
          },
        }
      : {}),
    responses: {
      '200': okResponse(response),
      ...errorResponses(statuses),
    },
  };
}

function pathItem(pathSuffix, item) {
  return [`${apiPrefix}${pathSuffix}`, item];
}

const pathParameters = {
  ConversationIdPath: parameter('conversationId', 'path', stringSchema()),
  FavoriteIdPath: parameter('favoriteId', 'path', stringSchema()),
  FriendshipIdPath: parameter('friendshipId', 'path', stringSchema()),
  MessageIdPath: parameter('messageId', 'path', stringSchema()),
  RequestIdPath: parameter('requestId', 'path', stringSchema()),
  RoomIdPath: parameter('roomId', 'path', stringSchema()),
  RtcSessionIdPath: parameter('rtcSessionId', 'path', stringSchema()),
  StreamIdPath: parameter('streamId', 'path', stringSchema()),
  TagIdPath: parameter('tagId', 'path', stringSchema()),
  TargetUserIdPath: parameter('targetUserId', 'path', stringSchema()),
};

const queryParameters = {
  AfterSeqQuery: parameter('afterSeq', 'query', sequenceSchema(), { required: false }),
  CursorQuery: parameter('cursor', 'query', stringSchema(), { required: false }),
  DirectionQuery: parameter('direction', 'query', stringSchema({ enum: ['incoming', 'outgoing'] }), { required: false }),
  FavoriteTypeQuery: parameter('favoriteType', 'query', { $ref: '#/components/schemas/MessageFavoriteType' }, { required: false }),
  LimitQuery: parameter('limit', 'query', { type: 'integer', format: 'int32', minimum: 1, maximum: 200 }, { required: false }),
  QQuery: parameter('q', 'query', stringSchema({ maxLength: 256 }), { required: false }),
  StatusQuery: parameter('status', 'query', stringSchema({ enum: ['pending', 'accepted', 'declined', 'canceled', 'expired', 'all'] }), { required: false }),
};

const p = (name) => ({ $ref: `#/components/parameters/${name}` });

const schemas = {
  AckResponse: objectSchema({
    ok: boolSchema(),
  }, ['ok']),
  PresenceHeartbeatRequest: objectSchema({
    deviceId: nullable(stringSchema()),
  }),
  PresenceView: objectSchema({
    tenantId: stringSchema(),
    principalId: stringSchema(),
    principalKind: stringSchema(),
    deviceId: stringSchema(),
    status: stringSchema(),
    updatedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'principalId', 'principalKind', 'deviceId', 'status', 'updatedAt']),
  RealtimeSubscriptionSyncRequest: objectSchema({
    deviceId: nullable(stringSchema()),
    conversations: arrayOf(stringSchema()),
    items: arrayOf(ref('RealtimeSubscriptionItemInput')),
  }),
  RealtimeSubscriptionItemInput: objectSchema({
    scopeType: stringSchema(),
    scopeId: stringSchema(),
    eventTypes: arrayOf(stringSchema()),
  }, ['scopeType', 'scopeId']),
  RealtimeSubscriptionSyncResponse: objectSchema({
    subscriptions: arrayOf(stringSchema()),
  }, ['subscriptions']),
  RealtimeWebSocketHandshake: objectSchema({
    endpoint: stringSchema(),
    protocol: stringSchema(),
  }, ['endpoint', 'protocol']),
  RealtimeEventAckRequest: objectSchema({
    eventIds: arrayOf(stringSchema()),
  }, ['eventIds']),
  RealtimeEventView: objectSchema({
    eventId: stringSchema(),
    scope: stringSchema(),
    scopeId: stringSchema(),
    eventType: stringSchema(),
    payload: nullable(stringSchema()),
    occurredAt: stringSchema({ format: 'date-time' }),
  }, ['eventId', 'scope', 'scopeId', 'eventType', 'occurredAt']),
  RealtimeEventsResponse: objectSchema({
    items: arrayOf(ref('RealtimeEventView')),
    nextCursor: nullable(stringSchema()),
    hasMore: boolSchema(),
  }, ['items', 'hasMore']),
  RtcSession: objectSchema({
    tenantId: stringSchema(),
    rtcSessionId: stringSchema(),
    conversationId: nullable(stringSchema()),
    initiatorId: stringSchema(),
    initiatorKind: stringSchema(),
    providerPluginId: nullable(stringSchema()),
    providerSessionId: nullable(stringSchema()),
    accessEndpoint: nullable(stringSchema()),
    providerRegion: nullable(stringSchema()),
    rtcMode: stringSchema(),
    state: stringSchema(),
    signalingStreamId: nullable(stringSchema()),
    artifactMessageId: nullable(stringSchema()),
    startedAt: stringSchema({ format: 'date-time' }),
    endedAt: nullable(stringSchema({ format: 'date-time' })),
  }, ['tenantId', 'rtcSessionId', 'rtcMode', 'initiatorId', 'initiatorKind', 'state', 'startedAt']),
  CreateRtcSessionRequest: objectSchema({
    rtcSessionId: stringSchema(),
    conversationId: nullable(stringSchema()),
    rtcMode: stringSchema(),
  }, ['rtcSessionId', 'rtcMode']),
  InviteRtcSessionRequest: objectSchema({
    signalingStreamId: nullable(stringSchema()),
  }),
  UpdateRtcSessionRequest: objectSchema({
    artifactMessageId: nullable(stringSchema()),
  }),
  PostRtcSignalRequest: objectSchema({
    signalType: stringSchema(),
    schemaRef: nullable(stringSchema()),
    payload: stringSchema(),
    signalingStreamId: nullable(stringSchema()),
  }, ['signalType', 'payload']),
  IssueRtcParticipantCredentialRequest: objectSchema({
    participantId: stringSchema(),
  }, ['participantId']),
  RtcSessionMutationResponse: objectSchema({
    tenantId: stringSchema(),
    rtcSessionId: stringSchema(),
    conversationId: nullable(stringSchema()),
    initiatorId: stringSchema(),
    initiatorKind: stringSchema(),
    providerPluginId: nullable(stringSchema()),
    providerSessionId: nullable(stringSchema()),
    accessEndpoint: nullable(stringSchema()),
    providerRegion: nullable(stringSchema()),
    rtcMode: stringSchema(),
    state: stringSchema(),
    signalingStreamId: nullable(stringSchema()),
    artifactMessageId: nullable(stringSchema()),
    startedAt: stringSchema({ format: 'date-time' }),
    endedAt: nullable(stringSchema({ format: 'date-time' })),
    requestKey: stringSchema(),
    deliveryStatus: stringSchema({ enum: ['applied', 'replayed'] }),
    proofVersion: stringSchema(),
  }, ['tenantId', 'rtcSessionId', 'rtcMode', 'initiatorId', 'initiatorKind', 'state', 'startedAt', 'requestKey', 'deliveryStatus', 'proofVersion']),
  RtcSignalSender: objectSchema({
    id: stringSchema(),
    kind: stringSchema(),
    memberId: nullable(stringSchema()),
    deviceId: nullable(stringSchema()),
    sessionId: nullable(stringSchema()),
    metadata: mapSchema(),
  }, ['id', 'kind', 'metadata']),
  RtcSignalEvent: objectSchema({
    tenantId: stringSchema(),
    rtcSessionId: stringSchema(),
    signalSeq: intSchema({ minimum: 0 }),
    conversationId: nullable(stringSchema()),
    rtcMode: stringSchema(),
    signalType: stringSchema(),
    schemaRef: nullable(stringSchema()),
    payload: stringSchema(),
    sender: ref('RtcSignalSender'),
    signalingStreamId: nullable(stringSchema()),
    occurredAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'rtcSessionId', 'signalSeq', 'rtcMode', 'signalType', 'payload', 'sender', 'occurredAt']),
  RtcParticipantCredential: objectSchema({
    tenantId: stringSchema(),
    rtcSessionId: stringSchema(),
    participantId: stringSchema(),
    credential: stringSchema(),
    expiresAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'rtcSessionId', 'participantId', 'credential', 'expiresAt']),
  Sender: objectSchema({
    id: stringSchema(),
    kind: stringSchema(),
    principalId: nullable(stringSchema()),
    principalKind: nullable(stringSchema()),
    displayName: nullable(stringSchema()),
    avatarUrl: nullable(stringSchema()),
  }, ['id', 'kind']),
  MessageReplyReference: objectSchema({
    messageId: stringSchema(),
    senderDisplayName: stringSchema(),
    contentPreview: stringSchema(),
  }, ['messageId', 'senderDisplayName', 'contentPreview']),
  MessageType: {
    type: 'string',
    enum: ['standard', 'system', 'signal'],
  },
  MediaKind: {
    type: 'string',
    enum: ['image', 'file', 'audio', 'video', 'link', 'voice', 'document'],
  },
  MediaSource: {
    type: 'string',
    enum: ['drive'],
  },
  DriveReference: objectSchema({
    driveUri: stringSchema(),
    spaceId: stringSchema(),
    nodeId: stringSchema(),
    nodeVersion: nullable(stringSchema()),
  }, ['driveUri', 'spaceId', 'nodeId']),
  MediaResource: objectSchema({
    id: nullable(stringSchema()),
    kind: nullable(ref('MediaKind')),
    mediaKind: nullable(ref('MediaKind')),
    source: ref('MediaSource'),
    uri: stringSchema(),
    publicUrl: nullable(stringSchema()),
    url: nullable(stringSchema()),
    name: nullable(stringSchema()),
    title: nullable(stringSchema()),
    fileName: nullable(stringSchema()),
    mimeType: nullable(stringSchema()),
    size: nullable(intSchema({ minimum: 0 })),
    sizeBytes: nullable(stringSchema()),
    fileSize: nullable(stringSchema()),
    durationSeconds: nullable(int32Schema({ minimum: 0 })),
    poster: nullable(ref('MediaResource')),
    thumbnails: arrayOf(ref('MediaResource')),
  }, ['source', 'uri']),
  ContentPart: objectSchema({
    kind: stringSchema(),
    text: nullable(stringSchema()),
    schemaRef: nullable(stringSchema()),
    encoding: nullable(stringSchema()),
    payload: nullable(stringSchema()),
    drive: nullable(ref('DriveReference')),
    resource: nullable(ref('MediaResource')),
    mediaRole: nullable(stringSchema()),
    signalType: nullable(stringSchema()),
    streamId: nullable(stringSchema()),
    streamType: nullable(stringSchema()),
    state: nullable(stringSchema()),
  }, ['kind']),
  MessageBody: objectSchema({
    text: nullable(stringSchema()),
    parts: arrayOf(ref('ContentPart')),
    replyTo: nullable(ref('MessageReplyReference')),
    renderHints: mapSchema(),
    summary: nullable(stringSchema()),
    metadata: mapSchema(),
  }, ['parts']),
  TimelineViewEntry: objectSchema({
    tenantId: stringSchema(),
    conversationId: stringSchema(),
    messageId: stringSchema(),
    messageSeq: sequenceSchema(),
    summary: nullable(stringSchema()),
    sender: ref('Sender'),
    body: ref('MessageBody'),
    messageType: ref('MessageType'),
    deliveryMode: stringSchema(),
    clientMsgId: nullable(stringSchema()),
    streamSessionId: nullable(stringSchema()),
    rtcSessionId: nullable(stringSchema()),
    occurredAt: stringSchema({ format: 'date-time' }),
    committedAt: nullable(stringSchema({ format: 'date-time' })),
  }, ['tenantId', 'conversationId', 'messageId', 'messageSeq', 'sender', 'body', 'messageType', 'deliveryMode', 'occurredAt']),
  TimelineResponse: objectSchema({
    items: arrayOf(ref('TimelineViewEntry')),
    nextAfterSeq: nullable(sequenceSchema()),
    hasMore: boolSchema(),
  }, ['items', 'hasMore']),
  PostMessageRequest: objectSchema({
    text: nullable(stringSchema()),
    parts: arrayOf(ref('ContentPart')),
    replyTo: nullable(ref('MessageReplyReference')),
    clientMsgId: nullable(stringSchema()),
    summary: nullable(stringSchema()),
    renderHints: mapSchema(),
  }),
  EditMessageRequest: objectSchema({
    text: nullable(stringSchema()),
    parts: arrayOf(ref('ContentPart')),
    replyTo: nullable(ref('MessageReplyReference')),
  }),
  PostedMessageResponse: objectSchema({
    conversationId: stringSchema(),
    messageId: stringSchema(),
    messageSeq: sequenceSchema(),
    body: ref('MessageBody'),
    occurredAt: stringSchema({ format: 'date-time' }),
  }, ['conversationId', 'messageId', 'messageSeq', 'body', 'occurredAt']),
  MessageReactionRequest: objectSchema({
    reactionKey: stringSchema({ maxLength: 32 }),
  }, ['reactionKey']),
  MessageReactionCountView: objectSchema({
    reactionKey: stringSchema(),
    count: int32Schema({ minimum: 0 }),
  }, ['reactionKey', 'count']),
  InteractionActorView: objectSchema({
    id: stringSchema(),
    kind: stringSchema(),
  }, ['id', 'kind']),
  MessagePinView: objectSchema({
    pinnedBy: ref('InteractionActorView'),
    pinnedAt: stringSchema({ format: 'date-time' }),
  }, ['pinnedBy', 'pinnedAt']),
  MessageInteractionSummaryView: objectSchema({
    tenantId: stringSchema(),
    conversationId: stringSchema(),
    messageId: stringSchema(),
    messageSeq: sequenceSchema(),
    totalReactionCount: int32Schema({ minimum: 0 }),
    reactionCounts: arrayOf(ref('MessageReactionCountView')),
    pin: nullable(ref('MessagePinView')),
  }, ['tenantId', 'conversationId', 'messageId', 'messageSeq', 'totalReactionCount', 'reactionCounts']),
  MessageReactionMutationResult: objectSchema({
    tenantId: stringSchema(),
    conversationId: stringSchema(),
    messageId: stringSchema(),
    reactionKey: stringSchema(),
    count: int32Schema({ minimum: 0 }),
    updatedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'conversationId', 'messageId', 'reactionKey', 'count', 'updatedAt']),
  MessagePinMutationResult: objectSchema({
    tenantId: stringSchema(),
    conversationId: stringSchema(),
    messageId: stringSchema(),
    isPinned: boolSchema(),
    updatedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'conversationId', 'messageId', 'isPinned', 'updatedAt']),
  MessageVisibilityMutationResult: objectSchema({
    tenantId: stringSchema(),
    conversationId: stringSchema(),
    messageId: stringSchema(),
    messageSeq: sequenceSchema(),
    principalKind: stringSchema(),
    principalId: stringSchema(),
    isDeleted: boolSchema(),
    updatedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'conversationId', 'messageId', 'messageSeq', 'principalKind', 'principalId', 'isDeleted', 'updatedAt']),
  MessageFavoriteType: {
    type: 'string',
    enum: ['link', 'image', 'file', 'chat'],
  },
  FavoriteMessageRequest: objectSchema({
    conversationId: stringSchema(),
    favoriteType: ref('MessageFavoriteType'),
    title: stringSchema({ maxLength: 256 }),
    contentPreview: stringSchema({ maxLength: 1024 }),
    sourceDisplayName: stringSchema({ maxLength: 128 }),
  }, ['conversationId', 'favoriteType', 'title', 'contentPreview', 'sourceDisplayName']),
  MessageFavoriteView: objectSchema({
    tenantId: stringSchema(),
    principalKind: stringSchema(),
    principalId: stringSchema(),
    favoriteId: stringSchema(),
    favoriteType: ref('MessageFavoriteType'),
    conversationId: stringSchema(),
    messageId: stringSchema(),
    messageSeq: sequenceSchema(),
    title: stringSchema(),
    contentPreview: stringSchema(),
    sourceDisplayName: stringSchema(),
    favoritedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'principalKind', 'principalId', 'favoriteId', 'favoriteType', 'conversationId', 'messageId', 'messageSeq', 'title', 'contentPreview', 'sourceDisplayName', 'favoritedAt']),
  FavoriteMessagesResponse: objectSchema({
    items: arrayOf(ref('MessageFavoriteView')),
    nextCursor: nullable(stringSchema()),
    hasMore: boolSchema(),
  }, ['items', 'hasMore']),
  DeleteMessageFavoriteResponse: objectSchema({
    favoriteId: stringSchema(),
    deleted: boolSchema(),
  }, ['favoriteId', 'deleted']),
  ConversationPreferencesView: objectSchema({
    tenantId: stringSchema(),
    conversationId: stringSchema(),
    principalKind: stringSchema(),
    principalId: stringSchema(),
    isPinned: boolSchema(),
    isMuted: boolSchema(),
    isMarkedUnread: boolSchema(),
    isHidden: boolSchema(),
    updatedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'conversationId', 'principalKind', 'principalId', 'isPinned', 'isMuted', 'isMarkedUnread', 'isHidden', 'updatedAt']),
  UpdateConversationPreferencesRequest: objectSchema({
    isPinned: boolSchema(),
    isMuted: boolSchema(),
    isMarkedUnread: boolSchema(),
    isHidden: boolSchema(),
  }),
  ConversationProfileView: objectSchema({
    tenantId: stringSchema(),
    conversationId: stringSchema(),
    displayName: stringSchema(),
    avatarUrl: stringSchema(),
    notice: stringSchema(),
    updatedAt: stringSchema({ format: 'date-time' }),
    updatedByPrincipalKind: nullable(stringSchema()),
    updatedByPrincipalId: nullable(stringSchema()),
  }, ['tenantId', 'conversationId', 'displayName', 'avatarUrl', 'notice', 'updatedAt']),
  UpdateConversationProfileRequest: objectSchema({
    displayName: stringSchema({ maxLength: 128 }),
    avatarUrl: stringSchema({ maxLength: 512 }),
    notice: stringSchema({ maxLength: 1024 }),
  }),
  ConversationSummaryView: objectSchema({
    tenantId: stringSchema(),
    conversationId: stringSchema(),
    messageCount: int32Schema({ minimum: 0 }),
    lastMessageSeq: sequenceSchema(),
    lastSummary: nullable(stringSchema()),
    lastMessageAt: nullable(stringSchema({ format: 'date-time' })),
  }, ['tenantId', 'conversationId', 'messageCount', 'lastMessageSeq']),
  ConversationInboxPeerView: objectSchema({
    principalKind: stringSchema(),
    principalId: stringSchema(),
    userId: nullable(stringSchema()),
    chatId: nullable(stringSchema()),
    displayName: nullable(stringSchema()),
    avatarUrl: nullable(stringSchema()),
    relationshipState: nullable(stringSchema()),
  }, ['principalKind', 'principalId']),
  ConversationInboxPreferencesView: objectSchema({
    isPinned: boolSchema(),
    isMuted: boolSchema(),
    isMarkedUnread: boolSchema(),
    isHidden: boolSchema(),
  }, ['isPinned', 'isMuted', 'isMarkedUnread', 'isHidden']),
  ConversationInboxEntry: objectSchema({
    tenantId: stringSchema(),
    conversationId: stringSchema(),
    agentHandoff: boolSchema(),
    conversationType: stringSchema(),
    displayName: nullable(stringSchema()),
    avatarUrl: nullable(stringSchema()),
    displaySource: nullable(stringSchema()),
    peer: ref('ConversationInboxPeerView'),
    preferences: ref('ConversationInboxPreferencesView'),
    lastActivityAt: stringSchema({ format: 'date-time' }),
    lastMessageId: nullable(stringSchema()),
    lastSenderId: nullable(stringSchema()),
    messageCount: int32Schema({ minimum: 0 }),
    lastMessageSeq: sequenceSchema(),
    lastSummary: nullable(stringSchema()),
    lastMessageAt: nullable(stringSchema({ format: 'date-time' })),
    unreadCount: int32Schema({ minimum: 0 }),
  }, ['tenantId', 'conversationId', 'conversationType', 'lastActivityAt', 'messageCount', 'lastMessageSeq', 'unreadCount']),
  InboxResponse: objectSchema({
    items: arrayOf(ref('ConversationInboxEntry')),
    nextCursor: nullable(stringSchema()),
    hasMore: boolSchema(),
  }, ['items', 'hasMore']),
  ContactView: objectSchema({
    tenantId: stringSchema(),
    ownerUserId: stringSchema(),
    targetUserId: stringSchema(),
    contactType: stringSchema(),
    relationshipState: stringSchema(),
    friendshipId: stringSchema(),
    directChatId: nullable(stringSchema()),
    conversationId: nullable(stringSchema()),
    establishedAt: stringSchema({ format: 'date-time' }),
    lastInteractionAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'ownerUserId', 'targetUserId', 'contactType', 'relationshipState', 'friendshipId', 'establishedAt', 'lastInteractionAt']),
  ContactsResponse: objectSchema({
    items: arrayOf(ref('ContactView')),
    nextCursor: nullable(stringSchema()),
    hasMore: boolSchema(),
  }, ['items', 'hasMore']),
  ContactPreferencesView: objectSchema({
    tenantId: stringSchema(),
    ownerUserId: stringSchema(),
    targetUserId: stringSchema(),
    isStarred: boolSchema(),
    remark: stringSchema(),
    isBlocked: boolSchema(),
    updatedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'ownerUserId', 'targetUserId', 'isStarred', 'remark', 'isBlocked', 'updatedAt']),
  UpdateContactPreferencesRequest: objectSchema({
    isStarred: boolSchema(),
    remark: { maxLength: 256, type: 'string' },
    isBlocked: boolSchema(),
  }),
  ContactTagView: objectSchema({
    tenantId: stringSchema(),
    ownerUserId: stringSchema(),
    tagId: stringSchema(),
    name: stringSchema(),
    color: stringSchema(),
    count: intSchema({ format: 'int32', minimum: 0 }),
    bg: stringSchema(),
    border: stringSchema(),
    createdAt: stringSchema({ format: 'date-time' }),
    updatedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'ownerUserId', 'tagId', 'name', 'color', 'count', 'bg', 'border', 'createdAt', 'updatedAt']),
  ContactTagsResponse: objectSchema({
    items: arrayOf(ref('ContactTagView')),
    nextCursor: nullable(stringSchema()),
    hasMore: boolSchema(),
  }, ['items', 'hasMore']),
  CreateContactTagRequest: objectSchema({
    name: stringSchema({ maxLength: 128 }),
    color: stringSchema({ maxLength: 64 }),
    count: intSchema({ format: 'int32', minimum: 0 }),
    bg: stringSchema({ maxLength: 128 }),
    border: stringSchema({ maxLength: 128 }),
  }, ['name', 'color']),
  UpdateContactTagRequest: objectSchema({
    name: stringSchema({ maxLength: 128 }),
    color: stringSchema({ maxLength: 64 }),
    count: intSchema({ format: 'int32', minimum: 0 }),
    bg: stringSchema({ maxLength: 128 }),
    border: stringSchema({ maxLength: 128 }),
  }),
  DeleteContactTagResponse: objectSchema({
    tagId: stringSchema(),
    deleted: boolSchema(),
  }, ['tagId', 'deleted']),
  ContactRecommendationView: objectSchema({
    tenantId: stringSchema(),
    ownerUserId: stringSchema(),
    targetUserId: stringSchema(),
    recommendationId: stringSchema(),
    targetConversationId: nullable(stringSchema()),
    createdAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'ownerUserId', 'targetUserId', 'recommendationId', 'createdAt']),
  CreateContactRecommendationRequest: objectSchema({
    targetConversationId: stringSchema({ maxLength: 128 }),
  }),
  SocialUserSearchResult: objectSchema({
    tenantId: stringSchema(),
    userId: stringSchema(),
    chatId: stringSchema({ minLength: 6, maxLength: 24, pattern: '^[a-z][a-z0-9]{5,23}$' }),
    displayName: stringSchema(),
    relationshipState: stringSchema(),
    avatarUrl: nullable(stringSchema()),
    email: nullable(stringSchema()),
    phone: nullable(stringSchema()),
    metadata: mapSchema(),
  }, ['tenantId', 'userId', 'chatId', 'displayName', 'relationshipState']),
  SocialUserSearchResponse: objectSchema({
    items: arrayOf(ref('SocialUserSearchResult')),
    nextCursor: nullable(stringSchema()),
    hasMore: boolSchema(),
  }, ['items', 'hasMore']),
  SubmitFriendRequestRequest: objectSchema({
    targetUserId: stringSchema(),
    requestMessage: nullable(stringSchema({ maxLength: 256 })),
  }, ['targetUserId']),
  FriendRequest: objectSchema({
    tenantId: stringSchema(),
    requestId: stringSchema(),
    requesterUserId: stringSchema(),
    targetUserId: stringSchema(),
    status: stringSchema(),
    requestMessage: nullable(stringSchema()),
    createdAt: stringSchema({ format: 'date-time' }),
    updatedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'requestId', 'requesterUserId', 'targetUserId', 'status', 'createdAt', 'updatedAt']),
  Friendship: objectSchema({
    tenantId: stringSchema(),
    friendshipId: stringSchema(),
    initiatorUserId: stringSchema(),
    leftUserId: stringSchema(),
    rightUserId: stringSchema(),
    userHighId: stringSchema(),
    userLowId: stringSchema(),
    status: stringSchema(),
    createdAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'friendshipId', 'initiatorUserId', 'leftUserId', 'rightUserId', 'userHighId', 'userLowId', 'status', 'createdAt']),
  DirectChat: objectSchema({
    tenantId: stringSchema(),
    directChatId: stringSchema(),
    conversationId: stringSchema(),
    status: stringSchema(),
  }, ['tenantId', 'directChatId', 'conversationId', 'status']),
  SocialFriendRequestMutationResponse: objectSchema({
    friendRequest: ref('FriendRequest'),
  }, ['friendRequest']),
  SocialFriendRequestListResponse: objectSchema({
    items: arrayOf(ref('FriendRequest')),
    nextCursor: nullable(stringSchema()),
  }, ['items']),
  SocialFriendRequestAcceptanceResponse: objectSchema({
    friendRequest: ref('FriendRequest'),
    friendship: ref('Friendship'),
    directChat: ref('DirectChat'),
    conversation: ref('CreateConversationResult'),
  }, ['friendRequest', 'friendship', 'directChat', 'conversation']),
  SocialFriendshipMutationResponse: objectSchema({
    friendship: ref('Friendship'),
  }, ['friendship']),
  CreateConversationRequest: objectSchema({
    conversationId: nullable(stringSchema()),
    conversationType: nullable(stringSchema()),
    kind: nullable(stringSchema()),
    title: nullable(stringSchema()),
    memberIds: arrayOf(stringSchema()),
  }),
  CreateAgentDialogRequest: objectSchema({
    agentId: stringSchema(),
    conversationId: nullable(stringSchema()),
    title: nullable(stringSchema()),
  }, ['agentId']),
  BindDirectChatRequest: objectSchema({
    conversationId: nullable(stringSchema()),
    directChatId: nullable(stringSchema()),
    leftActorId: nullable(stringSchema()),
    leftActorKind: nullable(stringSchema()),
    rightActorId: nullable(stringSchema()),
    rightActorKind: nullable(stringSchema()),
    targetUserId: stringSchema(),
  }),
  CreateConversationResult: objectSchema({
    tenantId: stringSchema(),
    conversationId: stringSchema(),
    kind: stringSchema(),
    createdAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'conversationId', 'kind', 'createdAt']),
  CreateRoomRequest: objectSchema({
    conversationId: stringSchema(),
    roomId: stringSchema(),
    roomKind: stringSchema({ enum: ['live', 'chat', 'game'] }),
  }, ['conversationId', 'roomId', 'roomKind']),
  RoomView: objectSchema({
    roomId: stringSchema(),
    roomKind: stringSchema({ enum: ['live', 'chat', 'game'] }),
    conversationId: stringSchema(),
    activeMemberCount: int32Schema({ minimum: 0 }),
    maxMembers: int32Schema({ minimum: 1 }),
  }, ['roomId', 'roomKind', 'conversationId', 'activeMemberCount', 'maxMembers']),
  EnterRoomResponse: objectSchema({
    member: ref('ConversationMember'),
  }, ['member']),
  AddConversationMemberRequest: objectSchema({
    principalId: stringSchema(),
    principalKind: stringSchema(),
    role: stringSchema(),
    attributes: mapSchema(),
  }, ['principalId', 'principalKind', 'role']),
  RemoveConversationMemberRequest: objectSchema({
    memberId: stringSchema(),
  }, ['memberId']),
  TransferConversationOwnerRequest: objectSchema({
    memberId: stringSchema(),
  }, ['memberId']),
  ChangeConversationMemberRoleRequest: objectSchema({
    memberId: stringSchema(),
    role: stringSchema(),
  }, ['memberId', 'role']),
  MembershipState: {
    type: 'string',
    enum: ['joined', 'invited', 'linked', 'left', 'removed'],
  },
  ConversationMember: objectSchema({
    tenantId: stringSchema(),
    conversationId: stringSchema(),
    memberId: stringSchema(),
    principalId: stringSchema(),
    principalKind: stringSchema(),
    role: stringSchema(),
    state: ref('MembershipState'),
    joinedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'conversationId', 'memberId', 'principalId', 'principalKind', 'role', 'state', 'joinedAt']),
  ListMembersResponse: objectSchema({
    items: arrayOf(ref('ConversationMember')),
    nextCursor: nullable(stringSchema()),
    hasMore: boolSchema(),
  }, ['items', 'hasMore']),
  MemberDirectoryResponse: objectSchema({
    items: arrayOf(ref('ConversationMember')),
  }, ['items']),
  ReadCursorView: objectSchema({
    tenantId: stringSchema(),
    conversationId: stringSchema(),
    principalId: stringSchema(),
    readSeq: sequenceSchema(),
    updatedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'conversationId', 'principalId', 'readSeq', 'updatedAt']),
  UpdateReadCursorRequest: objectSchema({
    readSeq: sequenceSchema(),
  }, ['readSeq']),
  PinnedMessagesResponse: objectSchema({
    items: arrayOf(ref('MessageInteractionSummaryView')),
  }, ['items']),
  StreamView: objectSchema({
    tenantId: stringSchema(),
    streamId: stringSchema(),
    state: stringSchema(),
    openedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'streamId', 'state', 'openedAt']),
  OpenStreamRequest: objectSchema({
    streamType: stringSchema(),
    conversationId: nullable(stringSchema()),
  }, ['streamType']),
  StreamFrameView: objectSchema({
    streamId: stringSchema(),
    frameSeq: sequenceSchema(),
    payload: stringSchema(),
    createdAt: stringSchema({ format: 'date-time' }),
  }, ['streamId', 'frameSeq', 'payload', 'createdAt']),
  StreamFramesResponse: objectSchema({
    items: arrayOf(ref('StreamFrameView')),
    nextCursor: nullable(stringSchema()),
    hasMore: boolSchema(),
  }, ['items', 'hasMore']),
  AppendStreamFrameRequest: objectSchema({
    payload: stringSchema(),
  }, ['payload']),
};

const paths = Object.fromEntries([
  pathItem('/presence/heartbeat', {
    post: operation({ tag: 'presence', operationId: 'presence.heartbeat.create', summary: 'Publish current client route presence heartbeat', request: 'PresenceHeartbeatRequest', response: 'PresenceView' }),
  }),
  pathItem('/presence/me', {
    get: operation({ tag: 'presence', operationId: 'presence.me.retrieve', summary: 'Retrieve current principal presence', response: 'PresenceView' }),
  }),
  pathItem('/realtime/subscriptions/sync', {
    post: operation({ tag: 'realtime', operationId: 'realtime.subscriptions.sync', summary: 'Sync realtime subscription targets', request: 'RealtimeSubscriptionSyncRequest', response: 'RealtimeSubscriptionSyncResponse' }),
  }),
  pathItem('/realtime/ws', {
    get: operation({ tag: 'realtime', operationId: 'realtime.ws.connect', summary: 'Upgrade to the IM realtime websocket', response: 'RealtimeWebSocketHandshake' }),
  }),
  pathItem('/realtime/events/ack', {
    post: operation({ tag: 'realtime', operationId: 'realtime.events.ack', summary: 'Acknowledge realtime events', request: 'RealtimeEventAckRequest', response: 'AckResponse' }),
  }),
  pathItem('/realtime/events', {
    get: operation({ tag: 'realtime', operationId: 'realtime.events.list', summary: 'List pending realtime events', parameters: [p('LimitQuery'), p('CursorQuery')], response: 'RealtimeEventsResponse' }),
  }),
  pathItem('/calls/sessions', {
    post: operation({ tag: 'calls', operationId: 'calls.sessions.create', summary: 'Create an IM call signaling session', request: 'CreateRtcSessionRequest', response: 'RtcSessionMutationResponse' }),
  }),
  pathItem('/calls/sessions/{rtcSessionId}', {
    parameters: [p('RtcSessionIdPath')],
    get: operation({ tag: 'calls', operationId: 'calls.sessions.retrieve', summary: 'Retrieve IM call signaling session state', parameters: [p('RtcSessionIdPath')], response: 'RtcSession' }),
  }),
  pathItem('/calls/sessions/{rtcSessionId}/invite', {
    parameters: [p('RtcSessionIdPath')],
    post: operation({ tag: 'calls', operationId: 'calls.sessions.invite', summary: 'Invite participants into an IM call signaling session', parameters: [p('RtcSessionIdPath')], request: 'InviteRtcSessionRequest', response: 'RtcSessionMutationResponse' }),
  }),
  pathItem('/calls/sessions/{rtcSessionId}/accept', {
    parameters: [p('RtcSessionIdPath')],
    post: operation({ tag: 'calls', operationId: 'calls.sessions.accept', summary: 'Accept an IM call signaling session', parameters: [p('RtcSessionIdPath')], request: 'UpdateRtcSessionRequest', response: 'RtcSessionMutationResponse' }),
  }),
  pathItem('/calls/sessions/{rtcSessionId}/reject', {
    parameters: [p('RtcSessionIdPath')],
    post: operation({ tag: 'calls', operationId: 'calls.sessions.reject', summary: 'Reject an IM call signaling session', parameters: [p('RtcSessionIdPath')], request: 'UpdateRtcSessionRequest', response: 'RtcSessionMutationResponse' }),
  }),
  pathItem('/calls/sessions/{rtcSessionId}/end', {
    parameters: [p('RtcSessionIdPath')],
    post: operation({ tag: 'calls', operationId: 'calls.sessions.end', summary: 'End an IM call signaling session', parameters: [p('RtcSessionIdPath')], request: 'UpdateRtcSessionRequest', response: 'RtcSessionMutationResponse' }),
  }),
  pathItem('/calls/sessions/{rtcSessionId}/signals', {
    parameters: [p('RtcSessionIdPath')],
    post: operation({ tag: 'calls', operationId: 'calls.sessions.signals.create', summary: 'Post an IM call signaling event', parameters: [p('RtcSessionIdPath')], request: 'PostRtcSignalRequest', response: 'RtcSignalEvent' }),
  }),
  pathItem('/calls/sessions/{rtcSessionId}/credentials', {
    parameters: [p('RtcSessionIdPath')],
    post: operation({ tag: 'calls', operationId: 'calls.sessions.credentials.create', summary: 'Issue an RTC media participant credential for an IM call', parameters: [p('RtcSessionIdPath')], request: 'IssueRtcParticipantCredentialRequest', response: 'RtcParticipantCredential' }),
  }),
  pathItem('/social/users', {
    get: operation({ tag: 'social', operationId: 'social.users.list', summary: 'Search social users', parameters: [p('QQuery'), p('LimitQuery'), p('CursorQuery')], response: 'SocialUserSearchResponse', statuses: ['400', '401', '403', '503'] }),
  }),
  pathItem('/social/friend_requests', {
    get: operation({ tag: 'social', operationId: 'social.friendRequests.list', summary: 'List friend requests', parameters: [p('DirectionQuery'), p('StatusQuery'), p('LimitQuery'), p('CursorQuery')], response: 'SocialFriendRequestListResponse' }),
    post: operation({ tag: 'social', operationId: 'social.friendRequests.create', summary: 'Create a friend request', request: 'SubmitFriendRequestRequest', response: 'SocialFriendRequestMutationResponse' }),
  }),
  pathItem('/social/friend_requests/{requestId}/accept', {
    parameters: [p('RequestIdPath')],
    post: operation({ tag: 'social', operationId: 'social.friendRequests.accept', summary: 'Accept a friend request', parameters: [p('RequestIdPath')], response: 'SocialFriendRequestAcceptanceResponse' }),
  }),
  pathItem('/social/friend_requests/{requestId}/decline', {
    parameters: [p('RequestIdPath')],
    post: operation({ tag: 'social', operationId: 'social.friendRequests.decline', summary: 'Decline a friend request', parameters: [p('RequestIdPath')], response: 'SocialFriendRequestMutationResponse' }),
  }),
  pathItem('/social/friend_requests/{requestId}/cancel', {
    parameters: [p('RequestIdPath')],
    post: operation({ tag: 'social', operationId: 'social.friendRequests.cancel', summary: 'Cancel a friend request', parameters: [p('RequestIdPath')], response: 'SocialFriendRequestMutationResponse' }),
  }),
  pathItem('/social/friendships/{friendshipId}/remove', {
    parameters: [p('FriendshipIdPath')],
    post: operation({ tag: 'social', operationId: 'social.friendships.remove', summary: 'Remove a friendship', parameters: [p('FriendshipIdPath')], response: 'SocialFriendshipMutationResponse' }),
  }),
  pathItem('/social/contacts/tags', {
    get: operation({ tag: 'social', operationId: 'social.contacts.tags.list', summary: 'List contact tags', parameters: [p('LimitQuery'), p('CursorQuery')], response: 'ContactTagsResponse' }),
    post: operation({ tag: 'social', operationId: 'social.contacts.tags.create', summary: 'Create a contact tag', request: 'CreateContactTagRequest', response: 'ContactTagView' }),
  }),
  pathItem('/social/contacts/tags/{tagId}', {
    parameters: [p('TagIdPath')],
    patch: operation({ tag: 'social', operationId: 'social.contacts.tags.update', summary: 'Update a contact tag', parameters: [p('TagIdPath')], request: 'UpdateContactTagRequest', response: 'ContactTagView' }),
    delete: operation({ tag: 'social', operationId: 'social.contacts.tags.delete', summary: 'Delete a contact tag', parameters: [p('TagIdPath')], response: 'DeleteContactTagResponse' }),
  }),
  pathItem('/social/contacts/{targetUserId}/recommendations', {
    parameters: [p('TargetUserIdPath')],
    post: operation({ tag: 'social', operationId: 'social.contacts.recommendations.create', summary: 'Create a contact recommendation', parameters: [p('TargetUserIdPath')], request: 'CreateContactRecommendationRequest', response: 'ContactRecommendationView' }),
  }),
  pathItem('/social/contacts/{targetUserId}/preferences', {
    parameters: [p('TargetUserIdPath')],
    get: operation({ tag: 'social', operationId: 'social.contacts.preferences.retrieve', summary: 'Retrieve contact preferences', parameters: [p('TargetUserIdPath')], response: 'ContactPreferencesView' }),
    patch: operation({ tag: 'social', operationId: 'social.contacts.preferences.update', summary: 'Update contact preferences', parameters: [p('TargetUserIdPath')], request: 'UpdateContactPreferencesRequest', response: 'ContactPreferencesView' }),
  }),
  pathItem('/chat/contacts', {
    get: operation({ tag: 'chat', operationId: 'contacts.list', summary: 'List IM contacts', parameters: [p('LimitQuery'), p('CursorQuery')], response: 'ContactsResponse' }),
  }),
  pathItem('/chat/inbox', {
    get: operation({ tag: 'chat', operationId: 'inbox.retrieve', summary: 'Retrieve current inbox window', parameters: [p('LimitQuery'), p('CursorQuery')], response: 'InboxResponse' }),
  }),
  pathItem('/chat/conversations', {
    post: operation({ tag: 'chat', operationId: 'conversations.create', summary: 'Create a conversation', request: 'CreateConversationRequest', response: 'CreateConversationResult' }),
  }),
  pathItem('/chat/conversations/agent_dialogs', {
    post: operation({ tag: 'chat', operationId: 'conversations.agentDialogs.create', summary: 'Create an agent dialog', request: 'CreateAgentDialogRequest', response: 'CreateConversationResult' }),
  }),
  pathItem('/chat/conversations/agent_handoffs', {
    post: operation({ tag: 'chat', operationId: 'conversations.agentHandoffs.create', summary: 'Create an agent handoff', request: 'CreateAgentDialogRequest', response: 'AckResponse' }),
  }),
  pathItem('/chat/conversations/system_channels', {
    post: operation({ tag: 'chat', operationId: 'conversations.systemChannels.create', summary: 'Create a system channel', request: 'CreateConversationRequest', response: 'CreateConversationResult' }),
  }),
  pathItem('/chat/conversations/threads', {
    post: operation({ tag: 'chat', operationId: 'conversations.threads.create', summary: 'Create a thread conversation', request: 'CreateConversationRequest', response: 'CreateConversationResult' }),
  }),
  pathItem('/chat/conversations/direct_chats/bindings', {
    post: operation({ tag: 'chat', operationId: 'conversations.directChats.bind', summary: 'Bind a direct chat conversation', request: 'BindDirectChatRequest', response: 'CreateConversationResult' }),
  }),
  pathItem('/chat/conversations/{conversationId}/agent_handoff', {
    parameters: [p('ConversationIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.agentHandoff.retrieve', summary: 'Retrieve agent handoff state', parameters: [p('ConversationIdPath')], response: 'AckResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}/agent_handoff/accept', {
    parameters: [p('ConversationIdPath')],
    post: operation({ tag: 'chat', operationId: 'conversations.agentHandoff.accept', summary: 'Accept agent handoff', parameters: [p('ConversationIdPath')], response: 'AckResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}/agent_handoff/resolve', {
    parameters: [p('ConversationIdPath')],
    post: operation({ tag: 'chat', operationId: 'conversations.agentHandoff.resolve', summary: 'Resolve agent handoff', parameters: [p('ConversationIdPath')], response: 'AckResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}/agent_handoff/close', {
    parameters: [p('ConversationIdPath')],
    post: operation({ tag: 'chat', operationId: 'conversations.agentHandoff.close', summary: 'Close agent handoff', parameters: [p('ConversationIdPath')], response: 'AckResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}', {
    parameters: [p('ConversationIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.retrieve', summary: 'Retrieve conversation summary', parameters: [p('ConversationIdPath')], response: 'ConversationSummaryView' }),
  }),
  pathItem('/chat/conversations/{conversationId}/members', {
    parameters: [p('ConversationIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.members.list', summary: 'List conversation members', parameters: [p('ConversationIdPath'), p('LimitQuery'), p('CursorQuery')], response: 'ListMembersResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}/members/add', {
    parameters: [p('ConversationIdPath')],
    post: operation({ tag: 'chat', operationId: 'conversations.members.add', summary: 'Add a conversation member', parameters: [p('ConversationIdPath')], request: 'AddConversationMemberRequest', response: 'ConversationMember' }),
  }),
  pathItem('/chat/conversations/{conversationId}/members/remove', {
    parameters: [p('ConversationIdPath')],
    post: operation({ tag: 'chat', operationId: 'conversations.members.remove', summary: 'Remove a conversation member', parameters: [p('ConversationIdPath')], request: 'RemoveConversationMemberRequest', response: 'AckResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}/members/transfer_owner', {
    parameters: [p('ConversationIdPath')],
    post: operation({ tag: 'chat', operationId: 'conversations.members.transferOwner', summary: 'Transfer conversation owner', parameters: [p('ConversationIdPath')], request: 'TransferConversationOwnerRequest', response: 'ConversationMember' }),
  }),
  pathItem('/chat/conversations/{conversationId}/members/change_role', {
    parameters: [p('ConversationIdPath')],
    post: operation({ tag: 'chat', operationId: 'conversations.members.changeRole', summary: 'Change conversation member role', parameters: [p('ConversationIdPath')], request: 'ChangeConversationMemberRoleRequest', response: 'ConversationMember' }),
  }),
  pathItem('/chat/conversations/{conversationId}/members/leave', {
    parameters: [p('ConversationIdPath')],
    post: operation({ tag: 'chat', operationId: 'conversations.members.leave', summary: 'Leave a conversation', parameters: [p('ConversationIdPath')], response: 'AckResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}/preferences', {
    parameters: [p('ConversationIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.preferences.retrieve', summary: 'Retrieve conversation preferences', parameters: [p('ConversationIdPath')], response: 'ConversationPreferencesView' }),
    patch: operation({ tag: 'chat', operationId: 'conversations.preferences.update', summary: 'Update conversation preferences', parameters: [p('ConversationIdPath')], request: 'UpdateConversationPreferencesRequest', response: 'ConversationPreferencesView' }),
  }),
  pathItem('/chat/conversations/{conversationId}/profile', {
    parameters: [p('ConversationIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.profile.retrieve', summary: 'Retrieve conversation profile', parameters: [p('ConversationIdPath')], response: 'ConversationProfileView' }),
    patch: operation({ tag: 'chat', operationId: 'conversations.profile.update', summary: 'Update conversation profile', parameters: [p('ConversationIdPath')], request: 'UpdateConversationProfileRequest', response: 'ConversationProfileView' }),
  }),
  pathItem('/chat/conversations/{conversationId}/read_cursor', {
    parameters: [p('ConversationIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.readCursor.retrieve', summary: 'Retrieve read cursor', parameters: [p('ConversationIdPath')], response: 'ReadCursorView' }),
    post: operation({ tag: 'chat', operationId: 'conversations.readCursor.update', summary: 'Update read cursor', parameters: [p('ConversationIdPath')], request: 'UpdateReadCursorRequest', response: 'ReadCursorView' }),
  }),
  pathItem('/chat/conversations/{conversationId}/member_directory', {
    parameters: [p('ConversationIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.memberDirectory.list', summary: 'List member directory', parameters: [p('ConversationIdPath')], response: 'MemberDirectoryResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}/messages', {
    parameters: [p('ConversationIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.messages.list', summary: 'List conversation message timeline', parameters: [p('ConversationIdPath'), p('AfterSeqQuery'), p('LimitQuery')], response: 'TimelineResponse' }),
    post: operation({ tag: 'chat', operationId: 'conversations.messages.create', summary: 'Post a conversation message', parameters: [p('ConversationIdPath')], request: 'PostMessageRequest', response: 'PostedMessageResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}/system_channel/publish', {
    parameters: [p('ConversationIdPath')],
    post: operation({ tag: 'chat', operationId: 'conversations.systemChannel.publish', summary: 'Publish a system channel message', parameters: [p('ConversationIdPath')], request: 'PostMessageRequest', response: 'PostedMessageResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}/pins', {
    parameters: [p('ConversationIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.pins.list', summary: 'List pinned messages', parameters: [p('ConversationIdPath')], response: 'PinnedMessagesResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}/messages/{messageId}/interaction_summary', {
    parameters: [p('ConversationIdPath'), p('MessageIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.messages.interactionSummary.retrieve', summary: 'Retrieve message interaction summary', parameters: [p('ConversationIdPath'), p('MessageIdPath')], response: 'MessageInteractionSummaryView' }),
  }),
  pathItem('/chat/messages/{messageId}/edit', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.edit', summary: 'Edit a message', parameters: [p('MessageIdPath')], request: 'EditMessageRequest', response: 'PostedMessageResponse' }),
  }),
  pathItem('/chat/messages/{messageId}/recall', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.recall', summary: 'Recall a message', parameters: [p('MessageIdPath')], response: 'PostedMessageResponse' }),
  }),
  pathItem('/chat/messages/favorites', {
    get: operation({ tag: 'chat', operationId: 'messages.favorites.list', summary: 'List message favorites', parameters: [p('LimitQuery'), p('CursorQuery'), p('FavoriteTypeQuery'), p('QQuery')], response: 'FavoriteMessagesResponse' }),
  }),
  pathItem('/chat/messages/{messageId}/favorites', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.favorites.create', summary: 'Favorite a message', parameters: [p('MessageIdPath')], request: 'FavoriteMessageRequest', response: 'MessageFavoriteView' }),
  }),
  pathItem('/chat/messages/favorites/{favoriteId}', {
    parameters: [p('FavoriteIdPath')],
    delete: operation({ tag: 'chat', operationId: 'messages.favorites.delete', summary: 'Delete a message favorite', parameters: [p('FavoriteIdPath')], response: 'DeleteMessageFavoriteResponse' }),
  }),
  pathItem('/chat/messages/{messageId}/visibility', {
    parameters: [p('MessageIdPath')],
    delete: operation({ tag: 'chat', operationId: 'messages.visibility.delete', summary: 'Delete message visibility for the current principal', parameters: [p('MessageIdPath')], response: 'MessageVisibilityMutationResult' }),
  }),
  pathItem('/chat/messages/{messageId}/reactions', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.reactions.create', summary: 'Add a message reaction', parameters: [p('MessageIdPath')], request: 'MessageReactionRequest', response: 'MessageReactionMutationResult' }),
  }),
  pathItem('/chat/messages/{messageId}/reactions/remove', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.reactions.delete', summary: 'Remove a message reaction', parameters: [p('MessageIdPath')], request: 'MessageReactionRequest', response: 'MessageReactionMutationResult' }),
  }),
  pathItem('/chat/messages/{messageId}/pin', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.pin.create', summary: 'Pin a message', parameters: [p('MessageIdPath')], response: 'MessagePinMutationResult' }),
  }),
  pathItem('/chat/messages/{messageId}/unpin', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.pin.delete', summary: 'Unpin a message', parameters: [p('MessageIdPath')], response: 'MessagePinMutationResult' }),
  }),
  pathItem('/chat/rooms', {
    post: operation({ tag: 'chat', operationId: 'rooms.create', summary: 'Create a live, chat, or game room bound to a group conversation', request: 'CreateRoomRequest', response: 'CreateConversationResult', statuses: ['400', '401', '403', '404', '409'] }),
  }),
  pathItem('/chat/rooms/{roomId}', {
    parameters: [p('RoomIdPath')],
    get: operation({ tag: 'chat', operationId: 'rooms.get', summary: 'Get room metadata and active member count', parameters: [p('RoomIdPath')], response: 'RoomView' }),
  }),
  pathItem('/chat/rooms/{roomId}/enter', {
    parameters: [p('RoomIdPath')],
    post: operation({ tag: 'chat', operationId: 'rooms.enter', summary: 'Enter a room as the authenticated principal', parameters: [p('RoomIdPath')], response: 'EnterRoomResponse' }),
  }),
  pathItem('/chat/rooms/{roomId}/leave', {
    parameters: [p('RoomIdPath')],
    post: operation({ tag: 'chat', operationId: 'rooms.leave', summary: 'Leave a room as the authenticated principal', parameters: [p('RoomIdPath')], response: 'EnterRoomResponse' }),
  }),
  pathItem('/streams', {
    post: operation({ tag: 'streams', operationId: 'streams.create', summary: 'Open a stream', request: 'OpenStreamRequest', response: 'StreamView' }),
  }),
  pathItem('/streams/{streamId}/frames', {
    parameters: [p('StreamIdPath')],
    get: operation({ tag: 'streams', operationId: 'streams.frames.list', summary: 'List stream frames', parameters: [p('StreamIdPath'), p('LimitQuery'), p('CursorQuery')], response: 'StreamFramesResponse' }),
    post: operation({ tag: 'streams', operationId: 'streams.frames.create', summary: 'Append a stream frame', parameters: [p('StreamIdPath')], request: 'AppendStreamFrameRequest', response: 'StreamFrameView' }),
  }),
  pathItem('/streams/{streamId}/checkpoint', {
    parameters: [p('StreamIdPath')],
    post: operation({ tag: 'streams', operationId: 'streams.checkpoint.create', summary: 'Checkpoint a stream', parameters: [p('StreamIdPath')], response: 'StreamView' }),
  }),
  pathItem('/streams/{streamId}/complete', {
    parameters: [p('StreamIdPath')],
    post: operation({ tag: 'streams', operationId: 'streams.complete', summary: 'Complete a stream', parameters: [p('StreamIdPath')], response: 'StreamView' }),
  }),
  pathItem('/streams/{streamId}/abort', {
    parameters: [p('StreamIdPath')],
    post: operation({ tag: 'streams', operationId: 'streams.abort', summary: 'Abort a stream', parameters: [p('StreamIdPath')], response: 'StreamView' }),
  }),
]);

const document = {
  openapi: '3.1.0',
  info: {
    title: 'Sdkwork IM IM Standardized Development API',
    version: '0.1.0',
    description: 'IM standardized development OpenAPI contract for conversations, messages, realtime, calls, media, streams, and social IM flows.',
  },
  tags: [
    { name: 'presence' },
    { name: 'realtime' },
    { name: 'calls' },
    { name: 'social' },
    { name: 'chat' },
    { name: 'streams' },
  ],
  paths,
  components: {
    parameters: {
      ...pathParameters,
      ...queryParameters,
    },
    schemas,
  },
};

applySdkworkV3OpenApiStandard(document);
const yaml = await loadGeneratorYaml(workspaceRoot);
mkdirSync(path.dirname(outputPath), { recursive: true });
writeFileSync(outputPath, yaml.dump(document, { noRefs: true, sortKeys: false, lineWidth: 120 }), 'utf8');
console.log(`[sdkwork-im-sdk] materialized ${path.relative(workspaceRoot, outputPath).replaceAll('\\', '/')}`);
