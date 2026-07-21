import Foundation

public struct ProblemDetail: Codable {
    public let type: String?
    public let title: String?
    public let status: Int?
    public let detail: String?
    public let instance: String?
    public let code: Int?
    public let traceId: String?
    public let i18nKey: String?
    public let locale: String?
    public let errors: [FieldError]?


    public init(type: String? = nil, title: String? = nil, status: Int? = nil, detail: String? = nil, instance: String? = nil, code: Int? = nil, traceId: String? = nil, i18nKey: String? = nil, locale: String? = nil, errors: [FieldError]? = nil) {
        self.type = type
        self.title = title
        self.status = status
        self.detail = detail
        self.instance = instance
        self.code = code
        self.traceId = traceId
        self.i18nKey = i18nKey
        self.locale = locale
        self.errors = errors
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
    public let policyVersion: String?


    public init(appliedAt: String? = nil, channelId: String? = nil, connectionId: String? = nil, conversationId: String? = nil, eventId: String? = nil, historyVisibility: String? = nil, policyId: String? = nil, policyVersion: String? = nil) {
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

public struct ProviderPolicyRollbackRequest: Codable {
    public let targetVersion: String?


    public init(targetVersion: String? = nil) {
        self.targetVersion = targetVersion
    }
}

public struct ProviderRegistrySnapshotResponse: Codable {

    public init() {}
}

public struct QuotaProfileResponse: Codable {
    public let maxConcurrentSessionsPerTenant: String?
    public let maxInflightMessages: String?
    public let maxPayloadBytes: String?
    public let maxSubscriptionsPerSession: String?
    public let profileId: String?


    public init(maxConcurrentSessionsPerTenant: String? = nil, maxInflightMessages: String? = nil, maxPayloadBytes: String? = nil, maxSubscriptionsPerSession: String? = nil, profileId: String? = nil) {
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
    public let trafficPercent: String?


    public init(cellSelector: String? = nil, operatorOverride: Bool? = nil, policyId: String? = nil, regionSelector: String? = nil, releaseChannel: String? = nil, tenantAllowlist: [String]? = nil, trafficPercent: String? = nil) {
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
    public let migratedRouteCount: String?
    public let sourceDrainStatus: String?
    public let sourceNodeId: String?
    public let sourceRebalanceState: String?
    public let targetDrainStatus: String?
    public let targetNodeId: String?
    public let targetRebalanceState: String?


    public init(migratedRouteCount: String? = nil, sourceDrainStatus: String? = nil, sourceNodeId: String? = nil, sourceRebalanceState: String? = nil, targetDrainStatus: String? = nil, targetNodeId: String? = nil, targetRebalanceState: String? = nil) {
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
    public let ownedRouteCount: String?
    public let rebalanceState: String?


    public init(drainStatus: String? = nil, nodeId: String? = nil, ownedRouteCount: String? = nil, rebalanceState: String? = nil) {
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

public struct SocialSharedChannelSyncPendingClaimResponse: Codable {

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
    public let requestMessage: String?
    public let requestedAt: String?
    public let requesterUserId: String?
    public let targetUserId: String?


    public init(eventId: String? = nil, requestMessage: String? = nil, requestedAt: String? = nil, requesterUserId: String? = nil, targetUserId: String? = nil) {
        self.eventId = eventId
        self.requestMessage = requestMessage
        self.requestedAt = requestedAt
        self.requesterUserId = requesterUserId
        self.targetUserId = targetUserId
    }
}

public struct UpsertProviderBindingPolicyRequest: Codable {
    public let domain: String?
    public let expectedBaseVersion: String?
    public let pluginId: String?
    public let tenantId: String?


    public init(domain: String? = nil, expectedBaseVersion: String? = nil, pluginId: String? = nil, tenantId: String? = nil) {
        self.domain = domain
        self.expectedBaseVersion = expectedBaseVersion
        self.pluginId = pluginId
        self.tenantId = tenantId
    }
}

public struct LagItem: Codable {
    public let component: String?
    public let scopeId: String?
    public let currentOffset: String?
    public let committedOffset: String?
    public let lag: String?


    public init(component: String? = nil, scopeId: String? = nil, currentOffset: String? = nil, committedOffset: String? = nil, lag: String? = nil) {
        self.component = component
        self.scopeId = scopeId
        self.currentOffset = currentOffset
        self.committedOffset = committedOffset
        self.lag = lag
    }
}

public struct ProviderBindingItem: Codable {
    public let domain: String?
    public let defaultPluginId: String?
    public let selectedPluginId: String?
    public let selectionSource: String?
    public let tenantOverrideAllowed: Bool?


    public init(domain: String? = nil, defaultPluginId: String? = nil, selectedPluginId: String? = nil, selectionSource: String? = nil, tenantOverrideAllowed: Bool? = nil) {
        self.domain = domain
        self.defaultPluginId = defaultPluginId
        self.selectedPluginId = selectedPluginId
        self.selectionSource = selectionSource
        self.tenantOverrideAllowed = tenantOverrideAllowed
    }
}

public struct ProviderBindingSnapshot: Codable {
    public let interfaceVersion: String?
    public let tenantId: String?
    public let effectiveBindings: [ProviderBindingItem]?
    public let precedence_: [String]?


    public init(interfaceVersion: String? = nil, tenantId: String? = nil, effectiveBindings: [ProviderBindingItem]? = nil, precedence_: [String]? = nil) {
        self.interfaceVersion = interfaceVersion
        self.tenantId = tenantId
        self.effectiveBindings = effectiveBindings
        self.precedence_ = precedence_
    }
}

public struct ProviderBindingDriftItem: Codable {
    public let tenantId: String?
    public let domain: String?
    public let baselineSelectedPluginId: String?
    public let selectedPluginId: String?
    public let baselineSelectionSource: String?
    public let selectionSource: String?
    public let driftKind: String?


    public init(tenantId: String? = nil, domain: String? = nil, baselineSelectedPluginId: String? = nil, selectedPluginId: String? = nil, baselineSelectionSource: String? = nil, selectionSource: String? = nil, driftKind: String? = nil) {
        self.tenantId = tenantId
        self.domain = domain
        self.baselineSelectedPluginId = baselineSelectedPluginId
        self.selectedPluginId = selectedPluginId
        self.baselineSelectionSource = baselineSelectionSource
        self.selectionSource = selectionSource
        self.driftKind = driftKind
    }
}

public struct LagPageData: Codable {
    public let items: [LagItem]?
    public let pageInfo: PageInfo?


    public init(items: [LagItem]? = nil, pageInfo: PageInfo? = nil) {
        self.items = items
        self.pageInfo = pageInfo
    }
}

public struct ProviderBindingSnapshotPageData: Codable {
    public let items: [ProviderBindingSnapshot]?
    public let pageInfo: PageInfo?


    public init(items: [ProviderBindingSnapshot]? = nil, pageInfo: PageInfo? = nil) {
        self.items = items
        self.pageInfo = pageInfo
    }
}

public struct ProviderBindingDriftPageData: Codable {
    public let items: [ProviderBindingDriftItem]?
    public let pageInfo: PageInfo?


    public init(items: [ProviderBindingDriftItem]? = nil, pageInfo: PageInfo? = nil) {
        self.items = items
        self.pageInfo = pageInfo
    }
}

public struct LagListResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ProviderBindingSnapshotListResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ProviderBindingDriftListResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SdkWorkApiResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SdkWorkPageData: Codable {
    public let items: [[String: Any]]?
    public let pageInfo: PageInfo?


    public init(items: [[String: Any]]? = nil, pageInfo: PageInfo? = nil) {
        self.items = items
        self.pageInfo = pageInfo
    }
}

public struct PageInfo: Codable {
    public let mode: String?
    public let page: Int?
    public let pageSize: Int?
    public let totalItems: String?
    public let totalPages: Int?
    public let nextCursor: String?
    public let hasMore: Bool?


    public init(mode: String? = nil, page: Int? = nil, pageSize: Int? = nil, totalItems: String? = nil, totalPages: Int? = nil, nextCursor: String? = nil, hasMore: Bool? = nil) {
        self.mode = mode
        self.page = page
        self.pageSize = pageSize
        self.totalItems = totalItems
        self.totalPages = totalPages
        self.nextCursor = nextCursor
        self.hasMore = hasMore
    }
}

public struct FieldError: Codable {
    public let field: String?
    public let message: String?
    public let code: Int?
    public let i18nKey: String?
    public let params: [String: String]?


    public init(field: String? = nil, message: String? = nil, code: Int? = nil, i18nKey: String? = nil, params: [String: String]? = nil) {
        self.field = field
        self.message = message
        self.code = code
        self.i18nKey = i18nKey
        self.params = params
    }
}

public struct SdkWorkListResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SdkWorkResourceData: Codable {
    public let item: [String: Any]?


    public init(item: [String: Any]? = nil) {
        self.item = item
    }
}

public struct SdkWorkCommandData: Codable {
    public let accepted: Bool?
    public let resourceId: String?
    public let status: String?


    public init(accepted: Bool? = nil, resourceId: String? = nil, status: String? = nil) {
        self.accepted = accepted
        self.resourceId = resourceId
        self.status = status
    }
}

public struct SdkWorkResourceResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SdkWorkCommandResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct HealthRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ClusterRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ReplayStatusRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct CommercialReadinessRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct RuntimeDirRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct DiagnosticsRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct RecordsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ExportRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct GovernanceRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct NodesActivateResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct NodesDrainResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct NodesRoutesMigrateResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ProtocolGovernanceRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ProtocolRegistryRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ProviderPoliciesPreviewResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ProviderPoliciesRollbackResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ProviderRegistryRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ControlProviderBindingsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialDirectChatsBindingsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialDirectChatsRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialExternalConnectionsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialExternalConnectionsRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialExternalMemberLinksCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialExternalMemberLinksRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialFriendRequestsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialFriendRequestsRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialFriendRequestsAcceptResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialFriendRequestsDeclineResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialFriendRequestsCancelResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialFriendshipsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialFriendshipsRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialFriendshipsRemoveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialRuntimeRepairDerivedSnapshotCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialRuntimeRepairSharedChannelSyncCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialSharedChannelPoliciesCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialSharedChannelPoliciesRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialUserBlocksCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct SocialUserBlocksRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ApiKeyGroupsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ApiKeyGroupsUpdateResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ApiKeyGroupsStatusResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ApiKeysCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ApiKeysUpdateResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ApiKeysStatusResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct BillingEventsSummaryRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct BillingSummaryRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ChannelModelsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ChannelsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct CredentialsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ExtensionsRuntimeReloadsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct GatewayRateLimitPoliciesCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct MarketingCampaignsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct MarketingCampaignsStatusResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ModelPricesCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ModelsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct ProvidersCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct RoutingHealthSnapshotsRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct RoutingProfilesCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct StorageConfigRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct StorageConfigCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct StorageConfigTenantsRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct StorageConfigTenantsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct StorageEffectiveTenantsRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct StorageValidationCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct StorageValidationTenantsCreateResponse201: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}

public struct UsageSummaryRetrieveResponse: Codable {
    public let code: Int?
    public let data: Any?
    public let traceId: String?


    public init(code: Int? = nil, data: Any? = nil, traceId: String? = nil) {
        self.code = code
        self.data = data
        self.traceId = traceId
    }
}
