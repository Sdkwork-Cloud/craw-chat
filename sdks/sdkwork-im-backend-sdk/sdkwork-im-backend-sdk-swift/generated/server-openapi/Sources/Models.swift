import Foundation

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

public struct ActivateFriendshipRequest: Codable {
    public let directChatId: String?
    public let establishedAt: String?
    public let eventId: String?
    public let friendshipId: String?
    public let initiatorUserId: String?
    public let peerUserId: String?


    public init(directChatId: String? = nil, establishedAt: String? = nil, eventId: String? = nil, friendshipId: String? = nil, initiatorUserId: String? = nil, peerUserId: String? = nil) {
        self.directChatId = directChatId
        self.establishedAt = establishedAt
        self.eventId = eventId
        self.friendshipId = friendshipId
        self.initiatorUserId = initiatorUserId
        self.peerUserId = peerUserId
    }
}

public struct ApplySharedChannelPolicyRequest: Codable {
    public let appliedAt: String?
    public let channelId: String?
    public let connectionId: String?
    public let conversationId: String?
    public let eventId: String?
    public let historyVisibility: String?
    public let policyId: String?
    public let policyVersion: Int?


    public init(appliedAt: String? = nil, channelId: String? = nil, connectionId: String? = nil, conversationId: String? = nil, eventId: String? = nil, historyVisibility: String? = nil, policyId: String? = nil, policyVersion: Int? = nil) {
        self.appliedAt = appliedAt
        self.channelId = channelId
        self.connectionId = connectionId
        self.conversationId = conversationId
        self.eventId = eventId
        self.historyVisibility = historyVisibility
        self.policyId = policyId
        self.policyVersion = policyVersion
    }
}

public struct BindDirectChatRequest: Codable {
    public let boundAt: String?
    public let conversationId: String?
    public let directChatId: String?
    public let eventId: String?
    public let leftActorId: String?
    public let rightActorId: String?


    public init(boundAt: String? = nil, conversationId: String? = nil, directChatId: String? = nil, eventId: String? = nil, leftActorId: String? = nil, rightActorId: String? = nil) {
        self.boundAt = boundAt
        self.conversationId = conversationId
        self.directChatId = directChatId
        self.eventId = eventId
        self.leftActorId = leftActorId
        self.rightActorId = rightActorId
    }
}

public struct BindExternalMemberLinkRequest: Codable {
    public let connectionId: String?
    public let eventId: String?
    public let externalDisplayName: String?
    public let externalMemberId: String?
    public let linkId: String?
    public let linkedAt: String?
    public let localActorId: String?
    public let localActorKind: String?


    public init(connectionId: String? = nil, eventId: String? = nil, externalDisplayName: String? = nil, externalMemberId: String? = nil, linkId: String? = nil, linkedAt: String? = nil, localActorId: String? = nil, localActorKind: String? = nil) {
        self.connectionId = connectionId
        self.eventId = eventId
        self.externalDisplayName = externalDisplayName
        self.externalMemberId = externalMemberId
        self.linkId = linkId
        self.linkedAt = linkedAt
        self.localActorId = localActorId
        self.localActorKind = localActorKind
    }
}

public struct BlockUserRequest: Codable {
    public let blockId: String?
    public let blockedUserId: String?
    public let blockerUserId: String?
    public let directChatId: String?
    public let effectiveAt: String?
    public let eventId: String?
    public let expiresAt: String?
    public let scope: String?


    public init(blockId: String? = nil, blockedUserId: String? = nil, blockerUserId: String? = nil, directChatId: String? = nil, effectiveAt: String? = nil, eventId: String? = nil, expiresAt: String? = nil, scope: String? = nil) {
        self.blockId = blockId
        self.blockedUserId = blockedUserId
        self.blockerUserId = blockerUserId
        self.directChatId = directChatId
        self.effectiveAt = effectiveAt
        self.eventId = eventId
        self.expiresAt = expiresAt
        self.scope = scope
    }
}

public struct BusinessPolicyVocabularyResponse: Codable {
    public let capabilityFlagsField: String?
    public let historyVisibilityField: String?
    public let historyVisibilityModes: [String]?
    public let policyVersionField: String?
    public let retentionPolicyRefField: String?
    public let retentionPolicyScopes: [String]?


    public init(capabilityFlagsField: String? = nil, historyVisibilityField: String? = nil, historyVisibilityModes: [String]? = nil, policyVersionField: String? = nil, retentionPolicyRefField: String? = nil, retentionPolicyScopes: [String]? = nil) {
        self.capabilityFlagsField = capabilityFlagsField
        self.historyVisibilityField = historyVisibilityField
        self.historyVisibilityModes = historyVisibilityModes
        self.policyVersionField = policyVersionField
        self.retentionPolicyRefField = retentionPolicyRefField
        self.retentionPolicyScopes = retentionPolicyScopes
    }
}

public struct CapabilityProfileResponse: Codable {
    public let enabledCapabilities: [String]?
    public let experimentalCapabilities: [String]?
    public let profileId: String?
    public let releaseChannel: String?


    public init(enabledCapabilities: [String]? = nil, experimentalCapabilities: [String]? = nil, profileId: String? = nil, releaseChannel: String? = nil) {
        self.enabledCapabilities = enabledCapabilities
        self.experimentalCapabilities = experimentalCapabilities
        self.profileId = profileId
        self.releaseChannel = releaseChannel
    }
}

public struct ClientCompatibilityResponse: Codable {
    public let blockedExperimentalCapabilities: [String]?
    public let clientType: String?
    public let minimumProtocolVersion: String?
    public let supportedBindings: [String]?
    public let supportedCapabilities: [String]?
    public let supportedCodecs: [String]?


    public init(blockedExperimentalCapabilities: [String]? = nil, clientType: String? = nil, minimumProtocolVersion: String? = nil, supportedBindings: [String]? = nil, supportedCapabilities: [String]? = nil, supportedCodecs: [String]? = nil) {
        self.blockedExperimentalCapabilities = blockedExperimentalCapabilities
        self.clientType = clientType
        self.minimumProtocolVersion = minimumProtocolVersion
        self.supportedBindings = supportedBindings
        self.supportedCapabilities = supportedCapabilities
        self.supportedCodecs = supportedCodecs
    }
}

public struct EffectiveProtocolSnapshotResponse: Codable {
    public let allowedBindings: [String]?
    public let allowedCodecs: [String]?
    public let enabledCapabilities: [String]?
    public let killSwitchActive: Bool?
    public let precedence_: [String]?
    public let protocolVersion: String?
    public let quotaProfileId: String?
    public let releaseChannel: String?


    public init(allowedBindings: [String]? = nil, allowedCodecs: [String]? = nil, enabledCapabilities: [String]? = nil, killSwitchActive: Bool? = nil, precedence_: [String]? = nil, protocolVersion: String? = nil, quotaProfileId: String? = nil, releaseChannel: String? = nil) {
        self.allowedBindings = allowedBindings
        self.allowedCodecs = allowedCodecs
        self.enabledCapabilities = enabledCapabilities
        self.killSwitchActive = killSwitchActive
        self.precedence_ = precedence_
        self.protocolVersion = protocolVersion
        self.quotaProfileId = quotaProfileId
        self.releaseChannel = releaseChannel
    }
}

public struct EstablishExternalConnectionRequest: Codable {
    public let connectionId: String?
    public let connectionKind: String?
    public let establishedAt: String?
    public let eventId: String?
    public let externalOrgName: String?
    public let externalTenantId: String?


    public init(connectionId: String? = nil, connectionKind: String? = nil, establishedAt: String? = nil, eventId: String? = nil, externalOrgName: String? = nil, externalTenantId: String? = nil) {
        self.connectionId = connectionId
        self.connectionKind = connectionKind
        self.establishedAt = establishedAt
        self.eventId = eventId
        self.externalOrgName = externalOrgName
        self.externalTenantId = externalTenantId
    }
}

public struct KillSwitchResponse: Codable {
    public let active: Bool?
    public let disabledBindings: [String]?
    public let disabledCapabilities: [String]?
    public let disabledCodecs: [String]?
    public let reason: String?
    public let ruleId: String?


    public init(active: Bool? = nil, disabledBindings: [String]? = nil, disabledCapabilities: [String]? = nil, disabledCodecs: [String]? = nil, reason: String? = nil, ruleId: String? = nil) {
        self.active = active
        self.disabledBindings = disabledBindings
        self.disabledCapabilities = disabledCapabilities
        self.disabledCodecs = disabledCodecs
        self.reason = reason
        self.ruleId = ruleId
    }
}

public struct MigrateRoutesRequest: Codable {
    public let targetNodeId: String?


    public init(targetNodeId: String? = nil) {
        self.targetNodeId = targetNodeId
    }
}

public struct ProtocolGovernanceResponse: Codable {
    public let businessPolicyVocabulary: BusinessPolicyVocabularyResponse?
    public let capabilityProfile: CapabilityProfileResponse?
    public let effectiveSnapshot: EffectiveProtocolSnapshotResponse?
    public let killSwitch: KillSwitchResponse?
    public let quotaProfile: QuotaProfileResponse?
    public let rolloutPolicy: RolloutPolicyResponse?
    public let sdkCompatibilityBaseline: SdkCompatibilityBaselineResponse?


    public init(businessPolicyVocabulary: BusinessPolicyVocabularyResponse? = nil, capabilityProfile: CapabilityProfileResponse? = nil, effectiveSnapshot: EffectiveProtocolSnapshotResponse? = nil, killSwitch: KillSwitchResponse? = nil, quotaProfile: QuotaProfileResponse? = nil, rolloutPolicy: RolloutPolicyResponse? = nil, sdkCompatibilityBaseline: SdkCompatibilityBaselineResponse? = nil) {
        self.businessPolicyVocabulary = businessPolicyVocabulary
        self.capabilityProfile = capabilityProfile
        self.effectiveSnapshot = effectiveSnapshot
        self.killSwitch = killSwitch
        self.quotaProfile = quotaProfile
        self.rolloutPolicy = rolloutPolicy
        self.sdkCompatibilityBaseline = sdkCompatibilityBaseline
    }
}

public struct ProtocolRegistryResponse: Codable {
    public let bindings: [String]?
    public let codecs: [String]?
    public let compatibilityMatrix: [ClientCompatibilityResponse]?
    public let protocolVersion: String?
    public let schemas: [ProtocolSchemaResponse]?


    public init(bindings: [String]? = nil, codecs: [String]? = nil, compatibilityMatrix: [ClientCompatibilityResponse]? = nil, protocolVersion: String? = nil, schemas: [ProtocolSchemaResponse]? = nil) {
        self.bindings = bindings
        self.codecs = codecs
        self.compatibilityMatrix = compatibilityMatrix
        self.protocolVersion = protocolVersion
        self.schemas = schemas
    }
}

public struct ProtocolSchemaResponse: Codable {
    public let bindingProtocols: [String]?
    public let kind: String?
    public let requiredCapabilities: [String]?
    public let schema: String?
    public let stage: String?
    public let supportedConsumers: [String]?


    public init(bindingProtocols: [String]? = nil, kind: String? = nil, requiredCapabilities: [String]? = nil, schema: String? = nil, stage: String? = nil, supportedConsumers: [String]? = nil) {
        self.bindingProtocols = bindingProtocols
        self.kind = kind
        self.requiredCapabilities = requiredCapabilities
        self.schema = schema
        self.stage = stage
        self.supportedConsumers = supportedConsumers
    }
}

public struct ProviderBindingCommitResponse: Codable {

    public init() {}
}

public struct ProviderBindingsResponse: Codable {

    public init() {}
}

public struct ProviderPolicyDiffResponse: Codable {

    public init() {}
}

public struct ProviderPolicyHistoryResponse: Codable {

    public init() {}
}

public struct ProviderPolicyRollbackRequest: Codable {
    public let targetVersion: Int?


    public init(targetVersion: Int? = nil) {
        self.targetVersion = targetVersion
    }
}

public struct ProviderRegistrySnapshotResponse: Codable {

    public init() {}
}

public struct QuotaProfileResponse: Codable {
    public let maxConcurrentSessionsPerTenant: Int?
    public let maxInflightMessages: Int?
    public let maxPayloadBytes: Int?
    public let maxSubscriptionsPerSession: Int?
    public let profileId: String?


    public init(maxConcurrentSessionsPerTenant: Int? = nil, maxInflightMessages: Int? = nil, maxPayloadBytes: Int? = nil, maxSubscriptionsPerSession: Int? = nil, profileId: String? = nil) {
        self.maxConcurrentSessionsPerTenant = maxConcurrentSessionsPerTenant
        self.maxInflightMessages = maxInflightMessages
        self.maxPayloadBytes = maxPayloadBytes
        self.maxSubscriptionsPerSession = maxSubscriptionsPerSession
        self.profileId = profileId
    }
}

public struct RolloutPolicyResponse: Codable {
    public let cellSelector: String?
    public let operatorOverride: Bool?
    public let policyId: String?
    public let regionSelector: String?
    public let releaseChannel: String?
    public let tenantAllowlist: [String]?
    public let trafficPercent: Int?


    public init(cellSelector: String? = nil, operatorOverride: Bool? = nil, policyId: String? = nil, regionSelector: String? = nil, releaseChannel: String? = nil, tenantAllowlist: [String]? = nil, trafficPercent: Int? = nil) {
        self.cellSelector = cellSelector
        self.operatorOverride = operatorOverride
        self.policyId = policyId
        self.regionSelector = regionSelector
        self.releaseChannel = releaseChannel
        self.tenantAllowlist = tenantAllowlist
        self.trafficPercent = trafficPercent
    }
}

public struct RouteMigrationResult: Codable {
    public let migratedRouteCount: Int?
    public let sourceDrainStatus: String?
    public let sourceNodeId: String?
    public let sourceRebalanceState: String?
    public let targetDrainStatus: String?
    public let targetNodeId: String?
    public let targetRebalanceState: String?


    public init(migratedRouteCount: Int? = nil, sourceDrainStatus: String? = nil, sourceNodeId: String? = nil, sourceRebalanceState: String? = nil, targetDrainStatus: String? = nil, targetNodeId: String? = nil, targetRebalanceState: String? = nil) {
        self.migratedRouteCount = migratedRouteCount
        self.sourceDrainStatus = sourceDrainStatus
        self.sourceNodeId = sourceNodeId
        self.sourceRebalanceState = sourceRebalanceState
        self.targetDrainStatus = targetDrainStatus
        self.targetNodeId = targetNodeId
        self.targetRebalanceState = targetRebalanceState
    }
}

public struct RouteNodeLifecycle: Codable {
    public let drainStatus: String?
    public let nodeId: String?
    public let ownedRouteCount: Int?
    public let rebalanceState: String?


    public init(drainStatus: String? = nil, nodeId: String? = nil, ownedRouteCount: Int? = nil, rebalanceState: String? = nil) {
        self.drainStatus = drainStatus
        self.nodeId = nodeId
        self.ownedRouteCount = ownedRouteCount
        self.rebalanceState = rebalanceState
    }
}

public struct SdkCompatibilityBaselineResponse: Codable {
    public let appSdkFamily: String?
    public let backendSdkFamily: String?
    public let imSdkFamily: String?
    public let rtcSdkFamily: String?
    public let matrixClientTypes: [String]?
    public let protocolGovernancePath: String?
    public let protocolRegistryPath: String?


    public init(appSdkFamily: String? = nil, backendSdkFamily: String? = nil, imSdkFamily: String? = nil, rtcSdkFamily: String? = nil, matrixClientTypes: [String]? = nil, protocolGovernancePath: String? = nil, protocolRegistryPath: String? = nil) {
        self.appSdkFamily = appSdkFamily
        self.backendSdkFamily = backendSdkFamily
        self.imSdkFamily = imSdkFamily
        self.rtcSdkFamily = rtcSdkFamily
        self.matrixClientTypes = matrixClientTypes
        self.protocolGovernancePath = protocolGovernancePath
        self.protocolRegistryPath = protocolRegistryPath
    }
}

public struct AcceptFriendRequestRequest: Codable {
    public let acceptedAt: String?
    public let acceptedByUserId: String?
    public let eventId: String?


    public init(acceptedAt: String? = nil, acceptedByUserId: String? = nil, eventId: String? = nil) {
        self.acceptedAt = acceptedAt
        self.acceptedByUserId = acceptedByUserId
        self.eventId = eventId
    }
}

public struct DeclineFriendRequestRequest: Codable {
    public let declinedAt: String?
    public let declinedByUserId: String?
    public let eventId: String?


    public init(declinedAt: String? = nil, declinedByUserId: String? = nil, eventId: String? = nil) {
        self.declinedAt = declinedAt
        self.declinedByUserId = declinedByUserId
        self.eventId = eventId
    }
}

public struct CancelFriendRequestRequest: Codable {
    public let canceledAt: String?
    public let canceledByUserId: String?
    public let eventId: String?


    public init(canceledAt: String? = nil, canceledByUserId: String? = nil, eventId: String? = nil) {
        self.canceledAt = canceledAt
        self.canceledByUserId = canceledByUserId
        self.eventId = eventId
    }
}

public struct RemoveFriendshipRequest: Codable {
    public let eventId: String?
    public let removedAt: String?
    public let removedByUserId: String?


    public init(eventId: String? = nil, removedAt: String? = nil, removedByUserId: String? = nil) {
        self.eventId = eventId
        self.removedAt = removedAt
        self.removedByUserId = removedByUserId
    }
}

public struct SocialDirectChatCommitResponse: Codable {

    public init() {}
}

public struct SocialDirectChatSnapshotResponse: Codable {

    public init() {}
}

public struct SocialExternalConnectionCommitResponse: Codable {

    public init() {}
}

public struct SocialExternalConnectionSnapshotResponse: Codable {

    public init() {}
}

public struct SocialExternalMemberLinkCommitResponse: Codable {

    public init() {}
}

public struct SocialExternalMemberLinkSnapshotResponse: Codable {

    public init() {}
}

public struct SocialFriendRequestCommitResponse: Codable {

    public init() {}
}

public struct SocialFriendRequestSnapshotResponse: Codable {

    public init() {}
}

public struct SocialFriendshipCommitResponse: Codable {

    public init() {}
}

public struct SocialFriendshipSnapshotResponse: Codable {

    public init() {}
}

public struct SocialRuntimeRepairResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelPolicyCommitResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelPolicySnapshotResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelSyncDeadLetterInventoryResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelSyncDeadLetterRequeueResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelSyncDeadLetterTargetedRequeueRequest: Codable {
    public let requestKeys: [String]?


    public init(requestKeys: [String]? = nil) {
        self.requestKeys = requestKeys
    }
}

public struct SocialSharedChannelSyncDeadLetterTargetedRequeueResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelSyncDeliveredInventoryResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelSyncDeliveryStateInventoryResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelSyncPendingClaimResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelSyncPendingInventoryResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelSyncPendingReleaseResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelSyncPendingStaleReclaimResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelSyncPendingTakeoverResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelSyncPendingTargetedClaimRequest: Codable {
    public let requestKeys: [String]?


    public init(requestKeys: [String]? = nil) {
        self.requestKeys = requestKeys
    }
}

public struct SocialSharedChannelSyncPendingTargetedReleaseRequest: Codable {
    public let requestKeys: [String]?


    public init(requestKeys: [String]? = nil) {
        self.requestKeys = requestKeys
    }
}

public struct SocialSharedChannelSyncPendingTargetedTakeoverRequest: Codable {
    public let allowLegacyUntracked: Bool?
    public let requestKeys: [String]?


    public init(allowLegacyUntracked: Bool? = nil, requestKeys: [String]? = nil) {
        self.allowLegacyUntracked = allowLegacyUntracked
        self.requestKeys = requestKeys
    }
}

public struct SocialSharedChannelSyncRepairResponse: Codable {

    public init() {}
}

public struct SocialSharedChannelSyncTargetedRepublishRequest: Codable {
    public let requestKeys: [String]?


    public init(requestKeys: [String]? = nil) {
        self.requestKeys = requestKeys
    }
}

public struct SocialSharedChannelSyncTargetedRepublishResponse: Codable {

    public init() {}
}

public struct SocialUserBlockCommitResponse: Codable {

    public init() {}
}

public struct SocialUserBlockSnapshotResponse: Codable {

    public init() {}
}

public struct SubmitFriendRequestRequest: Codable {
    public let eventId: String?
    public let requestId: String?
    public let requestMessage: String?
    public let requestedAt: String?
    public let requesterUserId: String?
    public let targetUserId: String?


    public init(eventId: String? = nil, requestId: String? = nil, requestMessage: String? = nil, requestedAt: String? = nil, requesterUserId: String? = nil, targetUserId: String? = nil) {
        self.eventId = eventId
        self.requestId = requestId
        self.requestMessage = requestMessage
        self.requestedAt = requestedAt
        self.requesterUserId = requesterUserId
        self.targetUserId = targetUserId
    }
}

public struct UpsertProviderBindingPolicyRequest: Codable {
    public let domain: String?
    public let expectedBaseVersion: Int?
    public let pluginId: String?
    public let tenantId: String?


    public init(domain: String? = nil, expectedBaseVersion: Int? = nil, pluginId: String? = nil, tenantId: String? = nil) {
        self.domain = domain
        self.expectedBaseVersion = expectedBaseVersion
        self.pluginId = pluginId
        self.tenantId = tenantId
    }
}
