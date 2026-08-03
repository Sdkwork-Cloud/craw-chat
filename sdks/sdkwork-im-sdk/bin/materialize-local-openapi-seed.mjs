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

function successResponse(status, schemaName) {
  if (status === '204') {
    return { description: 'No Content' };
  }
  return okResponse(schemaName, status === '201' ? 'Created' : 'OK');
}

function errorResponses(statuses = ['400', '401', '403', '404']) {
  return Object.fromEntries(statuses.map((status) => [status, { description: `HTTP ${status} problem` }]));
}

function operation({
  tag,
  operationId,
  summary,
  description,
  parameters = [],
  request,
  response = 'AckResponse',
  successStatus = '200',
  statuses,
}) {
  return {
    tags: [tag],
    operationId,
    summary,
    ...(description ? { description } : {}),
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
      [successStatus]: successResponse(successStatus, response),
      ...errorResponses(statuses),
    },
  };
}

function pathItem(pathSuffix, item) {
  return [`${apiPrefix}${pathSuffix}`, item];
}

const pathParameters = {
  BlockIdPath: parameter('blockId', 'path', stringSchema()),
  ConversationIdPath: parameter('conversationId', 'path', stringSchema()),
  FavoriteIdPath: parameter('favoriteId', 'path', stringSchema()),
  FriendshipIdPath: parameter('friendshipId', 'path', stringSchema()),
  MessageIdPath: parameter('messageId', 'path', stringSchema()),
  FriendRequestIdPath: parameter('friendRequestId', 'path', stringSchema()),
  RoomIdPath: parameter('roomId', 'path', stringSchema()),
  RtcSessionIdPath: parameter('rtcSessionId', 'path', stringSchema()),
  StreamIdPath: parameter('streamId', 'path', stringSchema()),
  TagIdPath: parameter('tagId', 'path', stringSchema()),
  TargetUserIdPath: parameter('targetUserId', 'path', stringSchema()),
};

const queryParameters = {
  AfterSignalSeqQuery: parameter('afterSignalSeq', 'query', intSchema({ minimum: 0 }), { required: false }),
  ConversationTypeQuery: parameter('conversation_type', 'query', stringSchema(), {
    description: 'Optional conversation type filter applied by the inbox projection before pagination.',
    required: false,
  }),
  CursorQuery: parameter('cursor', 'query', stringSchema(), { required: false }),
  DirectionQuery: parameter('direction', 'query', stringSchema({ enum: ['incoming', 'outgoing'] }), { required: false }),
  FavoriteTypeQuery: parameter('favoriteType', 'query', { $ref: '#/components/schemas/MessageFavoriteType' }, { required: false }),
  PageSizeQuery: parameter('page_size', 'query', {
    type: 'integer',
    format: 'int32',
    minimum: 1,
    maximum: 200,
    default: 20,
  }, { required: false }),
  QQuery: parameter('q', 'query', stringSchema({ maxLength: 256 }), { required: false }),
  SearchQQuery: parameter('q', 'query', stringSchema({ maxLength: 256 }), { required: true }),
  ConversationIdQuery: parameter('conversationId', 'query', stringSchema(), { required: false }),
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
  RtcSignalEventsResponse: objectSchema({
    items: arrayOf(ref('RtcSignalEvent')),
    nextCursor: nullable(stringSchema()),
    hasMore: boolSchema(),
  }, ['items', 'hasMore']),
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
  ConversationMessageEntry: objectSchema({
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
  ConversationMessageListResponse: {
    allOf: [
      ref('SdkWorkApiResponse'),
      objectSchema({
        data: objectSchema({
          items: arrayOf(ref('ConversationMessageEntry')),
          pageInfo: ref('PageInfo'),
          highWatermark: sequenceSchema(),
        }, ['items', 'pageInfo', 'highWatermark']),
      }, ['data']),
    ],
  },
  MessageSearchHit: objectSchema({
    conversationId: stringSchema(),
    messageId: stringSchema(),
    messageSeq: sequenceSchema(),
  }, ['conversationId', 'messageId', 'messageSeq']),
  MessageSearchResponse: {
    allOf: [
      ref('SdkWorkApiResponse'),
      objectSchema({
        data: objectSchema({
          items: arrayOf(ref('MessageSearchHit')),
          pageInfo: ref('PageInfo'),
        }, ['items', 'pageInfo']),
      }, ['data']),
    ],
  },
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
    summary: nullable(stringSchema()),
    renderHints: mapSchema(),
    idempotencyKey: nullable(stringSchema()),
  }),
  RecallMessageRequest: objectSchema({
    idempotencyKey: nullable(stringSchema()),
  }),
  PostMessageResult: objectSchema({
    messageId: stringSchema(),
    messageSeq: sequenceSchema(),
    eventId: stringSchema(),
    requestKey: stringSchema(),
    deliveryStatus: stringSchema({ enum: ['applied', 'replayed'] }),
    proofVersion: stringSchema(),
  }, ['messageId', 'messageSeq', 'eventId', 'deliveryStatus']),
  MessageMutationResult: objectSchema({
    conversationId: stringSchema(),
    messageId: stringSchema(),
    messageSeq: sequenceSchema(),
    eventId: stringSchema(),
  }, ['conversationId', 'messageId', 'messageSeq', 'eventId']),
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
  ConversationInboxPage: objectSchema({
    items: arrayOf(ref('ConversationInboxEntry')),
    nextCursor: nullable(stringSchema()),
    hasMore: boolSchema(),
  }, ['items', 'hasMore']),
  ContactView: objectSchema({
    tenantId: stringSchema(),
    ownerUserId: stringSchema(),
    targetUserId: stringSchema(),
    displayName: nullable(stringSchema()),
    avatarUrl: nullable(stringSchema()),
    chatId: nullable(stringSchema()),
    contactType: stringSchema(),
    relationshipState: stringSchema(),
    friendshipId: stringSchema(),
    directChatId: nullable(stringSchema()),
    conversationId: nullable(stringSchema()),
    establishedAt: stringSchema({ format: 'date-time' }),
    lastInteractionAt: stringSchema({ format: 'date-time' }),
    isStarred: boolSchema(),
    isBlocked: boolSchema(),
    remark: nullable(stringSchema()),
    updatedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'ownerUserId', 'targetUserId', 'contactType', 'relationshipState', 'friendshipId', 'establishedAt', 'lastInteractionAt', 'isStarred', 'isBlocked', 'updatedAt']),
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
  BlockScope: {
    type: 'string',
    enum: ['all', 'friendship', 'direct_chat'],
  },
  UserBlockStatus: {
    type: 'string',
    enum: ['active', 'released', 'expired'],
  },
  BlockUserRequest: objectSchema({
    blockedUserId: stringSchema(),
    scope: ref('BlockScope'),
    directChatId: nullable(stringSchema()),
    expiresAt: nullable(stringSchema({ format: 'date-time' })),
  }, ['blockedUserId', 'scope']),
  UserBlock: objectSchema({
    tenantId: stringSchema(),
    blockId: stringSchema(),
    blockerUserId: stringSchema(),
    blockedUserId: stringSchema(),
    scope: ref('BlockScope'),
    status: ref('UserBlockStatus'),
    directChatId: nullable(stringSchema()),
    expiresAt: nullable(stringSchema({ format: 'date-time' })),
    createdAt: stringSchema({ format: 'date-time' }),
    updatedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'blockId', 'blockerUserId', 'blockedUserId', 'scope', 'status', 'createdAt', 'updatedAt']),
  SocialDerivedSnapshotStatus: {
    type: 'string',
    enum: ['current', 'repair_required'],
  },
  SocialWritePersistence: objectSchema({
    journalAuthority: boolSchema(),
    snapshotStatus: ref('SocialDerivedSnapshotStatus'),
  }, ['journalAuthority', 'snapshotStatus']),
  EventActor: objectSchema({
    actorId: stringSchema(),
    actorKind: stringSchema(),
    actorSessionId: nullable(stringSchema()),
  }, ['actorId', 'actorKind']),
  CommitEnvelopeResponse: objectSchema({
    eventId: stringSchema(),
    tenantId: stringSchema(),
    eventType: stringSchema(),
    eventVersion: int32Schema({ minimum: 1 }),
    aggregateType: stringSchema(),
    aggregateId: stringSchema(),
    scopeType: stringSchema(),
    scopeId: stringSchema(),
    orderingKey: stringSchema(),
    orderingSeq: intSchema({ minimum: 0 }),
    causationId: nullable(stringSchema()),
    correlationId: nullable(stringSchema()),
    idempotencyKey: nullable(stringSchema()),
    actor: ref('EventActor'),
    occurredAt: stringSchema({ format: 'date-time' }),
    committedAt: stringSchema({ format: 'date-time' }),
    payloadSchema: nullable(stringSchema()),
    payload: stringSchema(),
    retentionClass: stringSchema(),
    auditClass: stringSchema(),
  }, [
    'eventId',
    'tenantId',
    'eventType',
    'eventVersion',
    'aggregateType',
    'aggregateId',
    'scopeType',
    'scopeId',
    'orderingKey',
    'orderingSeq',
    'actor',
    'occurredAt',
    'committedAt',
    'payload',
    'retentionClass',
    'auditClass',
  ]),
  OpenApiUserBlockResponse: objectSchema({
    userBlock: ref('UserBlock'),
    latestCommit: ref('CommitEnvelopeResponse'),
    persistence: ref('SocialWritePersistence'),
  }, ['userBlock', 'latestCommit', 'persistence']),
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
    friendRequestId: stringSchema(),
    requesterUserId: stringSchema(),
    targetUserId: stringSchema(),
    status: stringSchema(),
    requestMessage: nullable(stringSchema()),
    expiredAt: nullable(stringSchema({ format: 'date-time' })),
    createdAt: stringSchema({ format: 'date-time' }),
    updatedAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'friendRequestId', 'requesterUserId', 'targetUserId', 'status', 'createdAt', 'updatedAt']),
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
  SocialFriendRequestAcceptedConversation: objectSchema({
    tenantId: stringSchema(),
    conversationId: stringSchema(),
    kind: stringSchema(),
    createdAt: stringSchema({ format: 'date-time' }),
  }, ['tenantId', 'conversationId', 'kind', 'createdAt']),
  SocialFriendRequestMutationResponse: objectSchema({
    friendRequest: ref('FriendRequest'),
  }, ['friendRequest']),
  SocialFriendRequestPendingCountResponse: objectSchema({
    count: int32Schema({ minimum: 0 }),
  }, ['count']),
  SocialFriendRequestAcceptanceResponse: objectSchema({
    friendRequest: ref('FriendRequest'),
    friendship: ref('Friendship'),
    directChat: ref('DirectChat'),
    conversation: ref('SocialFriendRequestAcceptedConversation'),
  }, ['friendRequest', 'friendship', 'directChat', 'conversation']),
  SocialFriendshipMutationResponse: objectSchema({
    friendship: ref('Friendship'),
  }, ['friendship']),
  CreateConversationRequest: objectSchema({
    conversationId: nullable(stringSchema()),
    conversationType: stringSchema(),
    groupName: nullable(stringSchema({ maxLength: 256 })),
    clientRequestKey: nullable(stringSchema({ maxLength: 256 })),
    initializeKnowledgebase: {
      type: 'boolean',
      default: false,
      description: 'For group conversations only. When true, requests one Knowledgebase provisioning attempt after the group is durably created. Omitted or false never reserves, provisions, or validates a group Knowledgebase scope.',
    },
    memberUserIds: nullable({
      ...arrayOf(stringSchema({ minLength: 1, maxLength: 256 })),
      maxItems: 200,
    }),
    agentAssignments: nullable({
      ...arrayOf(ref('ConversationAgentAssignment')),
      minItems: 1,
      maxItems: 10,
    }),
    policyVersion: nullable(stringSchema()),
    capabilityFlags: nullable(arrayOf(stringSchema())),
    historyVisibility: nullable(stringSchema()),
    retentionPolicyRef: nullable(stringSchema()),
  }, ['conversationType']),
  ConversationAgentAssignment: objectSchema({
    agentId: stringSchema({
      minLength: 1,
      maxLength: 128,
      pattern: '^agent\\.[a-z0-9_-]+(?:\\.[a-z0-9_-]+)*$',
    }),
    revisionId: nullable(stringSchema({
      minLength: 1,
      maxLength: 128,
      pattern: '^revision\\.[a-z0-9_-]+(?:\\.[a-z0-9_-]+)*$',
    })),
  }, ['agentId']),
  ConversationAgentAssignments: objectSchema({
    generation: intSchema({ minimum: 1 }),
    source: stringSchema({ enum: ['default_policy', 'conversation_override'] }),
    agents: {
      ...arrayOf(ref('ConversationAgentAssignment')),
      minItems: 1,
      maxItems: 10,
    },
  }, ['generation', 'source', 'agents']),
  UpdateConversationAgentsRequest: objectSchema({
    expectedGeneration: intSchema({ minimum: 1 }),
    agentAssignments: {
      ...arrayOf(ref('ConversationAgentAssignment')),
      minItems: 1,
      maxItems: 10,
    },
  }, ['expectedGeneration', 'agentAssignments']),
  CreateAgentDialogRequest: objectSchema({
    agentId: stringSchema(),
    conversationId: nullable(stringSchema()),
  }, ['agentId']),
  CreateAgentHandoffRequest: objectSchema({
    conversationId: stringSchema(),
    targetId: stringSchema(),
    targetKind: stringSchema(),
    handoffSessionId: stringSchema(),
    handoffReason: nullable(stringSchema()),
  }, ['conversationId', 'targetId', 'targetKind', 'handoffSessionId']),
  CreateSystemChannelRequest: objectSchema({
    conversationId: stringSchema(),
    subscriberId: stringSchema(),
  }, ['conversationId', 'subscriberId']),
  CreateThreadConversationRequest: objectSchema({
    conversationId: stringSchema(),
    parentConversationId: stringSchema(),
    rootMessageId: stringSchema(),
  }, ['conversationId', 'parentConversationId', 'rootMessageId']),
  BindDirectChatRequest: objectSchema({
    conversationId: nullable(stringSchema()),
    directChatId: nullable(stringSchema()),
    leftActorId: stringSchema(),
    leftActorKind: stringSchema(),
    rightActorId: stringSchema(),
    rightActorKind: stringSchema(),
  }, ['leftActorId', 'leftActorKind', 'rightActorId', 'rightActorKind']),
  CreateConversationResult: objectSchema({
    conversationId: stringSchema(),
    eventId: stringSchema(),
    requestKey: stringSchema(),
    deliveryStatus: stringSchema({ enum: ['applied', 'replayed'] }),
    proofVersion: stringSchema(),
    knowledgebaseInitialization: stringSchema({
      enum: ['active', 'provisioning', 'failed'],
      description: 'Present only when initializeKnowledgebase was true. A failed value means group creation succeeded but the optional remote Knowledgebase provisioning attempt did not complete; the group owner can retry from the Knowledgebase action.',
    }),
  }, ['conversationId', 'eventId']),
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
    post: operation({ tag: 'presence', operationId: 'presence.heartbeat', summary: 'Publish current client route presence heartbeat', request: 'PresenceHeartbeatRequest', response: 'PresenceView' }),
  }),
  pathItem('/presence/me', {
    get: operation({ tag: 'presence', operationId: 'presence.me.retrieve', summary: 'Retrieve current principal presence', response: 'PresenceView' }),
  }),
  pathItem('/realtime/subscriptions/sync', {
    post: operation({ tag: 'realtime', operationId: 'realtime.subscriptions.sync', summary: 'Sync realtime subscription targets', request: 'RealtimeSubscriptionSyncRequest', response: 'RealtimeSubscriptionSyncResponse' }),
  }),
  pathItem('/realtime/ws', {
    get: {
      ...operation({ tag: 'realtime', operationId: 'realtime.ws.retrieve', summary: 'Retrieve the IM realtime websocket handshake', response: 'RealtimeWebSocketHandshake' }),
      security: [],
    },
  }),
  pathItem('/realtime/events/ack', {
    post: operation({ tag: 'realtime', operationId: 'realtime.events.ack', summary: 'Acknowledge realtime events', request: 'RealtimeEventAckRequest', response: 'AckResponse' }),
  }),
  pathItem('/realtime/events', {
    get: operation({ tag: 'realtime', operationId: 'realtime.events.list', summary: 'List pending realtime events', parameters: [p('PageSizeQuery'), p('CursorQuery')], response: 'RealtimeEventsResponse' }),
  }),
  pathItem('/calls/sessions', {
    post: operation({ tag: 'calls', operationId: 'calls.sessions.create', summary: 'Create an IM call signaling session', request: 'CreateRtcSessionRequest', response: 'RtcSessionMutationResponse', successStatus: '201' }),
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
    get: operation({ tag: 'calls', operationId: 'calls.sessions.signals.list', summary: 'List IM call signaling events', parameters: [p('RtcSessionIdPath'), p('AfterSignalSeqQuery'), p('CursorQuery'), p('PageSizeQuery')], response: 'RtcSignalEventsResponse' }),
    post: operation({ tag: 'calls', operationId: 'calls.sessions.signals.create', summary: 'Post an IM call signaling event', parameters: [p('RtcSessionIdPath')], request: 'PostRtcSignalRequest', response: 'RtcSignalEvent', successStatus: '201' }),
  }),
  pathItem('/calls/sessions/{rtcSessionId}/credentials', {
    parameters: [p('RtcSessionIdPath')],
    post: operation({ tag: 'calls', operationId: 'calls.sessions.credentials.create', summary: 'Issue an RTC media participant credential for an IM call', parameters: [p('RtcSessionIdPath')], request: 'IssueRtcParticipantCredentialRequest', response: 'RtcParticipantCredential', successStatus: '201' }),
  }),
  pathItem('/calls/sessions/{rtcSessionId}/credentials/refresh', {
    parameters: [p('RtcSessionIdPath')],
    post: operation({ tag: 'calls', operationId: 'calls.sessions.credentials.refresh', summary: 'Refresh an expiring RTC media participant credential', parameters: [p('RtcSessionIdPath')], request: 'IssueRtcParticipantCredentialRequest', response: 'RtcParticipantCredential' }),
  }),
  pathItem('/social/users', {
    get: operation({ tag: 'social', operationId: 'social.users.list', summary: 'Search social users', parameters: [p('QQuery'), p('PageSizeQuery'), p('CursorQuery')], response: 'SocialUserSearchResponse', statuses: ['400', '401', '403', '503'] }),
  }),
  pathItem('/social/friend_requests', {
    get: operation({ tag: 'social', operationId: 'social.friendRequests.list', summary: 'List friend requests', parameters: [p('DirectionQuery'), p('StatusQuery'), p('PageSizeQuery'), p('CursorQuery')], response: 'SdkWorkListResponse' }),
    post: operation({ tag: 'social', operationId: 'social.friendRequests.create', summary: 'Create a friend request', request: 'SubmitFriendRequestRequest', response: 'SocialFriendRequestMutationResponse', successStatus: '201' }),
  }),
  pathItem('/social/friend_requests/pending/count', {
    get: operation({ tag: 'social', operationId: 'social.friendRequests.pending.count.retrieve', summary: 'Retrieve pending incoming friend request count', response: 'SocialFriendRequestPendingCountResponse' }),
  }),
  pathItem('/social/friend_requests/{friendRequestId}/accept', {
    parameters: [p('FriendRequestIdPath')],
    post: operation({ tag: 'social', operationId: 'social.friendRequests.accept', summary: 'Accept a friend request', parameters: [p('FriendRequestIdPath')], response: 'SocialFriendRequestAcceptanceResponse' }),
  }),
  pathItem('/social/friend_requests/{friendRequestId}/decline', {
    parameters: [p('FriendRequestIdPath')],
    post: operation({ tag: 'social', operationId: 'social.friendRequests.decline', summary: 'Decline a friend request', parameters: [p('FriendRequestIdPath')], response: 'SocialFriendRequestMutationResponse' }),
  }),
  pathItem('/social/friend_requests/{friendRequestId}/cancel', {
    parameters: [p('FriendRequestIdPath')],
    post: operation({ tag: 'social', operationId: 'social.friendRequests.cancel', summary: 'Cancel a friend request', parameters: [p('FriendRequestIdPath')], response: 'SocialFriendRequestMutationResponse' }),
  }),
  pathItem('/social/friendships/{friendshipId}/remove', {
    parameters: [p('FriendshipIdPath')],
    post: operation({ tag: 'social', operationId: 'social.friendships.remove', summary: 'Remove a friendship', parameters: [p('FriendshipIdPath')], response: 'SocialFriendshipMutationResponse' }),
  }),
  pathItem('/social/user_blocks', {
    post: operation({ tag: 'social', operationId: 'social.userBlocks.create', summary: 'Block a social user', request: 'BlockUserRequest', response: 'OpenApiUserBlockResponse', successStatus: '201' }),
  }),
  pathItem('/social/user_blocks/{blockId}', {
    parameters: [p('BlockIdPath')],
    delete: operation({ tag: 'social', operationId: 'social.userBlocks.delete', summary: 'Release a social user block', parameters: [p('BlockIdPath')], successStatus: '204' }),
  }),
  pathItem('/social/contacts/tags', {
    get: operation({ tag: 'social', operationId: 'social.contacts.tags.list', summary: 'List contact tags', parameters: [p('PageSizeQuery'), p('CursorQuery')], response: 'SdkWorkListResponse' }),
    post: operation({ tag: 'social', operationId: 'social.contacts.tags.create', summary: 'Create a contact tag', request: 'CreateContactTagRequest', response: 'ContactTagView', successStatus: '201' }),
  }),
  pathItem('/social/contacts/tags/{tagId}', {
    parameters: [p('TagIdPath')],
    patch: operation({ tag: 'social', operationId: 'social.contacts.tags.update', summary: 'Update a contact tag', parameters: [p('TagIdPath')], request: 'UpdateContactTagRequest', response: 'ContactTagView' }),
    delete: operation({ tag: 'social', operationId: 'social.contacts.tags.delete', summary: 'Delete a contact tag', parameters: [p('TagIdPath')], response: 'DeleteContactTagResponse', successStatus: '204' }),
  }),
  pathItem('/social/contacts/{targetUserId}/recommendations', {
    parameters: [p('TargetUserIdPath')],
    post: operation({ tag: 'social', operationId: 'social.contacts.recommendations.create', summary: 'Create a contact recommendation', parameters: [p('TargetUserIdPath')], request: 'CreateContactRecommendationRequest', response: 'ContactRecommendationView', successStatus: '201' }),
  }),
  pathItem('/social/contacts/{targetUserId}/preferences', {
    parameters: [p('TargetUserIdPath')],
    get: operation({ tag: 'social', operationId: 'social.contacts.preferences.retrieve', summary: 'Retrieve contact preferences', parameters: [p('TargetUserIdPath')], response: 'ContactPreferencesView' }),
    patch: operation({ tag: 'social', operationId: 'social.contacts.preferences.update', summary: 'Update contact preferences', parameters: [p('TargetUserIdPath')], request: 'UpdateContactPreferencesRequest', response: 'ContactPreferencesView' }),
  }),
  pathItem('/social/contacts', {
    get: operation({ tag: 'social', operationId: 'social.contacts.list', summary: 'List social contacts', parameters: [p('PageSizeQuery'), p('CursorQuery')], response: 'ContactsResponse' }),
  }),
  pathItem('/chat/inbox', {
    get: operation({ tag: 'chat', operationId: 'inbox.list', summary: 'List current inbox window', parameters: [p('PageSizeQuery'), p('CursorQuery'), p('ConversationTypeQuery'), p('QQuery')], response: 'ConversationInboxPage' }),
  }),
  pathItem('/chat/conversations', {
    post: operation({ tag: 'chat', operationId: 'conversations.create', summary: 'Create a conversation', request: 'CreateConversationRequest', response: 'CreateConversationResult', successStatus: '201', statuses: ['400', '401', '403', '404', '409'] }),
  }),
  pathItem('/chat/conversations/agent_dialogs', {
    post: operation({ tag: 'chat', operationId: 'conversations.agentDialogs.create', summary: 'Create an agent dialog', request: 'CreateAgentDialogRequest', response: 'CreateConversationResult', successStatus: '201' }),
  }),
  pathItem('/chat/conversations/agent_handoffs', {
    post: operation({ tag: 'chat', operationId: 'conversations.agentHandoffs.create', summary: 'Create an agent handoff', request: 'CreateAgentHandoffRequest', response: 'CreateConversationResult', successStatus: '201' }),
  }),
  pathItem('/chat/conversations/system_channels', {
    post: operation({ tag: 'chat', operationId: 'conversations.systemChannels.create', summary: 'Create a system channel', request: 'CreateSystemChannelRequest', response: 'CreateConversationResult', successStatus: '201' }),
  }),
  pathItem('/chat/conversations/threads', {
    post: operation({ tag: 'chat', operationId: 'conversations.threads.create', summary: 'Create a thread conversation', request: 'CreateThreadConversationRequest', response: 'CreateConversationResult', successStatus: '201' }),
  }),
  pathItem('/chat/conversations/direct_chats/bindings', {
    post: operation({ tag: 'chat', operationId: 'conversations.directChats.bindings.create', summary: 'Create a direct chat conversation binding', request: 'BindDirectChatRequest', response: 'CreateConversationResult', successStatus: '201' }),
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
    get: operation({ tag: 'chat', operationId: 'conversations.members.list', summary: 'List conversation members', parameters: [p('ConversationIdPath'), p('PageSizeQuery'), p('CursorQuery')], response: 'ListMembersResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}/members/current', {
    parameters: [p('ConversationIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.members.current.retrieve', summary: 'Retrieve the current conversation member', parameters: [p('ConversationIdPath')], response: 'ConversationMember' }),
  }),
  pathItem('/chat/conversations/{conversationId}/agents', {
    parameters: [p('ConversationIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.agents.retrieve', summary: 'Retrieve assigned group agents', parameters: [p('ConversationIdPath')], response: 'ConversationAgentAssignments' }),
    put: operation({ tag: 'chat', operationId: 'conversations.agents.update', summary: 'Update assigned group agents', parameters: [p('ConversationIdPath')], request: 'UpdateConversationAgentsRequest', response: 'ConversationAgentAssignments', statuses: ['400', '401', '403', '404', '409'] }),
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
  pathItem('/chat/conversations/{conversationId}/members/accept_invitation', {
    parameters: [p('ConversationIdPath')],
    post: operation({ tag: 'chat', operationId: 'conversations.members.acceptInvitation', summary: 'Accept a conversation invitation', parameters: [p('ConversationIdPath')], response: 'ConversationMember' }),
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
    patch: operation({ tag: 'chat', operationId: 'conversations.readCursor.update', summary: 'Update read cursor', parameters: [p('ConversationIdPath')], request: 'UpdateReadCursorRequest', response: 'ReadCursorView' }),
  }),
  pathItem('/chat/conversations/{conversationId}/member_directory', {
    parameters: [p('ConversationIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.memberDirectory.list', summary: 'List member directory', parameters: [p('ConversationIdPath'), p('CursorQuery'), p('PageSizeQuery')], response: 'MemberDirectoryResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}/messages', {
    parameters: [p('ConversationIdPath')],
    get: operation({
      tag: 'chat',
      operationId: 'conversations.messages.list',
      summary: 'List conversation message history',
      description: 'Returns the latest message page when cursor is omitted. Subsequent requests pass the opaque server-issued cursor to continue toward older messages. Items in each page are returned in chronological messageSeq order. The cursor is bound to the authenticated tenant, organization, and conversation and must not be parsed or constructed by clients.',
      parameters: [p('ConversationIdPath'), p('CursorQuery'), p('PageSizeQuery')],
      response: 'ConversationMessageListResponse',
    }),
    post: operation({ tag: 'chat', operationId: 'conversations.messages.create', summary: 'Post a conversation message', parameters: [p('ConversationIdPath')], request: 'PostMessageRequest', response: 'PostMessageResult', successStatus: '201' }),
  }),
  pathItem('/chat/conversations/{conversationId}/system_channel/publish', {
    parameters: [p('ConversationIdPath')],
    post: operation({ tag: 'chat', operationId: 'conversations.systemChannel.publish', summary: 'Publish a system channel message', parameters: [p('ConversationIdPath')], request: 'PostMessageRequest', response: 'PostMessageResult' }),
  }),
  pathItem('/chat/conversations/{conversationId}/pins', {
    parameters: [p('ConversationIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.pins.list', summary: 'List pinned messages', parameters: [p('ConversationIdPath'), p('CursorQuery'), p('PageSizeQuery')], response: 'PinnedMessagesResponse' }),
  }),
  pathItem('/chat/conversations/{conversationId}/messages/{messageId}/interaction_summary', {
    parameters: [p('ConversationIdPath'), p('MessageIdPath')],
    get: operation({ tag: 'chat', operationId: 'conversations.messages.interactionSummary.retrieve', summary: 'Retrieve message interaction summary', parameters: [p('ConversationIdPath'), p('MessageIdPath')], response: 'MessageInteractionSummaryView' }),
  }),
  pathItem('/chat/messages/search', {
    get: operation({
      tag: 'chat',
      operationId: 'messages.search',
      summary: 'Search conversation message history',
      description: 'Full-text search over message history scoped to the authenticated principal. When conversationId is omitted the search covers every conversation the principal is a member of. Results are returned newest-first with an opaque keyset cursor for older pages.',
      parameters: [p('SearchQQuery'), p('ConversationIdQuery'), p('PageSizeQuery'), p('CursorQuery')],
      response: 'MessageSearchResponse',
      statuses: ['400', '401', '403', '503'],
    }),
  }),
  pathItem('/chat/messages/{messageId}/edit', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.edit', summary: 'Edit a message', parameters: [p('MessageIdPath')], request: 'EditMessageRequest', response: 'MessageMutationResult' }),
  }),
  pathItem('/chat/messages/{messageId}/recall', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.recall', summary: 'Recall a message', parameters: [p('MessageIdPath')], request: 'RecallMessageRequest', response: 'MessageMutationResult' }),
  }),
  pathItem('/chat/messages/favorites', {
    get: operation({ tag: 'chat', operationId: 'messages.favorites.list', summary: 'List message favorites', parameters: [p('PageSizeQuery'), p('CursorQuery'), p('FavoriteTypeQuery'), p('QQuery')], response: 'FavoriteMessagesResponse' }),
  }),
  pathItem('/chat/messages/{messageId}/favorites', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.favorites.create', summary: 'Favorite a message', parameters: [p('MessageIdPath')], request: 'FavoriteMessageRequest', response: 'MessageFavoriteView', successStatus: '201' }),
  }),
  pathItem('/chat/messages/favorites/{favoriteId}', {
    parameters: [p('FavoriteIdPath')],
    delete: operation({ tag: 'chat', operationId: 'messages.favorites.delete', summary: 'Delete a message favorite', parameters: [p('FavoriteIdPath')], response: 'DeleteMessageFavoriteResponse', successStatus: '204' }),
  }),
  pathItem('/chat/messages/{messageId}/visibility', {
    parameters: [p('MessageIdPath')],
    delete: operation({ tag: 'chat', operationId: 'messages.visibility.delete', summary: 'Delete message visibility for the current principal', parameters: [p('MessageIdPath')], successStatus: '204' }),
  }),
  pathItem('/chat/messages/{messageId}/reactions', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.reactions.create', summary: 'Add a message reaction', parameters: [p('MessageIdPath')], request: 'MessageReactionRequest', response: 'MessageReactionMutationResult', successStatus: '201' }),
  }),
  pathItem('/chat/messages/{messageId}/reactions/remove', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.reactions.remove', summary: 'Remove a message reaction', parameters: [p('MessageIdPath')], request: 'MessageReactionRequest', response: 'MessageReactionMutationResult' }),
  }),
  pathItem('/chat/messages/{messageId}/pin', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.pin', summary: 'Pin a message', parameters: [p('MessageIdPath')], response: 'MessagePinMutationResult' }),
  }),
  pathItem('/chat/messages/{messageId}/unpin', {
    parameters: [p('MessageIdPath')],
    post: operation({ tag: 'chat', operationId: 'messages.unpin', summary: 'Unpin a message', parameters: [p('MessageIdPath')], response: 'MessagePinMutationResult' }),
  }),
  pathItem('/chat/rooms', {
    post: operation({ tag: 'chat', operationId: 'rooms.create', summary: 'Create a live, chat, or game room bound to a group conversation', request: 'CreateRoomRequest', response: 'CreateConversationResult', successStatus: '201', statuses: ['400', '401', '403', '404', '409'] }),
  }),
  pathItem('/chat/rooms/{roomId}', {
    parameters: [p('RoomIdPath')],
    get: operation({ tag: 'chat', operationId: 'rooms.retrieve', summary: 'Retrieve room metadata and active member count', parameters: [p('RoomIdPath')], response: 'RoomView' }),
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
    post: operation({ tag: 'streams', operationId: 'streams.create', summary: 'Open a stream', request: 'OpenStreamRequest', response: 'StreamView', successStatus: '201' }),
  }),
  pathItem('/streams/{streamId}/frames', {
    parameters: [p('StreamIdPath')],
    get: operation({ tag: 'streams', operationId: 'streams.frames.list', summary: 'List stream frames', parameters: [p('StreamIdPath'), p('PageSizeQuery'), p('CursorQuery')], response: 'StreamFramesResponse' }),
    post: operation({ tag: 'streams', operationId: 'streams.frames.create', summary: 'Append a stream frame', parameters: [p('StreamIdPath')], request: 'AppendStreamFrameRequest', response: 'StreamFrameView', successStatus: '201' }),
  }),
  pathItem('/streams/{streamId}/checkpoint', {
    parameters: [p('StreamIdPath')],
    post: operation({ tag: 'streams', operationId: 'streams.checkpoint', summary: 'Checkpoint a stream', parameters: [p('StreamIdPath')], response: 'StreamView' }),
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

applySdkworkV3OpenApiStandard(document, { authProfile: 'api-key-or-dual-token' });
const yaml = await loadGeneratorYaml(workspaceRoot);
const serialized = yaml.dump(document, { noRefs: true, sortKeys: false, lineWidth: 120 });
mkdirSync(path.dirname(outputPath), { recursive: true });
writeFileSync(outputPath, serialized, 'utf8');
console.log(`[sdkwork-im-sdk] materialized ${path.relative(workspaceRoot, outputPath).replaceAll('\\', '/')}`);
