import Foundation

public struct AckResponse: Codable {
    public let ok: Bool?


    public init(ok: Bool? = nil) {
        self.ok = ok
    }
}

public struct PresenceHeartbeatRequest: Codable {
    public let deviceId: String?


    public init(deviceId: String? = nil) {
        self.deviceId = deviceId
    }
}

public struct PresenceView: Codable {
    public let tenantId: String?
    public let principalId: String?
    public let principalKind: String?
    public let deviceId: String?
    public let status: String?
    public let updatedAt: String?


    public init(tenantId: String? = nil, principalId: String? = nil, principalKind: String? = nil, deviceId: String? = nil, status: String? = nil, updatedAt: String? = nil) {
        self.tenantId = tenantId
        self.principalId = principalId
        self.principalKind = principalKind
        self.deviceId = deviceId
        self.status = status
        self.updatedAt = updatedAt
    }
}

public struct RealtimeSubscriptionSyncRequest: Codable {
    public let deviceId: String?
    public let conversations: [String]?
    public let items: [RealtimeSubscriptionItemInput]?


    public init(deviceId: String? = nil, conversations: [String]? = nil, items: [RealtimeSubscriptionItemInput]? = nil) {
        self.deviceId = deviceId
        self.conversations = conversations
        self.items = items
    }
}

public struct RealtimeSubscriptionItemInput: Codable {
    public let scopeType: String?
    public let scopeId: String?
    public let eventTypes: [String]?


    public init(scopeType: String? = nil, scopeId: String? = nil, eventTypes: [String]? = nil) {
        self.scopeType = scopeType
        self.scopeId = scopeId
        self.eventTypes = eventTypes
    }
}

public struct RealtimeSubscriptionSyncResponse: Codable {
    public let subscriptions: [String]?


    public init(subscriptions: [String]? = nil) {
        self.subscriptions = subscriptions
    }
}

public struct RealtimeEventAckRequest: Codable {
    public let eventIds: [String]?


    public init(eventIds: [String]? = nil) {
        self.eventIds = eventIds
    }
}

public struct RealtimeEventView: Codable {
    public let eventId: String?
    public let scope: String?
    public let scopeId: String?
    public let eventType: String?
    public let payload: String?
    public let occurredAt: String?


    public init(eventId: String? = nil, scope: String? = nil, scopeId: String? = nil, eventType: String? = nil, payload: String? = nil, occurredAt: String? = nil) {
        self.eventId = eventId
        self.scope = scope
        self.scopeId = scopeId
        self.eventType = eventType
        self.payload = payload
        self.occurredAt = occurredAt
    }
}

public struct RealtimeEventsResponse: Codable {
    public let items: [RealtimeEventView]?
    public let nextCursor: String?
    public let hasMore: Bool?


    public init(items: [RealtimeEventView]? = nil, nextCursor: String? = nil, hasMore: Bool? = nil) {
        self.items = items
        self.nextCursor = nextCursor
        self.hasMore = hasMore
    }
}

public struct RtcSession: Codable {
    public let tenantId: String?
    public let rtcSessionId: String?
    public let conversationId: String?
    public let initiatorId: String?
    public let initiatorKind: String?
    public let providerPluginId: String?
    public let providerSessionId: String?
    public let accessEndpoint: String?
    public let providerRegion: String?
    public let rtcMode: String?
    public let state: String?
    public let signalingStreamId: String?
    public let artifactMessageId: String?
    public let startedAt: String?
    public let endedAt: String?


    public init(tenantId: String? = nil, rtcSessionId: String? = nil, conversationId: String? = nil, initiatorId: String? = nil, initiatorKind: String? = nil, providerPluginId: String? = nil, providerSessionId: String? = nil, accessEndpoint: String? = nil, providerRegion: String? = nil, rtcMode: String? = nil, state: String? = nil, signalingStreamId: String? = nil, artifactMessageId: String? = nil, startedAt: String? = nil, endedAt: String? = nil) {
        self.tenantId = tenantId
        self.rtcSessionId = rtcSessionId
        self.conversationId = conversationId
        self.initiatorId = initiatorId
        self.initiatorKind = initiatorKind
        self.providerPluginId = providerPluginId
        self.providerSessionId = providerSessionId
        self.accessEndpoint = accessEndpoint
        self.providerRegion = providerRegion
        self.rtcMode = rtcMode
        self.state = state
        self.signalingStreamId = signalingStreamId
        self.artifactMessageId = artifactMessageId
        self.startedAt = startedAt
        self.endedAt = endedAt
    }
}

public struct CreateRtcSessionRequest: Codable {
    public let rtcSessionId: String?
    public let conversationId: String?
    public let rtcMode: String?


    public init(rtcSessionId: String? = nil, conversationId: String? = nil, rtcMode: String? = nil) {
        self.rtcSessionId = rtcSessionId
        self.conversationId = conversationId
        self.rtcMode = rtcMode
    }
}

public struct InviteRtcSessionRequest: Codable {
    public let signalingStreamId: String?


    public init(signalingStreamId: String? = nil) {
        self.signalingStreamId = signalingStreamId
    }
}

public struct UpdateRtcSessionRequest: Codable {
    public let artifactMessageId: String?


    public init(artifactMessageId: String? = nil) {
        self.artifactMessageId = artifactMessageId
    }
}

public struct PostRtcSignalRequest: Codable {
    public let signalType: String?
    public let schemaRef: String?
    public let payload: String?
    public let signalingStreamId: String?


    public init(signalType: String? = nil, schemaRef: String? = nil, payload: String? = nil, signalingStreamId: String? = nil) {
        self.signalType = signalType
        self.schemaRef = schemaRef
        self.payload = payload
        self.signalingStreamId = signalingStreamId
    }
}

public struct IssueRtcParticipantCredentialRequest: Codable {
    public let participantId: String?


    public init(participantId: String? = nil) {
        self.participantId = participantId
    }
}

public struct RtcSessionMutationResponse: Codable {
    public let tenantId: String?
    public let rtcSessionId: String?
    public let conversationId: String?
    public let initiatorId: String?
    public let initiatorKind: String?
    public let providerPluginId: String?
    public let providerSessionId: String?
    public let accessEndpoint: String?
    public let providerRegion: String?
    public let rtcMode: String?
    public let state: String?
    public let signalingStreamId: String?
    public let artifactMessageId: String?
    public let startedAt: String?
    public let endedAt: String?
    public let requestKey: String?
    public let deliveryStatus: String?
    public let proofVersion: String?


    public init(tenantId: String? = nil, rtcSessionId: String? = nil, conversationId: String? = nil, initiatorId: String? = nil, initiatorKind: String? = nil, providerPluginId: String? = nil, providerSessionId: String? = nil, accessEndpoint: String? = nil, providerRegion: String? = nil, rtcMode: String? = nil, state: String? = nil, signalingStreamId: String? = nil, artifactMessageId: String? = nil, startedAt: String? = nil, endedAt: String? = nil, requestKey: String? = nil, deliveryStatus: String? = nil, proofVersion: String? = nil) {
        self.tenantId = tenantId
        self.rtcSessionId = rtcSessionId
        self.conversationId = conversationId
        self.initiatorId = initiatorId
        self.initiatorKind = initiatorKind
        self.providerPluginId = providerPluginId
        self.providerSessionId = providerSessionId
        self.accessEndpoint = accessEndpoint
        self.providerRegion = providerRegion
        self.rtcMode = rtcMode
        self.state = state
        self.signalingStreamId = signalingStreamId
        self.artifactMessageId = artifactMessageId
        self.startedAt = startedAt
        self.endedAt = endedAt
        self.requestKey = requestKey
        self.deliveryStatus = deliveryStatus
        self.proofVersion = proofVersion
    }
}

public struct RtcSignalSender: Codable {
    public let id: String?
    public let kind: String?
    public let memberId: String?
    public let deviceId: String?
    public let sessionId: String?
    public let metadata: [String: Any]?


    public init(id: String? = nil, kind: String? = nil, memberId: String? = nil, deviceId: String? = nil, sessionId: String? = nil, metadata: [String: Any]? = nil) {
        self.id = id
        self.kind = kind
        self.memberId = memberId
        self.deviceId = deviceId
        self.sessionId = sessionId
        self.metadata = metadata
    }
}

public struct RtcSignalEvent: Codable {
    public let tenantId: String?
    public let rtcSessionId: String?
    public let signalSeq: Int?
    public let conversationId: String?
    public let rtcMode: String?
    public let signalType: String?
    public let schemaRef: String?
    public let payload: String?
    public let sender: RtcSignalSender?
    public let signalingStreamId: String?
    public let occurredAt: String?


    public init(tenantId: String? = nil, rtcSessionId: String? = nil, signalSeq: Int? = nil, conversationId: String? = nil, rtcMode: String? = nil, signalType: String? = nil, schemaRef: String? = nil, payload: String? = nil, sender: RtcSignalSender? = nil, signalingStreamId: String? = nil, occurredAt: String? = nil) {
        self.tenantId = tenantId
        self.rtcSessionId = rtcSessionId
        self.signalSeq = signalSeq
        self.conversationId = conversationId
        self.rtcMode = rtcMode
        self.signalType = signalType
        self.schemaRef = schemaRef
        self.payload = payload
        self.sender = sender
        self.signalingStreamId = signalingStreamId
        self.occurredAt = occurredAt
    }
}

public struct RtcParticipantCredential: Codable {
    public let tenantId: String?
    public let rtcSessionId: String?
    public let participantId: String?
    public let credential: String?
    public let expiresAt: String?


    public init(tenantId: String? = nil, rtcSessionId: String? = nil, participantId: String? = nil, credential: String? = nil, expiresAt: String? = nil) {
        self.tenantId = tenantId
        self.rtcSessionId = rtcSessionId
        self.participantId = participantId
        self.credential = credential
        self.expiresAt = expiresAt
    }
}

public struct Sender: Codable {
    public let id: String?
    public let kind: String?
    public let principalId: String?
    public let principalKind: String?
    public let displayName: String?
    public let avatarUrl: String?


    public init(id: String? = nil, kind: String? = nil, principalId: String? = nil, principalKind: String? = nil, displayName: String? = nil, avatarUrl: String? = nil) {
        self.id = id
        self.kind = kind
        self.principalId = principalId
        self.principalKind = principalKind
        self.displayName = displayName
        self.avatarUrl = avatarUrl
    }
}

public struct MessageReplyReference: Codable {
    public let messageId: String?
    public let senderDisplayName: String?
    public let contentPreview: String?


    public init(messageId: String? = nil, senderDisplayName: String? = nil, contentPreview: String? = nil) {
        self.messageId = messageId
        self.senderDisplayName = senderDisplayName
        self.contentPreview = contentPreview
    }
}

public struct DriveReference: Codable {
    public let driveUri: String?
    public let spaceId: String?
    public let nodeId: String?
    public let nodeVersion: String?


    public init(driveUri: String? = nil, spaceId: String? = nil, nodeId: String? = nil, nodeVersion: String? = nil) {
        self.driveUri = driveUri
        self.spaceId = spaceId
        self.nodeId = nodeId
        self.nodeVersion = nodeVersion
    }
}

public struct MediaResource: Codable {
    public let id: String?
    public let kind: String?
    public let mediaKind: String?
    public let source: String?
    public let uri: String?
    public let publicUrl: String?
    public let url: String?
    public let name: String?
    public let title: String?
    public let fileName: String?
    public let mimeType: String?
    public let size: Int?
    public let sizeBytes: String?
    public let fileSize: String?
    public let durationSeconds: Int?
    public let poster: MediaResource?
    public let thumbnails: [MediaResource]?


    public init(id: String? = nil, kind: String? = nil, mediaKind: String? = nil, source: String? = nil, uri: String? = nil, publicUrl: String? = nil, url: String? = nil, name: String? = nil, title: String? = nil, fileName: String? = nil, mimeType: String? = nil, size: Int? = nil, sizeBytes: String? = nil, fileSize: String? = nil, durationSeconds: Int? = nil, poster: MediaResource? = nil, thumbnails: [MediaResource]? = nil) {
        self.id = id
        self.kind = kind
        self.mediaKind = mediaKind
        self.source = source
        self.uri = uri
        self.publicUrl = publicUrl
        self.url = url
        self.name = name
        self.title = title
        self.fileName = fileName
        self.mimeType = mimeType
        self.size = size
        self.sizeBytes = sizeBytes
        self.fileSize = fileSize
        self.durationSeconds = durationSeconds
        self.poster = poster
        self.thumbnails = thumbnails
    }
}

public enum ContentPart: Codable {
    case text(TextContentPart)
    case data(DataContentPart)
    case media(MediaContentPart)
    case signal(SignalContentPart)
    case streamRef(StreamRefContentPart)

    private enum CodingKeys: String, CodingKey {
        case kind = "kind"
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        let kind = try container.decode(String.self, forKey: .kind)
        switch kind {
        case "text": self = .text(try TextContentPart(from: decoder))
        case "data": self = .data(try DataContentPart(from: decoder))
        case "media": self = .media(try MediaContentPart(from: decoder))
        case "signal": self = .signal(try SignalContentPart(from: decoder))
        case "stream_ref": self = .streamRef(try StreamRefContentPart(from: decoder))
        default:
            throw DecodingError.dataCorruptedError(forKey: .kind, in: container, debugDescription: "Unknown kind discriminator: \(kind)")
        }
    }

    public func encode(to encoder: Encoder) throws {
        switch self {
        case .text(let value): try value.encode(to: encoder)
        case .data(let value): try value.encode(to: encoder)
        case .media(let value): try value.encode(to: encoder)
        case .signal(let value): try value.encode(to: encoder)
        case .streamRef(let value): try value.encode(to: encoder)
        }
    }
}

public struct MessageBody: Codable {
    public let text: String?
    public let parts: [ContentPart]?
    public let replyTo: MessageReplyReference?
    public let renderHints: [String: Any]?
    public let summary: String?
    public let metadata: [String: Any]?


    public init(text: String? = nil, parts: [ContentPart]? = nil, replyTo: MessageReplyReference? = nil, renderHints: [String: Any]? = nil, summary: String? = nil, metadata: [String: Any]? = nil) {
        self.text = text
        self.parts = parts
        self.replyTo = replyTo
        self.renderHints = renderHints
        self.summary = summary
        self.metadata = metadata
    }
}

public struct TimelineViewEntry: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let messageId: String?
    public let messageSeq: Int?
    public let summary: String?
    public let sender: Sender?
    public let body: MessageBody?
    public let messageType: String?
    public let deliveryMode: String?
    public let clientMsgId: String?
    public let streamSessionId: String?
    public let rtcSessionId: String?
    public let occurredAt: String?
    public let committedAt: String?


    public init(tenantId: String? = nil, conversationId: String? = nil, messageId: String? = nil, messageSeq: Int? = nil, summary: String? = nil, sender: Sender? = nil, body: MessageBody? = nil, messageType: String? = nil, deliveryMode: String? = nil, clientMsgId: String? = nil, streamSessionId: String? = nil, rtcSessionId: String? = nil, occurredAt: String? = nil, committedAt: String? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.messageId = messageId
        self.messageSeq = messageSeq
        self.summary = summary
        self.sender = sender
        self.body = body
        self.messageType = messageType
        self.deliveryMode = deliveryMode
        self.clientMsgId = clientMsgId
        self.streamSessionId = streamSessionId
        self.rtcSessionId = rtcSessionId
        self.occurredAt = occurredAt
        self.committedAt = committedAt
    }
}

public struct TimelineResponse: Codable {
    public let items: [TimelineViewEntry]?
    public let nextAfterSeq: Int?
    public let hasMore: Bool?


    public init(items: [TimelineViewEntry]? = nil, nextAfterSeq: Int? = nil, hasMore: Bool? = nil) {
        self.items = items
        self.nextAfterSeq = nextAfterSeq
        self.hasMore = hasMore
    }
}

public struct PostMessageRequest: Codable {
    public let text: String?
    public let parts: [ContentPart]?
    public let replyTo: MessageReplyReference?
    public let clientMsgId: String?
    public let summary: String?
    public let renderHints: [String: Any]?


    public init(text: String? = nil, parts: [ContentPart]? = nil, replyTo: MessageReplyReference? = nil, clientMsgId: String? = nil, summary: String? = nil, renderHints: [String: Any]? = nil) {
        self.text = text
        self.parts = parts
        self.replyTo = replyTo
        self.clientMsgId = clientMsgId
        self.summary = summary
        self.renderHints = renderHints
    }
}

public struct EditMessageRequest: Codable {
    public let text: String?
    public let parts: [ContentPart]?
    public let replyTo: MessageReplyReference?


    public init(text: String? = nil, parts: [ContentPart]? = nil, replyTo: MessageReplyReference? = nil) {
        self.text = text
        self.parts = parts
        self.replyTo = replyTo
    }
}

public struct PostedMessageResponse: Codable {
    public let conversationId: String?
    public let messageId: String?
    public let messageSeq: Int?
    public let body: MessageBody?
    public let occurredAt: String?


    public init(conversationId: String? = nil, messageId: String? = nil, messageSeq: Int? = nil, body: MessageBody? = nil, occurredAt: String? = nil) {
        self.conversationId = conversationId
        self.messageId = messageId
        self.messageSeq = messageSeq
        self.body = body
        self.occurredAt = occurredAt
    }
}

public struct MessageReactionRequest: Codable {
    public let reactionKey: String?


    public init(reactionKey: String? = nil) {
        self.reactionKey = reactionKey
    }
}

public struct MessageReactionCountView: Codable {
    public let reactionKey: String?
    public let count: Int?


    public init(reactionKey: String? = nil, count: Int? = nil) {
        self.reactionKey = reactionKey
        self.count = count
    }
}

public struct InteractionActorView: Codable {
    public let id: String?
    public let kind: String?


    public init(id: String? = nil, kind: String? = nil) {
        self.id = id
        self.kind = kind
    }
}

public struct MessagePinView: Codable {
    public let pinnedBy: InteractionActorView?
    public let pinnedAt: String?


    public init(pinnedBy: InteractionActorView? = nil, pinnedAt: String? = nil) {
        self.pinnedBy = pinnedBy
        self.pinnedAt = pinnedAt
    }
}

public struct MessageInteractionSummaryView: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let messageId: String?
    public let messageSeq: Int?
    public let totalReactionCount: Int?
    public let reactionCounts: [MessageReactionCountView]?
    public let pin: MessagePinView?


    public init(tenantId: String? = nil, conversationId: String? = nil, messageId: String? = nil, messageSeq: Int? = nil, totalReactionCount: Int? = nil, reactionCounts: [MessageReactionCountView]? = nil, pin: MessagePinView? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.messageId = messageId
        self.messageSeq = messageSeq
        self.totalReactionCount = totalReactionCount
        self.reactionCounts = reactionCounts
        self.pin = pin
    }
}

public struct MessageReactionMutationResult: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let messageId: String?
    public let reactionKey: String?
    public let count: Int?
    public let updatedAt: String?


    public init(tenantId: String? = nil, conversationId: String? = nil, messageId: String? = nil, reactionKey: String? = nil, count: Int? = nil, updatedAt: String? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.messageId = messageId
        self.reactionKey = reactionKey
        self.count = count
        self.updatedAt = updatedAt
    }
}

public struct MessagePinMutationResult: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let messageId: String?
    public let isPinned: Bool?
    public let updatedAt: String?


    public init(tenantId: String? = nil, conversationId: String? = nil, messageId: String? = nil, isPinned: Bool? = nil, updatedAt: String? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.messageId = messageId
        self.isPinned = isPinned
        self.updatedAt = updatedAt
    }
}

public struct MessageVisibilityMutationResult: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let messageId: String?
    public let messageSeq: Int?
    public let principalKind: String?
    public let principalId: String?
    public let isDeleted: Bool?
    public let updatedAt: String?


    public init(tenantId: String? = nil, conversationId: String? = nil, messageId: String? = nil, messageSeq: Int? = nil, principalKind: String? = nil, principalId: String? = nil, isDeleted: Bool? = nil, updatedAt: String? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.messageId = messageId
        self.messageSeq = messageSeq
        self.principalKind = principalKind
        self.principalId = principalId
        self.isDeleted = isDeleted
        self.updatedAt = updatedAt
    }
}

public struct FavoriteMessageRequest: Codable {
    public let conversationId: String?
    public let favoriteType: String?
    public let title: String?
    public let contentPreview: String?
    public let sourceDisplayName: String?


    public init(conversationId: String? = nil, favoriteType: String? = nil, title: String? = nil, contentPreview: String? = nil, sourceDisplayName: String? = nil) {
        self.conversationId = conversationId
        self.favoriteType = favoriteType
        self.title = title
        self.contentPreview = contentPreview
        self.sourceDisplayName = sourceDisplayName
    }
}

public struct MessageFavoriteView: Codable {
    public let tenantId: String?
    public let principalKind: String?
    public let principalId: String?
    public let favoriteId: String?
    public let favoriteType: String?
    public let conversationId: String?
    public let messageId: String?
    public let messageSeq: Int?
    public let title: String?
    public let contentPreview: String?
    public let sourceDisplayName: String?
    public let favoritedAt: String?


    public init(tenantId: String? = nil, principalKind: String? = nil, principalId: String? = nil, favoriteId: String? = nil, favoriteType: String? = nil, conversationId: String? = nil, messageId: String? = nil, messageSeq: Int? = nil, title: String? = nil, contentPreview: String? = nil, sourceDisplayName: String? = nil, favoritedAt: String? = nil) {
        self.tenantId = tenantId
        self.principalKind = principalKind
        self.principalId = principalId
        self.favoriteId = favoriteId
        self.favoriteType = favoriteType
        self.conversationId = conversationId
        self.messageId = messageId
        self.messageSeq = messageSeq
        self.title = title
        self.contentPreview = contentPreview
        self.sourceDisplayName = sourceDisplayName
        self.favoritedAt = favoritedAt
    }
}

public struct FavoriteMessagesResponse: Codable {
    public let items: [MessageFavoriteView]?
    public let nextCursor: String?
    public let hasMore: Bool?


    public init(items: [MessageFavoriteView]? = nil, nextCursor: String? = nil, hasMore: Bool? = nil) {
        self.items = items
        self.nextCursor = nextCursor
        self.hasMore = hasMore
    }
}

public struct DeleteMessageFavoriteResponse: Codable {
    public let favoriteId: String?
    public let deleted: Bool?


    public init(favoriteId: String? = nil, deleted: Bool? = nil) {
        self.favoriteId = favoriteId
        self.deleted = deleted
    }
}

public struct ConversationPreferencesView: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let principalKind: String?
    public let principalId: String?
    public let isPinned: Bool?
    public let isMuted: Bool?
    public let isMarkedUnread: Bool?
    public let isHidden: Bool?
    public let updatedAt: String?


    public init(tenantId: String? = nil, conversationId: String? = nil, principalKind: String? = nil, principalId: String? = nil, isPinned: Bool? = nil, isMuted: Bool? = nil, isMarkedUnread: Bool? = nil, isHidden: Bool? = nil, updatedAt: String? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.principalKind = principalKind
        self.principalId = principalId
        self.isPinned = isPinned
        self.isMuted = isMuted
        self.isMarkedUnread = isMarkedUnread
        self.isHidden = isHidden
        self.updatedAt = updatedAt
    }
}

public struct UpdateConversationPreferencesRequest: Codable {
    public let isPinned: Bool?
    public let isMuted: Bool?
    public let isMarkedUnread: Bool?
    public let isHidden: Bool?


    public init(isPinned: Bool? = nil, isMuted: Bool? = nil, isMarkedUnread: Bool? = nil, isHidden: Bool? = nil) {
        self.isPinned = isPinned
        self.isMuted = isMuted
        self.isMarkedUnread = isMarkedUnread
        self.isHidden = isHidden
    }
}

public struct ConversationProfileView: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let displayName: String?
    public let avatarUrl: String?
    public let notice: String?
    public let updatedAt: String?
    public let updatedByPrincipalKind: String?
    public let updatedByPrincipalId: String?


    public init(tenantId: String? = nil, conversationId: String? = nil, displayName: String? = nil, avatarUrl: String? = nil, notice: String? = nil, updatedAt: String? = nil, updatedByPrincipalKind: String? = nil, updatedByPrincipalId: String? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.displayName = displayName
        self.avatarUrl = avatarUrl
        self.notice = notice
        self.updatedAt = updatedAt
        self.updatedByPrincipalKind = updatedByPrincipalKind
        self.updatedByPrincipalId = updatedByPrincipalId
    }
}

public struct UpdateConversationProfileRequest: Codable {
    public let displayName: String?
    public let avatarUrl: String?
    public let notice: String?


    public init(displayName: String? = nil, avatarUrl: String? = nil, notice: String? = nil) {
        self.displayName = displayName
        self.avatarUrl = avatarUrl
        self.notice = notice
    }
}

public struct ConversationSummaryView: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let messageCount: Int?
    public let lastMessageSeq: Int?
    public let lastSummary: String?
    public let lastMessageAt: String?


    public init(tenantId: String? = nil, conversationId: String? = nil, messageCount: Int? = nil, lastMessageSeq: Int? = nil, lastSummary: String? = nil, lastMessageAt: String? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.messageCount = messageCount
        self.lastMessageSeq = lastMessageSeq
        self.lastSummary = lastSummary
        self.lastMessageAt = lastMessageAt
    }
}

public struct ConversationInboxPeerView: Codable {
    public let principalKind: String?
    public let principalId: String?
    public let userId: String?
    public let chatId: String?
    public let displayName: String?
    public let avatarUrl: String?
    public let relationshipState: String?


    public init(principalKind: String? = nil, principalId: String? = nil, userId: String? = nil, chatId: String? = nil, displayName: String? = nil, avatarUrl: String? = nil, relationshipState: String? = nil) {
        self.principalKind = principalKind
        self.principalId = principalId
        self.userId = userId
        self.chatId = chatId
        self.displayName = displayName
        self.avatarUrl = avatarUrl
        self.relationshipState = relationshipState
    }
}

public struct ConversationInboxPreferencesView: Codable {
    public let isPinned: Bool?
    public let isMuted: Bool?
    public let isMarkedUnread: Bool?
    public let isHidden: Bool?


    public init(isPinned: Bool? = nil, isMuted: Bool? = nil, isMarkedUnread: Bool? = nil, isHidden: Bool? = nil) {
        self.isPinned = isPinned
        self.isMuted = isMuted
        self.isMarkedUnread = isMarkedUnread
        self.isHidden = isHidden
    }
}

public struct ConversationInboxEntry: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let agentHandoff: Bool?
    public let conversationType: String?
    public let displayName: String?
    public let avatarUrl: String?
    public let displaySource: String?
    public let peer: ConversationInboxPeerView?
    public let preferences: ConversationInboxPreferencesView?
    public let lastActivityAt: String?
    public let lastMessageId: String?
    public let lastSenderId: String?
    public let messageCount: Int?
    public let lastMessageSeq: Int?
    public let lastSummary: String?
    public let lastMessageAt: String?
    public let unreadCount: Int?


    public init(tenantId: String? = nil, conversationId: String? = nil, agentHandoff: Bool? = nil, conversationType: String? = nil, displayName: String? = nil, avatarUrl: String? = nil, displaySource: String? = nil, peer: ConversationInboxPeerView? = nil, preferences: ConversationInboxPreferencesView? = nil, lastActivityAt: String? = nil, lastMessageId: String? = nil, lastSenderId: String? = nil, messageCount: Int? = nil, lastMessageSeq: Int? = nil, lastSummary: String? = nil, lastMessageAt: String? = nil, unreadCount: Int? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.agentHandoff = agentHandoff
        self.conversationType = conversationType
        self.displayName = displayName
        self.avatarUrl = avatarUrl
        self.displaySource = displaySource
        self.peer = peer
        self.preferences = preferences
        self.lastActivityAt = lastActivityAt
        self.lastMessageId = lastMessageId
        self.lastSenderId = lastSenderId
        self.messageCount = messageCount
        self.lastMessageSeq = lastMessageSeq
        self.lastSummary = lastSummary
        self.lastMessageAt = lastMessageAt
        self.unreadCount = unreadCount
    }
}

public struct InboxResponse: Codable {
    public let items: [ConversationInboxEntry]?
    public let nextCursor: String?
    public let hasMore: Bool?


    public init(items: [ConversationInboxEntry]? = nil, nextCursor: String? = nil, hasMore: Bool? = nil) {
        self.items = items
        self.nextCursor = nextCursor
        self.hasMore = hasMore
    }
}

public struct ContactView: Codable {
    public let tenantId: String?
    public let ownerUserId: String?
    public let targetUserId: String?
    public let contactType: String?
    public let relationshipState: String?
    public let friendshipId: String?
    public let directChatId: String?
    public let conversationId: String?
    public let establishedAt: String?
    public let lastInteractionAt: String?


    public init(tenantId: String? = nil, ownerUserId: String? = nil, targetUserId: String? = nil, contactType: String? = nil, relationshipState: String? = nil, friendshipId: String? = nil, directChatId: String? = nil, conversationId: String? = nil, establishedAt: String? = nil, lastInteractionAt: String? = nil) {
        self.tenantId = tenantId
        self.ownerUserId = ownerUserId
        self.targetUserId = targetUserId
        self.contactType = contactType
        self.relationshipState = relationshipState
        self.friendshipId = friendshipId
        self.directChatId = directChatId
        self.conversationId = conversationId
        self.establishedAt = establishedAt
        self.lastInteractionAt = lastInteractionAt
    }
}

public struct ContactsResponse: Codable {
    public let items: [ContactView]?
    public let nextCursor: String?
    public let hasMore: Bool?


    public init(items: [ContactView]? = nil, nextCursor: String? = nil, hasMore: Bool? = nil) {
        self.items = items
        self.nextCursor = nextCursor
        self.hasMore = hasMore
    }
}

public struct ContactPreferencesView: Codable {
    public let tenantId: String?
    public let ownerUserId: String?
    public let targetUserId: String?
    public let isStarred: Bool?
    public let remark: String?
    public let isBlocked: Bool?
    public let updatedAt: String?


    public init(tenantId: String? = nil, ownerUserId: String? = nil, targetUserId: String? = nil, isStarred: Bool? = nil, remark: String? = nil, isBlocked: Bool? = nil, updatedAt: String? = nil) {
        self.tenantId = tenantId
        self.ownerUserId = ownerUserId
        self.targetUserId = targetUserId
        self.isStarred = isStarred
        self.remark = remark
        self.isBlocked = isBlocked
        self.updatedAt = updatedAt
    }
}

public struct UpdateContactPreferencesRequest: Codable {
    public let isStarred: Bool?
    public let remark: String?
    public let isBlocked: Bool?


    public init(isStarred: Bool? = nil, remark: String? = nil, isBlocked: Bool? = nil) {
        self.isStarred = isStarred
        self.remark = remark
        self.isBlocked = isBlocked
    }
}

public struct ContactTagView: Codable {
    public let tenantId: String?
    public let ownerUserId: String?
    public let tagId: String?
    public let name: String?
    public let color: String?
    public let count: Int?
    public let bg: String?
    public let border: String?
    public let createdAt: String?
    public let updatedAt: String?


    public init(tenantId: String? = nil, ownerUserId: String? = nil, tagId: String? = nil, name: String? = nil, color: String? = nil, count: Int? = nil, bg: String? = nil, border: String? = nil, createdAt: String? = nil, updatedAt: String? = nil) {
        self.tenantId = tenantId
        self.ownerUserId = ownerUserId
        self.tagId = tagId
        self.name = name
        self.color = color
        self.count = count
        self.bg = bg
        self.border = border
        self.createdAt = createdAt
        self.updatedAt = updatedAt
    }
}

public struct ContactTagsResponse: Codable {
    public let items: [ContactTagView]?
    public let nextCursor: String?
    public let hasMore: Bool?


    public init(items: [ContactTagView]? = nil, nextCursor: String? = nil, hasMore: Bool? = nil) {
        self.items = items
        self.nextCursor = nextCursor
        self.hasMore = hasMore
    }
}

public struct CreateContactTagRequest: Codable {
    public let name: String?
    public let color: String?
    public let count: Int?
    public let bg: String?
    public let border: String?


    public init(name: String? = nil, color: String? = nil, count: Int? = nil, bg: String? = nil, border: String? = nil) {
        self.name = name
        self.color = color
        self.count = count
        self.bg = bg
        self.border = border
    }
}

public struct UpdateContactTagRequest: Codable {
    public let name: String?
    public let color: String?
    public let count: Int?
    public let bg: String?
    public let border: String?


    public init(name: String? = nil, color: String? = nil, count: Int? = nil, bg: String? = nil, border: String? = nil) {
        self.name = name
        self.color = color
        self.count = count
        self.bg = bg
        self.border = border
    }
}

public struct DeleteContactTagResponse: Codable {
    public let tagId: String?
    public let deleted: Bool?


    public init(tagId: String? = nil, deleted: Bool? = nil) {
        self.tagId = tagId
        self.deleted = deleted
    }
}

public struct ContactRecommendationView: Codable {
    public let tenantId: String?
    public let ownerUserId: String?
    public let targetUserId: String?
    public let recommendationId: String?
    public let targetConversationId: String?
    public let createdAt: String?


    public init(tenantId: String? = nil, ownerUserId: String? = nil, targetUserId: String? = nil, recommendationId: String? = nil, targetConversationId: String? = nil, createdAt: String? = nil) {
        self.tenantId = tenantId
        self.ownerUserId = ownerUserId
        self.targetUserId = targetUserId
        self.recommendationId = recommendationId
        self.targetConversationId = targetConversationId
        self.createdAt = createdAt
    }
}

public struct CreateContactRecommendationRequest: Codable {
    public let targetConversationId: String?


    public init(targetConversationId: String? = nil) {
        self.targetConversationId = targetConversationId
    }
}

public struct SocialUserSearchResult: Codable {
    public let tenantId: String?
    public let userId: String?
    public let chatId: String?
    public let displayName: String?
    public let relationshipState: String?
    public let avatarUrl: String?
    public let email: String?
    public let phone: String?
    public let metadata: [String: Any]?


    public init(tenantId: String? = nil, userId: String? = nil, chatId: String? = nil, displayName: String? = nil, relationshipState: String? = nil, avatarUrl: String? = nil, email: String? = nil, phone: String? = nil, metadata: [String: Any]? = nil) {
        self.tenantId = tenantId
        self.userId = userId
        self.chatId = chatId
        self.displayName = displayName
        self.relationshipState = relationshipState
        self.avatarUrl = avatarUrl
        self.email = email
        self.phone = phone
        self.metadata = metadata
    }
}

public struct SocialUserSearchResponse: Codable {
    public let items: [SocialUserSearchResult]?
    public let nextCursor: String?
    public let hasMore: Bool?


    public init(items: [SocialUserSearchResult]? = nil, nextCursor: String? = nil, hasMore: Bool? = nil) {
        self.items = items
        self.nextCursor = nextCursor
        self.hasMore = hasMore
    }
}

public struct SubmitFriendRequestRequest: Codable {
    public let targetUserId: String?
    public let requestMessage: String?


    public init(targetUserId: String? = nil, requestMessage: String? = nil) {
        self.targetUserId = targetUserId
        self.requestMessage = requestMessage
    }
}

public struct FriendRequest: Codable {
    public let tenantId: String?
    public let requestId: String?
    public let requesterUserId: String?
    public let targetUserId: String?
    public let status: String?
    public let requestMessage: String?
    public let createdAt: String?
    public let updatedAt: String?


    public init(tenantId: String? = nil, requestId: String? = nil, requesterUserId: String? = nil, targetUserId: String? = nil, status: String? = nil, requestMessage: String? = nil, createdAt: String? = nil, updatedAt: String? = nil) {
        self.tenantId = tenantId
        self.requestId = requestId
        self.requesterUserId = requesterUserId
        self.targetUserId = targetUserId
        self.status = status
        self.requestMessage = requestMessage
        self.createdAt = createdAt
        self.updatedAt = updatedAt
    }
}

public struct Friendship: Codable {
    public let tenantId: String?
    public let friendshipId: String?
    public let initiatorUserId: String?
    public let leftUserId: String?
    public let rightUserId: String?
    public let userHighId: String?
    public let userLowId: String?
    public let status: String?
    public let createdAt: String?


    public init(tenantId: String? = nil, friendshipId: String? = nil, initiatorUserId: String? = nil, leftUserId: String? = nil, rightUserId: String? = nil, userHighId: String? = nil, userLowId: String? = nil, status: String? = nil, createdAt: String? = nil) {
        self.tenantId = tenantId
        self.friendshipId = friendshipId
        self.initiatorUserId = initiatorUserId
        self.leftUserId = leftUserId
        self.rightUserId = rightUserId
        self.userHighId = userHighId
        self.userLowId = userLowId
        self.status = status
        self.createdAt = createdAt
    }
}

public struct DirectChat: Codable {
    public let tenantId: String?
    public let directChatId: String?
    public let conversationId: String?
    public let status: String?


    public init(tenantId: String? = nil, directChatId: String? = nil, conversationId: String? = nil, status: String? = nil) {
        self.tenantId = tenantId
        self.directChatId = directChatId
        self.conversationId = conversationId
        self.status = status
    }
}

public struct SocialFriendRequestMutationResponse: Codable {
    public let friendRequest: FriendRequest?


    public init(friendRequest: FriendRequest? = nil) {
        self.friendRequest = friendRequest
    }
}

public struct SocialFriendRequestListResponse: Codable {
    public let items: [FriendRequest]?
    public let nextCursor: String?


    public init(items: [FriendRequest]? = nil, nextCursor: String? = nil) {
        self.items = items
        self.nextCursor = nextCursor
    }
}

public struct SocialFriendRequestAcceptanceResponse: Codable {
    public let friendRequest: FriendRequest?
    public let friendship: Friendship?
    public let directChat: DirectChat?
    public let conversation: CreateConversationResult?


    public init(friendRequest: FriendRequest? = nil, friendship: Friendship? = nil, directChat: DirectChat? = nil, conversation: CreateConversationResult? = nil) {
        self.friendRequest = friendRequest
        self.friendship = friendship
        self.directChat = directChat
        self.conversation = conversation
    }
}

public struct SocialFriendshipMutationResponse: Codable {
    public let friendship: Friendship?


    public init(friendship: Friendship? = nil) {
        self.friendship = friendship
    }
}

public struct CreateConversationRequest: Codable {
    public let conversationId: String?
    public let conversationType: String?
    public let kind: String?
    public let title: String?
    public let memberIds: [String]?


    public init(conversationId: String? = nil, conversationType: String? = nil, kind: String? = nil, title: String? = nil, memberIds: [String]? = nil) {
        self.conversationId = conversationId
        self.conversationType = conversationType
        self.kind = kind
        self.title = title
        self.memberIds = memberIds
    }
}

public struct CreateAgentDialogRequest: Codable {
    public let agentId: String?
    public let conversationId: String?
    public let title: String?


    public init(agentId: String? = nil, conversationId: String? = nil, title: String? = nil) {
        self.agentId = agentId
        self.conversationId = conversationId
        self.title = title
    }
}

public struct BindDirectChatRequest: Codable {
    public let conversationId: String?
    public let directChatId: String?
    public let leftActorId: String?
    public let leftActorKind: String?
    public let rightActorId: String?
    public let rightActorKind: String?
    public let targetUserId: String?


    public init(conversationId: String? = nil, directChatId: String? = nil, leftActorId: String? = nil, leftActorKind: String? = nil, rightActorId: String? = nil, rightActorKind: String? = nil, targetUserId: String? = nil) {
        self.conversationId = conversationId
        self.directChatId = directChatId
        self.leftActorId = leftActorId
        self.leftActorKind = leftActorKind
        self.rightActorId = rightActorId
        self.rightActorKind = rightActorKind
        self.targetUserId = targetUserId
    }
}

public struct CreateConversationResult: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let kind: String?
    public let createdAt: String?


    public init(tenantId: String? = nil, conversationId: String? = nil, kind: String? = nil, createdAt: String? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.kind = kind
        self.createdAt = createdAt
    }
}

public struct CreateRoomRequest: Codable {
    public let conversationId: String?
    public let roomId: String?
    public let roomKind: String?


    public init(conversationId: String? = nil, roomId: String? = nil, roomKind: String? = nil) {
        self.conversationId = conversationId
        self.roomId = roomId
        self.roomKind = roomKind
    }
}

public struct RoomView: Codable {
    public let roomId: String?
    public let roomKind: String?
    public let conversationId: String?
    public let activeMemberCount: Int?
    public let maxMembers: Int?


    public init(roomId: String? = nil, roomKind: String? = nil, conversationId: String? = nil, activeMemberCount: Int? = nil, maxMembers: Int? = nil) {
        self.roomId = roomId
        self.roomKind = roomKind
        self.conversationId = conversationId
        self.activeMemberCount = activeMemberCount
        self.maxMembers = maxMembers
    }
}

public struct EnterRoomResponse: Codable {
    public let member: ConversationMember?


    public init(member: ConversationMember? = nil) {
        self.member = member
    }
}

public struct AddConversationMemberRequest: Codable {
    public let principalId: String?
    public let principalKind: String?
    public let role: String?
    public let attributes: [String: Any]?


    public init(principalId: String? = nil, principalKind: String? = nil, role: String? = nil, attributes: [String: Any]? = nil) {
        self.principalId = principalId
        self.principalKind = principalKind
        self.role = role
        self.attributes = attributes
    }
}

public struct RemoveConversationMemberRequest: Codable {
    public let memberId: String?


    public init(memberId: String? = nil) {
        self.memberId = memberId
    }
}

public struct TransferConversationOwnerRequest: Codable {
    public let memberId: String?


    public init(memberId: String? = nil) {
        self.memberId = memberId
    }
}

public struct ChangeConversationMemberRoleRequest: Codable {
    public let memberId: String?
    public let role: String?


    public init(memberId: String? = nil, role: String? = nil) {
        self.memberId = memberId
        self.role = role
    }
}

public struct ConversationMember: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let memberId: String?
    public let principalId: String?
    public let principalKind: String?
    public let role: String?
    public let state: String?
    public let joinedAt: String?


    public init(tenantId: String? = nil, conversationId: String? = nil, memberId: String? = nil, principalId: String? = nil, principalKind: String? = nil, role: String? = nil, state: String? = nil, joinedAt: String? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.memberId = memberId
        self.principalId = principalId
        self.principalKind = principalKind
        self.role = role
        self.state = state
        self.joinedAt = joinedAt
    }
}

public struct ListMembersResponse: Codable {
    public let items: [ConversationMember]?
    public let nextCursor: String?
    public let hasMore: Bool?


    public init(items: [ConversationMember]? = nil, nextCursor: String? = nil, hasMore: Bool? = nil) {
        self.items = items
        self.nextCursor = nextCursor
        self.hasMore = hasMore
    }
}

public struct MemberDirectoryResponse: Codable {
    public let items: [ConversationMember]?


    public init(items: [ConversationMember]? = nil) {
        self.items = items
    }
}

public struct ReadCursorView: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let principalId: String?
    public let readSeq: Int?
    public let updatedAt: String?


    public init(tenantId: String? = nil, conversationId: String? = nil, principalId: String? = nil, readSeq: Int? = nil, updatedAt: String? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.principalId = principalId
        self.readSeq = readSeq
        self.updatedAt = updatedAt
    }
}

public struct UpdateReadCursorRequest: Codable {
    public let readSeq: Int?


    public init(readSeq: Int? = nil) {
        self.readSeq = readSeq
    }
}

public struct PinnedMessagesResponse: Codable {
    public let items: [MessageInteractionSummaryView]?


    public init(items: [MessageInteractionSummaryView]? = nil) {
        self.items = items
    }
}

public struct StreamView: Codable {
    public let tenantId: String?
    public let streamId: String?
    public let state: String?
    public let openedAt: String?


    public init(tenantId: String? = nil, streamId: String? = nil, state: String? = nil, openedAt: String? = nil) {
        self.tenantId = tenantId
        self.streamId = streamId
        self.state = state
        self.openedAt = openedAt
    }
}

public struct OpenStreamRequest: Codable {
    public let streamType: String?
    public let conversationId: String?


    public init(streamType: String? = nil, conversationId: String? = nil) {
        self.streamType = streamType
        self.conversationId = conversationId
    }
}

public struct StreamFrameView: Codable {
    public let streamId: String?
    public let frameSeq: Int?
    public let payload: String?
    public let createdAt: String?


    public init(streamId: String? = nil, frameSeq: Int? = nil, payload: String? = nil, createdAt: String? = nil) {
        self.streamId = streamId
        self.frameSeq = frameSeq
        self.payload = payload
        self.createdAt = createdAt
    }
}

public struct StreamFramesResponse: Codable {
    public let items: [StreamFrameView]?
    public let nextCursor: String?
    public let hasMore: Bool?


    public init(items: [StreamFrameView]? = nil, nextCursor: String? = nil, hasMore: Bool? = nil) {
        self.items = items
        self.nextCursor = nextCursor
        self.hasMore = hasMore
    }
}

public struct AppendStreamFrameRequest: Codable {
    public let payload: String?


    public init(payload: String? = nil) {
        self.payload = payload
    }
}

public struct ProblemDetail: Codable {
    public let type: String?
    public let title: String?
    public let status: Int?
    public let detail: String?
    public let code: String?
    public let message: String?
    public let traceId: String?
    public let retryable: Bool?


    public init(type: String? = nil, title: String? = nil, status: Int? = nil, detail: String? = nil, code: String? = nil, message: String? = nil, traceId: String? = nil, retryable: Bool? = nil) {
        self.type = type
        self.title = title
        self.status = status
        self.detail = detail
        self.code = code
        self.message = message
        self.traceId = traceId
        self.retryable = retryable
    }
}

public struct SpaceCreateRequest: Codable {
    public let spaceName: String?
    public let spaceType: String?
    public let description: String?


    public init(spaceName: String? = nil, spaceType: String? = nil, description: String? = nil) {
        self.spaceName = spaceName
        self.spaceType = spaceType
        self.description = description
    }
}

public struct SpaceUpdateRequest: Codable {
    public let spaceName: String?
    public let description: String?


    public init(spaceName: String? = nil, description: String? = nil) {
        self.spaceName = spaceName
        self.description = description
    }
}

public struct SpaceView: Codable {
    public let spaceId: String?
    public let spaceName: String?
    public let spaceType: String?
    public let ownerUserId: String?
    public let createdAt: String?


    public init(spaceId: String? = nil, spaceName: String? = nil, spaceType: String? = nil, ownerUserId: String? = nil, createdAt: String? = nil) {
        self.spaceId = spaceId
        self.spaceName = spaceName
        self.spaceType = spaceType
        self.ownerUserId = ownerUserId
        self.createdAt = createdAt
    }
}

public struct SpaceListResponse: Codable {
    public let items: [SpaceView]?


    public init(items: [SpaceView]? = nil) {
        self.items = items
    }
}

public struct SpaceMemberCreateRequest: Codable {
    public let userId: String?
    public let role: String?


    public init(userId: String? = nil, role: String? = nil) {
        self.userId = userId
        self.role = role
    }
}

public struct SpaceMemberUpdateRequest: Codable {
    public let role: String?


    public init(role: String? = nil) {
        self.role = role
    }
}

public struct SpaceMemberView: Codable {
    public let userId: String?
    public let role: String?


    public init(userId: String? = nil, role: String? = nil) {
        self.userId = userId
        self.role = role
    }
}

public struct SpaceMemberListResponse: Codable {
    public let items: [SpaceMemberView]?


    public init(items: [SpaceMemberView]? = nil) {
        self.items = items
    }
}

public struct SpaceGroupCreateRequest: Codable {
    public let groupName: String?
    public let description: String?


    public init(groupName: String? = nil, description: String? = nil) {
        self.groupName = groupName
        self.description = description
    }
}

public struct SpaceGroupUpdateRequest: Codable {
    public let groupName: String?
    public let description: String?


    public init(groupName: String? = nil, description: String? = nil) {
        self.groupName = groupName
        self.description = description
    }
}

public struct SpaceGroupView: Codable {
    public let groupId: String?
    public let groupName: String?


    public init(groupId: String? = nil, groupName: String? = nil) {
        self.groupId = groupId
        self.groupName = groupName
    }
}

public struct SpaceGroupListResponse: Codable {
    public let items: [SpaceGroupView]?


    public init(items: [SpaceGroupView]? = nil) {
        self.items = items
    }
}

public struct SpaceGroupMemberCreateRequest: Codable {
    public let userId: String?
    public let role: String?
    public let nickname: String?


    public init(userId: String? = nil, role: String? = nil, nickname: String? = nil) {
        self.userId = userId
        self.role = role
        self.nickname = nickname
    }
}

public struct SpaceGroupMemberUpdateRequest: Codable {
    public let role: String?
    public let nickname: String?
    public let muteUntil: String?


    public init(role: String? = nil, nickname: String? = nil, muteUntil: String? = nil) {
        self.role = role
        self.nickname = nickname
        self.muteUntil = muteUntil
    }
}

public struct SpaceGroupMemberView: Codable {
    public let userId: String?
    public let role: String?
    public let nickname: String?
    public let muteUntil: String?
    public let joinedAt: String?


    public init(userId: String? = nil, role: String? = nil, nickname: String? = nil, muteUntil: String? = nil, joinedAt: String? = nil) {
        self.userId = userId
        self.role = role
        self.nickname = nickname
        self.muteUntil = muteUntil
        self.joinedAt = joinedAt
    }
}

public struct SpaceGroupMemberListResponse: Codable {
    public let items: [SpaceGroupMemberView]?


    public init(items: [SpaceGroupMemberView]? = nil) {
        self.items = items
    }
}

public struct SpaceChannelCreateRequest: Codable {
    public let channelName: String?
    public let channelType: String?


    public init(channelName: String? = nil, channelType: String? = nil) {
        self.channelName = channelName
        self.channelType = channelType
    }
}

public struct SpaceChannelUpdateRequest: Codable {
    public let channelName: String?


    public init(channelName: String? = nil) {
        self.channelName = channelName
    }
}

public struct SpaceChannelView: Codable {
    public let channelId: String?
    public let channelName: String?
    public let channelType: String?


    public init(channelId: String? = nil, channelName: String? = nil, channelType: String? = nil) {
        self.channelId = channelId
        self.channelName = channelName
        self.channelType = channelType
    }
}

public struct SpaceChannelListResponse: Codable {
    public let items: [SpaceChannelView]?


    public init(items: [SpaceChannelView]? = nil) {
        self.items = items
    }
}

public struct SpaceChannelAccessRuleCreateRequest: Codable {
    public let ruleType: String?
    public let principalKind: String?
    public let principalId: String?
    public let permission: String?


    public init(ruleType: String? = nil, principalKind: String? = nil, principalId: String? = nil, permission: String? = nil) {
        self.ruleType = ruleType
        self.principalKind = principalKind
        self.principalId = principalId
        self.permission = permission
    }
}

public struct SpaceChannelAccessRuleView: Codable {
    public let ruleId: String?
    public let channelId: String?
    public let ruleType: String?
    public let principalKind: String?
    public let principalId: String?
    public let permission: String?
    public let createdAt: String?


    public init(ruleId: String? = nil, channelId: String? = nil, ruleType: String? = nil, principalKind: String? = nil, principalId: String? = nil, permission: String? = nil, createdAt: String? = nil) {
        self.ruleId = ruleId
        self.channelId = channelId
        self.ruleType = ruleType
        self.principalKind = principalKind
        self.principalId = principalId
        self.permission = permission
        self.createdAt = createdAt
    }
}

public struct SpaceChannelAccessRuleListResponse: Codable {
    public let items: [SpaceChannelAccessRuleView]?


    public init(items: [SpaceChannelAccessRuleView]? = nil) {
        self.items = items
    }
}

public struct SpaceInviteCreateRequest: Codable {
    public let maxUses: Int?


    public init(maxUses: Int? = nil) {
        self.maxUses = maxUses
    }
}

public struct SpaceInviteView: Codable {
    public let inviteCode: String?
    public let spaceId: String?


    public init(inviteCode: String? = nil, spaceId: String? = nil) {
        self.inviteCode = inviteCode
        self.spaceId = spaceId
    }
}

public struct SpaceInviteListResponse: Codable {
    public let items: [SpaceInviteView]?


    public init(items: [SpaceInviteView]? = nil) {
        self.items = items
    }
}

public struct SpaceBanCreateRequest: Codable {
    public let userId: String?
    public let reason: String?


    public init(userId: String? = nil, reason: String? = nil) {
        self.userId = userId
        self.reason = reason
    }
}

public struct SpaceBanView: Codable {
    public let userId: String?
    public let reason: String?


    public init(userId: String? = nil, reason: String? = nil) {
        self.userId = userId
        self.reason = reason
    }
}

public struct SpaceBanListResponse: Codable {
    public let items: [SpaceBanView]?


    public init(items: [SpaceBanView]? = nil) {
        self.items = items
    }
}

public struct TextContentPart: Codable {
    public let kind: String
    public let text: String


    public init(kind: String, text: String) {
        self.kind = kind
        self.text = text
    }
}

public struct DataContentPart: Codable {
    public let kind: String
    public let schemaRef: String
    public let encoding: String
    public let payload: String


    public init(kind: String, schemaRef: String, encoding: String, payload: String) {
        self.kind = kind
        self.schemaRef = schemaRef
        self.encoding = encoding
        self.payload = payload
    }
}

public struct MediaContentPart: Codable {
    public let kind: String
    public let drive: DriveReference
    public let resource: MediaResource
    public let mediaRole: String?


    public init(kind: String, drive: DriveReference, resource: MediaResource, mediaRole: String? = nil) {
        self.kind = kind
        self.drive = drive
        self.resource = resource
        self.mediaRole = mediaRole
    }
}

public struct SignalContentPart: Codable {
    public let kind: String
    public let signalType: String
    public let schemaRef: String?
    public let payload: String


    public init(kind: String, signalType: String, schemaRef: String? = nil, payload: String) {
        self.kind = kind
        self.signalType = signalType
        self.schemaRef = schemaRef
        self.payload = payload
    }
}

public struct StreamRefContentPart: Codable {
    public let kind: String
    public let streamId: String
    public let streamType: String
    public let state: String


    public init(kind: String, streamId: String, streamType: String, state: String) {
        self.kind = kind
        self.streamId = streamId
        self.streamType = streamType
        self.state = state
    }
}
