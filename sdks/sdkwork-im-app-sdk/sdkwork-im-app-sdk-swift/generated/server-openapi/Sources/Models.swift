import Foundation

public struct PortalWorkspaceView: Codable {
    public let name: String?
    public let slug: String?
    public let tier: String?
    public let region: String?
    public let supportPlan: String?
    public let seats: Int?
    public let activeBrands: Int?
    public let uptime: String?


    public init(name: String? = nil, slug: String? = nil, tier: String? = nil, region: String? = nil, supportPlan: String? = nil, seats: Int? = nil, activeBrands: Int? = nil, uptime: String? = nil) {
        self.name = name
        self.slug = slug
        self.tier = tier
        self.region = region
        self.supportPlan = supportPlan
        self.seats = seats
        self.activeBrands = activeBrands
        self.uptime = uptime
    }
}

public struct Sender: Codable {
    public let id: String?
    public let kind: String?
    public let memberId: String?
    public let deviceId: String?
    public let sessionId: String?
    public let metadata: [String: String]?


    public init(id: String? = nil, kind: String? = nil, memberId: String? = nil, deviceId: String? = nil, sessionId: String? = nil, metadata: [String: String]? = nil) {
        self.id = id
        self.kind = kind
        self.memberId = memberId
        self.deviceId = deviceId
        self.sessionId = sessionId
        self.metadata = metadata
    }
}

public struct StreamSession: Codable {
    public let tenantId: String?
    public let streamId: String?
    public let streamType: String?
    public let scopeKind: String?
    public let scopeId: String?
    public let durabilityClass: String?
    public let orderingScope: String?
    public let schemaRef: String?
    public let state: String?
    public let lastFrameSeq: Int?
    public let lastCheckpointSeq: Int?
    public let resultMessageId: String?
    public let openedAt: String?
    public let closedAt: String?
    public let expiresAt: String?


    public init(tenantId: String? = nil, streamId: String? = nil, streamType: String? = nil, scopeKind: String? = nil, scopeId: String? = nil, durabilityClass: String? = nil, orderingScope: String? = nil, schemaRef: String? = nil, state: String? = nil, lastFrameSeq: Int? = nil, lastCheckpointSeq: Int? = nil, resultMessageId: String? = nil, openedAt: String? = nil, closedAt: String? = nil, expiresAt: String? = nil) {
        self.tenantId = tenantId
        self.streamId = streamId
        self.streamType = streamType
        self.scopeKind = scopeKind
        self.scopeId = scopeId
        self.durabilityClass = durabilityClass
        self.orderingScope = orderingScope
        self.schemaRef = schemaRef
        self.state = state
        self.lastFrameSeq = lastFrameSeq
        self.lastCheckpointSeq = lastCheckpointSeq
        self.resultMessageId = resultMessageId
        self.openedAt = openedAt
        self.closedAt = closedAt
        self.expiresAt = expiresAt
    }
}

public struct StreamFrame: Codable {
    public let tenantId: String?
    public let streamId: String?
    public let streamType: String?
    public let scopeKind: String?
    public let scopeId: String?
    public let frameSeq: Int?
    public let frameType: String?
    public let schemaRef: String?
    public let encoding: String?
    public let payload: String?
    public let sender: Sender?
    public let attributes: [String: String]?
    public let occurredAt: String?


    public init(tenantId: String? = nil, streamId: String? = nil, streamType: String? = nil, scopeKind: String? = nil, scopeId: String? = nil, frameSeq: Int? = nil, frameType: String? = nil, schemaRef: String? = nil, encoding: String? = nil, payload: String? = nil, sender: Sender? = nil, attributes: [String: String]? = nil, occurredAt: String? = nil) {
        self.tenantId = tenantId
        self.streamId = streamId
        self.streamType = streamType
        self.scopeKind = scopeKind
        self.scopeId = scopeId
        self.frameSeq = frameSeq
        self.frameType = frameType
        self.schemaRef = schemaRef
        self.encoding = encoding
        self.payload = payload
        self.sender = sender
        self.attributes = attributes
        self.occurredAt = occurredAt
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

public struct AgentSubject: Codable {
    public let agentId: String?
    public let sessionId: String?
    public let metadata: [String: String]?


    public init(agentId: String? = nil, sessionId: String? = nil, metadata: [String: String]? = nil) {
        self.agentId = agentId
        self.sessionId = sessionId
        self.metadata = metadata
    }
}

public struct AgentToolCall: Codable {
    public let tenantId: String?
    public let executionId: String?
    public let agentId: String?
    public let toolCallId: String?
    public let toolName: String?
    public let argumentsPayload: String?
    public let resultPayload: String?
    public let state: String?
    public let requestedAt: String?
    public let completedAt: String?


    public init(tenantId: String? = nil, executionId: String? = nil, agentId: String? = nil, toolCallId: String? = nil, toolName: String? = nil, argumentsPayload: String? = nil, resultPayload: String? = nil, state: String? = nil, requestedAt: String? = nil, completedAt: String? = nil) {
        self.tenantId = tenantId
        self.executionId = executionId
        self.agentId = agentId
        self.toolCallId = toolCallId
        self.toolName = toolName
        self.argumentsPayload = argumentsPayload
        self.resultPayload = resultPayload
        self.state = state
        self.requestedAt = requestedAt
        self.completedAt = completedAt
    }
}

public struct AppendAgentResponseDeltaRequest: Codable {
    public let frameSeq: Int?
    public let frameType: String?
    public let schemaRef: String?
    public let encoding: String?
    public let payload: String?
    public let attributes: [String: String]?


    public init(frameSeq: Int? = nil, frameType: String? = nil, schemaRef: String? = nil, encoding: String? = nil, payload: String? = nil, attributes: [String: String]? = nil) {
        self.frameSeq = frameSeq
        self.frameType = frameType
        self.schemaRef = schemaRef
        self.encoding = encoding
        self.payload = payload
        self.attributes = attributes
    }
}

public struct AutomationExecution: Codable {
    public let tenantId: String?
    public let principalId: String?
    public let principalKind: String?
    public let executionId: String?
    public let triggerType: String?
    public let targetKind: String?
    public let targetRef: String?
    public let inputPayload: String?
    public let outputPayload: String?
    public let state: String?
    public let retryCount: Int?
    public let requestedAt: String?
    public let completedAt: String?
    public let failureReason: String?


    public init(tenantId: String? = nil, principalId: String? = nil, principalKind: String? = nil, executionId: String? = nil, triggerType: String? = nil, targetKind: String? = nil, targetRef: String? = nil, inputPayload: String? = nil, outputPayload: String? = nil, state: String? = nil, retryCount: Int? = nil, requestedAt: String? = nil, completedAt: String? = nil, failureReason: String? = nil) {
        self.tenantId = tenantId
        self.principalId = principalId
        self.principalKind = principalKind
        self.executionId = executionId
        self.triggerType = triggerType
        self.targetKind = targetKind
        self.targetRef = targetRef
        self.inputPayload = inputPayload
        self.outputPayload = outputPayload
        self.state = state
        self.retryCount = retryCount
        self.requestedAt = requestedAt
        self.completedAt = completedAt
        self.failureReason = failureReason
    }
}

public struct AutomationExecutionRequestResponse: Codable {
    public let tenantId: String?
    public let principalId: String?
    public let principalKind: String?
    public let executionId: String?
    public let triggerType: String?
    public let targetKind: String?
    public let targetRef: String?
    public let inputPayload: String?
    public let outputPayload: String?
    public let state: String?
    public let retryCount: Int?
    public let requestedAt: String?
    public let completedAt: String?
    public let failureReason: String?
    public let requestKey: String?
    public let deliveryStatus: String?
    public let proofVersion: String?


    public init(tenantId: String? = nil, principalId: String? = nil, principalKind: String? = nil, executionId: String? = nil, triggerType: String? = nil, targetKind: String? = nil, targetRef: String? = nil, inputPayload: String? = nil, outputPayload: String? = nil, state: String? = nil, retryCount: Int? = nil, requestedAt: String? = nil, completedAt: String? = nil, failureReason: String? = nil, requestKey: String? = nil, deliveryStatus: String? = nil, proofVersion: String? = nil) {
        self.tenantId = tenantId
        self.principalId = principalId
        self.principalKind = principalKind
        self.executionId = executionId
        self.triggerType = triggerType
        self.targetKind = targetKind
        self.targetRef = targetRef
        self.inputPayload = inputPayload
        self.outputPayload = outputPayload
        self.state = state
        self.retryCount = retryCount
        self.requestedAt = requestedAt
        self.completedAt = completedAt
        self.failureReason = failureReason
        self.requestKey = requestKey
        self.deliveryStatus = deliveryStatus
        self.proofVersion = proofVersion
    }
}

public struct CompleteAgentResponseRequest: Codable {
    public let frameSeq: Int?
    public let resultMessageId: String?


    public init(frameSeq: Int? = nil, resultMessageId: String? = nil) {
        self.frameSeq = frameSeq
        self.resultMessageId = resultMessageId
    }
}

public struct CompleteAgentToolCallRequest: Codable {
    public let resultPayload: String?


    public init(resultPayload: String? = nil) {
        self.resultPayload = resultPayload
    }
}

public struct NotificationTask: Codable {
    public let tenantId: String?
    public let notificationId: String?
    public let sourceEventId: String?
    public let sourceEventType: String?
    public let category: String?
    public let channel: String?
    public let recipientId: String?
    public let recipientKind: String?
    public let status: String?
    public let title: String?
    public let body: String?
    public let payload: String?
    public let requestedAt: String?
    public let dispatchedAt: String?
    public let failureReason: String?


    public init(tenantId: String? = nil, notificationId: String? = nil, sourceEventId: String? = nil, sourceEventType: String? = nil, category: String? = nil, channel: String? = nil, recipientId: String? = nil, recipientKind: String? = nil, status: String? = nil, title: String? = nil, body: String? = nil, payload: String? = nil, requestedAt: String? = nil, dispatchedAt: String? = nil, failureReason: String? = nil) {
        self.tenantId = tenantId
        self.notificationId = notificationId
        self.sourceEventId = sourceEventId
        self.sourceEventType = sourceEventType
        self.category = category
        self.channel = channel
        self.recipientId = recipientId
        self.recipientKind = recipientKind
        self.status = status
        self.title = title
        self.body = body
        self.payload = payload
        self.requestedAt = requestedAt
        self.dispatchedAt = dispatchedAt
        self.failureReason = failureReason
    }
}

public struct NotificationListResponse: Codable {
    public let items: [NotificationTask]?


    public init(items: [NotificationTask]? = nil) {
        self.items = items
    }
}

public struct NotificationRequestResponse: Codable {
    public let tenantId: String?
    public let notificationId: String?
    public let sourceEventId: String?
    public let sourceEventType: String?
    public let category: String?
    public let channel: String?
    public let recipientId: String?
    public let recipientKind: String?
    public let status: String?
    public let title: String?
    public let body: String?
    public let payload: String?
    public let requestedAt: String?
    public let dispatchedAt: String?
    public let failureReason: String?
    public let requestKey: String?
    public let deliveryStatus: String?
    public let proofVersion: String?


    public init(tenantId: String? = nil, notificationId: String? = nil, sourceEventId: String? = nil, sourceEventType: String? = nil, category: String? = nil, channel: String? = nil, recipientId: String? = nil, recipientKind: String? = nil, status: String? = nil, title: String? = nil, body: String? = nil, payload: String? = nil, requestedAt: String? = nil, dispatchedAt: String? = nil, failureReason: String? = nil, requestKey: String? = nil, deliveryStatus: String? = nil, proofVersion: String? = nil) {
        self.tenantId = tenantId
        self.notificationId = notificationId
        self.sourceEventId = sourceEventId
        self.sourceEventType = sourceEventType
        self.category = category
        self.channel = channel
        self.recipientId = recipientId
        self.recipientKind = recipientKind
        self.status = status
        self.title = title
        self.body = body
        self.payload = payload
        self.requestedAt = requestedAt
        self.dispatchedAt = dispatchedAt
        self.failureReason = failureReason
        self.requestKey = requestKey
        self.deliveryStatus = deliveryStatus
        self.proofVersion = proofVersion
    }
}

public struct RequestAgentToolCallRequest: Codable {
    public let executionId: String?
    public let toolCallId: String?
    public let toolName: String?
    public let argumentsPayload: String?


    public init(executionId: String? = nil, toolCallId: String? = nil, toolName: String? = nil, argumentsPayload: String? = nil) {
        self.executionId = executionId
        self.toolCallId = toolCallId
        self.toolName = toolName
        self.argumentsPayload = argumentsPayload
    }
}

public struct RequestAutomationExecution: Codable {
    public let executionId: String?
    public let triggerType: String?
    public let targetKind: String?
    public let targetRef: String?
    public let inputPayload: String?


    public init(executionId: String? = nil, triggerType: String? = nil, targetKind: String? = nil, targetRef: String? = nil, inputPayload: String? = nil) {
        self.executionId = executionId
        self.triggerType = triggerType
        self.targetKind = targetKind
        self.targetRef = targetRef
        self.inputPayload = inputPayload
    }
}

public struct RequestNotification: Codable {
    public let notificationId: String?
    public let sourceEventId: String?
    public let sourceEventType: String?
    public let category: String?
    public let channel: String?
    public let recipientId: String?
    public let recipientKind: String?
    public let title: String?
    public let body: String?
    public let payload: String?


    public init(notificationId: String? = nil, sourceEventId: String? = nil, sourceEventType: String? = nil, category: String? = nil, channel: String? = nil, recipientId: String? = nil, recipientKind: String? = nil, title: String? = nil, body: String? = nil, payload: String? = nil) {
        self.notificationId = notificationId
        self.sourceEventId = sourceEventId
        self.sourceEventType = sourceEventType
        self.category = category
        self.channel = channel
        self.recipientId = recipientId
        self.recipientKind = recipientKind
        self.title = title
        self.body = body
        self.payload = payload
    }
}

public struct StartAgentResponseRequest: Codable {
    public let executionId: String?
    public let streamId: String?
    public let streamType: String?
    public let conversationId: String?
    public let schemaRef: String?
    public let memberId: String?
    public let agent: AgentSubject?


    public init(executionId: String? = nil, streamId: String? = nil, streamType: String? = nil, conversationId: String? = nil, schemaRef: String? = nil, memberId: String? = nil, agent: AgentSubject? = nil) {
        self.executionId = executionId
        self.streamId = streamId
        self.streamType = streamType
        self.conversationId = conversationId
        self.schemaRef = schemaRef
        self.memberId = memberId
        self.agent = agent
    }
}
