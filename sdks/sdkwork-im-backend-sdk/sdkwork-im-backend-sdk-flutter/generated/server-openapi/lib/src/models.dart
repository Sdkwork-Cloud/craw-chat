Map<String, dynamic>? _sdkworkAsMap(dynamic value) {
  if (value is Map<String, dynamic>) {
    return value;
  }
  if (value is Map) {
    return value.map((key, item) => MapEntry(key.toString(), item));
  }
  return null;
}

List<dynamic>? _sdkworkAsList(dynamic value) {
  return value is List ? value : null;
}

class ProblemDetail {
  final String type;
  final String title;
  final int status;
  final String detail;
  final String? code;
  final String? message;
  final String? traceId;
  final bool? retryable;

  ProblemDetail({
    required this.type,
    required this.title,
    required this.status,
    required this.detail,
    this.code,
    this.message,
    this.traceId,
    this.retryable
  });

  factory ProblemDetail.fromJson(Map<String, dynamic> json) {
    return ProblemDetail(
      type: (() {
        final value = json['type']?.toString();
        if (value == null) {
          throw FormatException('ProblemDetail.type is required');
        }
        return value;
      })(),
      title: (() {
        final value = json['title']?.toString();
        if (value == null) {
          throw FormatException('ProblemDetail.title is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status'];
        if (value is! int) {
          throw FormatException('ProblemDetail.status is required');
        }
        return value;
      })(),
      detail: (() {
        final value = json['detail']?.toString();
        if (value == null) {
          throw FormatException('ProblemDetail.detail is required');
        }
        return value;
      })(),
      code: json['code']?.toString(),
      message: json['message']?.toString(),
      traceId: json['traceId']?.toString(),
      retryable: json['retryable'] is bool ? json['retryable'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'type': type,
      'title': title,
      'status': status,
      'detail': detail,
      'code': code,
      'message': message,
      'traceId': traceId,
      'retryable': retryable,
    };
  }
}

class ActivateFriendshipRequest {
  final String? directChatId;
  final String establishedAt;
  final String eventId;
  final String friendshipId;
  final String initiatorUserId;
  final String peerUserId;

  ActivateFriendshipRequest({
    this.directChatId,
    required this.establishedAt,
    required this.eventId,
    required this.friendshipId,
    required this.initiatorUserId,
    required this.peerUserId
  });

  factory ActivateFriendshipRequest.fromJson(Map<String, dynamic> json) {
    return ActivateFriendshipRequest(
      directChatId: json['directChatId']?.toString(),
      establishedAt: (() {
        final value = json['establishedAt']?.toString();
        if (value == null) {
          throw FormatException('ActivateFriendshipRequest.establishedAt is required');
        }
        return value;
      })(),
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('ActivateFriendshipRequest.eventId is required');
        }
        return value;
      })(),
      friendshipId: (() {
        final value = json['friendshipId']?.toString();
        if (value == null) {
          throw FormatException('ActivateFriendshipRequest.friendshipId is required');
        }
        return value;
      })(),
      initiatorUserId: (() {
        final value = json['initiatorUserId']?.toString();
        if (value == null) {
          throw FormatException('ActivateFriendshipRequest.initiatorUserId is required');
        }
        return value;
      })(),
      peerUserId: (() {
        final value = json['peerUserId']?.toString();
        if (value == null) {
          throw FormatException('ActivateFriendshipRequest.peerUserId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'directChatId': directChatId,
      'establishedAt': establishedAt,
      'eventId': eventId,
      'friendshipId': friendshipId,
      'initiatorUserId': initiatorUserId,
      'peerUserId': peerUserId,
    };
  }
}

class ApplySharedChannelPolicyRequest {
  final String appliedAt;
  final String channelId;
  final String connectionId;
  final String? conversationId;
  final String eventId;
  final String historyVisibility;
  final String policyId;
  final int policyVersion;

  ApplySharedChannelPolicyRequest({
    required this.appliedAt,
    required this.channelId,
    required this.connectionId,
    this.conversationId,
    required this.eventId,
    required this.historyVisibility,
    required this.policyId,
    required this.policyVersion
  });

  factory ApplySharedChannelPolicyRequest.fromJson(Map<String, dynamic> json) {
    return ApplySharedChannelPolicyRequest(
      appliedAt: (() {
        final value = json['appliedAt']?.toString();
        if (value == null) {
          throw FormatException('ApplySharedChannelPolicyRequest.appliedAt is required');
        }
        return value;
      })(),
      channelId: (() {
        final value = json['channelId']?.toString();
        if (value == null) {
          throw FormatException('ApplySharedChannelPolicyRequest.channelId is required');
        }
        return value;
      })(),
      connectionId: (() {
        final value = json['connectionId']?.toString();
        if (value == null) {
          throw FormatException('ApplySharedChannelPolicyRequest.connectionId is required');
        }
        return value;
      })(),
      conversationId: json['conversationId']?.toString(),
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('ApplySharedChannelPolicyRequest.eventId is required');
        }
        return value;
      })(),
      historyVisibility: (() {
        final value = json['historyVisibility']?.toString();
        if (value == null) {
          throw FormatException('ApplySharedChannelPolicyRequest.historyVisibility is required');
        }
        return value;
      })(),
      policyId: (() {
        final value = json['policyId']?.toString();
        if (value == null) {
          throw FormatException('ApplySharedChannelPolicyRequest.policyId is required');
        }
        return value;
      })(),
      policyVersion: (() {
        final value = json['policyVersion'];
        if (value is! int) {
          throw FormatException('ApplySharedChannelPolicyRequest.policyVersion is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'appliedAt': appliedAt,
      'channelId': channelId,
      'connectionId': connectionId,
      'conversationId': conversationId,
      'eventId': eventId,
      'historyVisibility': historyVisibility,
      'policyId': policyId,
      'policyVersion': policyVersion,
    };
  }
}

class BindDirectChatRequest {
  final String boundAt;
  final String conversationId;
  final String directChatId;
  final String eventId;
  final String leftActorId;
  final String rightActorId;

  BindDirectChatRequest({
    required this.boundAt,
    required this.conversationId,
    required this.directChatId,
    required this.eventId,
    required this.leftActorId,
    required this.rightActorId
  });

  factory BindDirectChatRequest.fromJson(Map<String, dynamic> json) {
    return BindDirectChatRequest(
      boundAt: (() {
        final value = json['boundAt']?.toString();
        if (value == null) {
          throw FormatException('BindDirectChatRequest.boundAt is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('BindDirectChatRequest.conversationId is required');
        }
        return value;
      })(),
      directChatId: (() {
        final value = json['directChatId']?.toString();
        if (value == null) {
          throw FormatException('BindDirectChatRequest.directChatId is required');
        }
        return value;
      })(),
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('BindDirectChatRequest.eventId is required');
        }
        return value;
      })(),
      leftActorId: (() {
        final value = json['leftActorId']?.toString();
        if (value == null) {
          throw FormatException('BindDirectChatRequest.leftActorId is required');
        }
        return value;
      })(),
      rightActorId: (() {
        final value = json['rightActorId']?.toString();
        if (value == null) {
          throw FormatException('BindDirectChatRequest.rightActorId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'boundAt': boundAt,
      'conversationId': conversationId,
      'directChatId': directChatId,
      'eventId': eventId,
      'leftActorId': leftActorId,
      'rightActorId': rightActorId,
    };
  }
}

class BindExternalMemberLinkRequest {
  final String connectionId;
  final String eventId;
  final String? externalDisplayName;
  final String externalMemberId;
  final String linkId;
  final String linkedAt;
  final String localActorId;
  final String localActorKind;

  BindExternalMemberLinkRequest({
    required this.connectionId,
    required this.eventId,
    this.externalDisplayName,
    required this.externalMemberId,
    required this.linkId,
    required this.linkedAt,
    required this.localActorId,
    required this.localActorKind
  });

  factory BindExternalMemberLinkRequest.fromJson(Map<String, dynamic> json) {
    return BindExternalMemberLinkRequest(
      connectionId: (() {
        final value = json['connectionId']?.toString();
        if (value == null) {
          throw FormatException('BindExternalMemberLinkRequest.connectionId is required');
        }
        return value;
      })(),
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('BindExternalMemberLinkRequest.eventId is required');
        }
        return value;
      })(),
      externalDisplayName: json['externalDisplayName']?.toString(),
      externalMemberId: (() {
        final value = json['externalMemberId']?.toString();
        if (value == null) {
          throw FormatException('BindExternalMemberLinkRequest.externalMemberId is required');
        }
        return value;
      })(),
      linkId: (() {
        final value = json['linkId']?.toString();
        if (value == null) {
          throw FormatException('BindExternalMemberLinkRequest.linkId is required');
        }
        return value;
      })(),
      linkedAt: (() {
        final value = json['linkedAt']?.toString();
        if (value == null) {
          throw FormatException('BindExternalMemberLinkRequest.linkedAt is required');
        }
        return value;
      })(),
      localActorId: (() {
        final value = json['localActorId']?.toString();
        if (value == null) {
          throw FormatException('BindExternalMemberLinkRequest.localActorId is required');
        }
        return value;
      })(),
      localActorKind: (() {
        final value = json['localActorKind']?.toString();
        if (value == null) {
          throw FormatException('BindExternalMemberLinkRequest.localActorKind is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'connectionId': connectionId,
      'eventId': eventId,
      'externalDisplayName': externalDisplayName,
      'externalMemberId': externalMemberId,
      'linkId': linkId,
      'linkedAt': linkedAt,
      'localActorId': localActorId,
      'localActorKind': localActorKind,
    };
  }
}

class BlockUserRequest {
  final String blockId;
  final String blockedUserId;
  final String blockerUserId;
  final String? directChatId;
  final String effectiveAt;
  final String eventId;
  final String? expiresAt;
  final String scope;

  BlockUserRequest({
    required this.blockId,
    required this.blockedUserId,
    required this.blockerUserId,
    this.directChatId,
    required this.effectiveAt,
    required this.eventId,
    this.expiresAt,
    required this.scope
  });

  factory BlockUserRequest.fromJson(Map<String, dynamic> json) {
    return BlockUserRequest(
      blockId: (() {
        final value = json['blockId']?.toString();
        if (value == null) {
          throw FormatException('BlockUserRequest.blockId is required');
        }
        return value;
      })(),
      blockedUserId: (() {
        final value = json['blockedUserId']?.toString();
        if (value == null) {
          throw FormatException('BlockUserRequest.blockedUserId is required');
        }
        return value;
      })(),
      blockerUserId: (() {
        final value = json['blockerUserId']?.toString();
        if (value == null) {
          throw FormatException('BlockUserRequest.blockerUserId is required');
        }
        return value;
      })(),
      directChatId: json['directChatId']?.toString(),
      effectiveAt: (() {
        final value = json['effectiveAt']?.toString();
        if (value == null) {
          throw FormatException('BlockUserRequest.effectiveAt is required');
        }
        return value;
      })(),
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('BlockUserRequest.eventId is required');
        }
        return value;
      })(),
      expiresAt: json['expiresAt']?.toString(),
      scope: (() {
        final value = json['scope']?.toString();
        if (value == null) {
          throw FormatException('BlockUserRequest.scope is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'blockId': blockId,
      'blockedUserId': blockedUserId,
      'blockerUserId': blockerUserId,
      'directChatId': directChatId,
      'effectiveAt': effectiveAt,
      'eventId': eventId,
      'expiresAt': expiresAt,
      'scope': scope,
    };
  }
}

class BusinessPolicyVocabularyResponse {
  final String capabilityFlagsField;
  final String historyVisibilityField;
  final List<String> historyVisibilityModes;
  final String policyVersionField;
  final String retentionPolicyRefField;
  final List<String> retentionPolicyScopes;

  BusinessPolicyVocabularyResponse({
    required this.capabilityFlagsField,
    required this.historyVisibilityField,
    required this.historyVisibilityModes,
    required this.policyVersionField,
    required this.retentionPolicyRefField,
    required this.retentionPolicyScopes
  });

  factory BusinessPolicyVocabularyResponse.fromJson(Map<String, dynamic> json) {
    return BusinessPolicyVocabularyResponse(
      capabilityFlagsField: (() {
        final value = json['capabilityFlagsField']?.toString();
        if (value == null) {
          throw FormatException('BusinessPolicyVocabularyResponse.capabilityFlagsField is required');
        }
        return value;
      })(),
      historyVisibilityField: (() {
        final value = json['historyVisibilityField']?.toString();
        if (value == null) {
          throw FormatException('BusinessPolicyVocabularyResponse.historyVisibilityField is required');
        }
        return value;
      })(),
      historyVisibilityModes: (() {
        final list = _sdkworkAsList(json['historyVisibilityModes']);
        if (list == null) {
          throw FormatException('BusinessPolicyVocabularyResponse.historyVisibilityModes is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      policyVersionField: (() {
        final value = json['policyVersionField']?.toString();
        if (value == null) {
          throw FormatException('BusinessPolicyVocabularyResponse.policyVersionField is required');
        }
        return value;
      })(),
      retentionPolicyRefField: (() {
        final value = json['retentionPolicyRefField']?.toString();
        if (value == null) {
          throw FormatException('BusinessPolicyVocabularyResponse.retentionPolicyRefField is required');
        }
        return value;
      })(),
      retentionPolicyScopes: (() {
        final list = _sdkworkAsList(json['retentionPolicyScopes']);
        if (list == null) {
          throw FormatException('BusinessPolicyVocabularyResponse.retentionPolicyScopes is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'capabilityFlagsField': capabilityFlagsField,
      'historyVisibilityField': historyVisibilityField,
      'historyVisibilityModes': historyVisibilityModes.map((item) => item).toList(),
      'policyVersionField': policyVersionField,
      'retentionPolicyRefField': retentionPolicyRefField,
      'retentionPolicyScopes': retentionPolicyScopes.map((item) => item).toList(),
    };
  }
}

class CapabilityProfileResponse {
  final List<String> enabledCapabilities;
  final List<String> experimentalCapabilities;
  final String profileId;
  final String releaseChannel;

  CapabilityProfileResponse({
    required this.enabledCapabilities,
    required this.experimentalCapabilities,
    required this.profileId,
    required this.releaseChannel
  });

  factory CapabilityProfileResponse.fromJson(Map<String, dynamic> json) {
    return CapabilityProfileResponse(
      enabledCapabilities: (() {
        final list = _sdkworkAsList(json['enabledCapabilities']);
        if (list == null) {
          throw FormatException('CapabilityProfileResponse.enabledCapabilities is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      experimentalCapabilities: (() {
        final list = _sdkworkAsList(json['experimentalCapabilities']);
        if (list == null) {
          throw FormatException('CapabilityProfileResponse.experimentalCapabilities is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      profileId: (() {
        final value = json['profileId']?.toString();
        if (value == null) {
          throw FormatException('CapabilityProfileResponse.profileId is required');
        }
        return value;
      })(),
      releaseChannel: (() {
        final value = json['releaseChannel']?.toString();
        if (value == null) {
          throw FormatException('CapabilityProfileResponse.releaseChannel is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'enabledCapabilities': enabledCapabilities.map((item) => item).toList(),
      'experimentalCapabilities': experimentalCapabilities.map((item) => item).toList(),
      'profileId': profileId,
      'releaseChannel': releaseChannel,
    };
  }
}

class ClientCompatibilityResponse {
  final List<String> blockedExperimentalCapabilities;
  final String clientType;
  final String minimumProtocolVersion;
  final List<String> supportedBindings;
  final List<String> supportedCapabilities;
  final List<String> supportedCodecs;

  ClientCompatibilityResponse({
    required this.blockedExperimentalCapabilities,
    required this.clientType,
    required this.minimumProtocolVersion,
    required this.supportedBindings,
    required this.supportedCapabilities,
    required this.supportedCodecs
  });

  factory ClientCompatibilityResponse.fromJson(Map<String, dynamic> json) {
    return ClientCompatibilityResponse(
      blockedExperimentalCapabilities: (() {
        final list = _sdkworkAsList(json['blockedExperimentalCapabilities']);
        if (list == null) {
          throw FormatException('ClientCompatibilityResponse.blockedExperimentalCapabilities is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      clientType: (() {
        final value = json['clientType']?.toString();
        if (value == null) {
          throw FormatException('ClientCompatibilityResponse.clientType is required');
        }
        return value;
      })(),
      minimumProtocolVersion: (() {
        final value = json['minimumProtocolVersion']?.toString();
        if (value == null) {
          throw FormatException('ClientCompatibilityResponse.minimumProtocolVersion is required');
        }
        return value;
      })(),
      supportedBindings: (() {
        final list = _sdkworkAsList(json['supportedBindings']);
        if (list == null) {
          throw FormatException('ClientCompatibilityResponse.supportedBindings is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      supportedCapabilities: (() {
        final list = _sdkworkAsList(json['supportedCapabilities']);
        if (list == null) {
          throw FormatException('ClientCompatibilityResponse.supportedCapabilities is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      supportedCodecs: (() {
        final list = _sdkworkAsList(json['supportedCodecs']);
        if (list == null) {
          throw FormatException('ClientCompatibilityResponse.supportedCodecs is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'blockedExperimentalCapabilities': blockedExperimentalCapabilities.map((item) => item).toList(),
      'clientType': clientType,
      'minimumProtocolVersion': minimumProtocolVersion,
      'supportedBindings': supportedBindings.map((item) => item).toList(),
      'supportedCapabilities': supportedCapabilities.map((item) => item).toList(),
      'supportedCodecs': supportedCodecs.map((item) => item).toList(),
    };
  }
}

class EffectiveProtocolSnapshotResponse {
  final List<String> allowedBindings;
  final List<String> allowedCodecs;
  final List<String> enabledCapabilities;
  final bool killSwitchActive;
  final List<String> precedence;
  final String protocolVersion;
  final String quotaProfileId;
  final String releaseChannel;

  EffectiveProtocolSnapshotResponse({
    required this.allowedBindings,
    required this.allowedCodecs,
    required this.enabledCapabilities,
    required this.killSwitchActive,
    required this.precedence,
    required this.protocolVersion,
    required this.quotaProfileId,
    required this.releaseChannel
  });

  factory EffectiveProtocolSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return EffectiveProtocolSnapshotResponse(
      allowedBindings: (() {
        final list = _sdkworkAsList(json['allowedBindings']);
        if (list == null) {
          throw FormatException('EffectiveProtocolSnapshotResponse.allowedBindings is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      allowedCodecs: (() {
        final list = _sdkworkAsList(json['allowedCodecs']);
        if (list == null) {
          throw FormatException('EffectiveProtocolSnapshotResponse.allowedCodecs is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      enabledCapabilities: (() {
        final list = _sdkworkAsList(json['enabledCapabilities']);
        if (list == null) {
          throw FormatException('EffectiveProtocolSnapshotResponse.enabledCapabilities is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      killSwitchActive: (() {
        final value = json['killSwitchActive'];
        if (value is! bool) {
          throw FormatException('EffectiveProtocolSnapshotResponse.killSwitchActive is required');
        }
        return value;
      })(),
      precedence: (() {
        final list = _sdkworkAsList(json['precedence']);
        if (list == null) {
          throw FormatException('EffectiveProtocolSnapshotResponse.precedence is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      protocolVersion: (() {
        final value = json['protocolVersion']?.toString();
        if (value == null) {
          throw FormatException('EffectiveProtocolSnapshotResponse.protocolVersion is required');
        }
        return value;
      })(),
      quotaProfileId: (() {
        final value = json['quotaProfileId']?.toString();
        if (value == null) {
          throw FormatException('EffectiveProtocolSnapshotResponse.quotaProfileId is required');
        }
        return value;
      })(),
      releaseChannel: (() {
        final value = json['releaseChannel']?.toString();
        if (value == null) {
          throw FormatException('EffectiveProtocolSnapshotResponse.releaseChannel is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'allowedBindings': allowedBindings.map((item) => item).toList(),
      'allowedCodecs': allowedCodecs.map((item) => item).toList(),
      'enabledCapabilities': enabledCapabilities.map((item) => item).toList(),
      'killSwitchActive': killSwitchActive,
      'precedence': precedence.map((item) => item).toList(),
      'protocolVersion': protocolVersion,
      'quotaProfileId': quotaProfileId,
      'releaseChannel': releaseChannel,
    };
  }
}

class EstablishExternalConnectionRequest {
  final String connectionId;
  final String connectionKind;
  final String establishedAt;
  final String eventId;
  final String? externalOrgName;
  final String externalTenantId;

  EstablishExternalConnectionRequest({
    required this.connectionId,
    required this.connectionKind,
    required this.establishedAt,
    required this.eventId,
    this.externalOrgName,
    required this.externalTenantId
  });

  factory EstablishExternalConnectionRequest.fromJson(Map<String, dynamic> json) {
    return EstablishExternalConnectionRequest(
      connectionId: (() {
        final value = json['connectionId']?.toString();
        if (value == null) {
          throw FormatException('EstablishExternalConnectionRequest.connectionId is required');
        }
        return value;
      })(),
      connectionKind: (() {
        final value = json['connectionKind']?.toString();
        if (value == null) {
          throw FormatException('EstablishExternalConnectionRequest.connectionKind is required');
        }
        return value;
      })(),
      establishedAt: (() {
        final value = json['establishedAt']?.toString();
        if (value == null) {
          throw FormatException('EstablishExternalConnectionRequest.establishedAt is required');
        }
        return value;
      })(),
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('EstablishExternalConnectionRequest.eventId is required');
        }
        return value;
      })(),
      externalOrgName: json['externalOrgName']?.toString(),
      externalTenantId: (() {
        final value = json['externalTenantId']?.toString();
        if (value == null) {
          throw FormatException('EstablishExternalConnectionRequest.externalTenantId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'connectionId': connectionId,
      'connectionKind': connectionKind,
      'establishedAt': establishedAt,
      'eventId': eventId,
      'externalOrgName': externalOrgName,
      'externalTenantId': externalTenantId,
    };
  }
}

class KillSwitchResponse {
  final bool active;
  final List<String> disabledBindings;
  final List<String> disabledCapabilities;
  final List<String> disabledCodecs;
  final String reason;
  final String ruleId;

  KillSwitchResponse({
    required this.active,
    required this.disabledBindings,
    required this.disabledCapabilities,
    required this.disabledCodecs,
    required this.reason,
    required this.ruleId
  });

  factory KillSwitchResponse.fromJson(Map<String, dynamic> json) {
    return KillSwitchResponse(
      active: (() {
        final value = json['active'];
        if (value is! bool) {
          throw FormatException('KillSwitchResponse.active is required');
        }
        return value;
      })(),
      disabledBindings: (() {
        final list = _sdkworkAsList(json['disabledBindings']);
        if (list == null) {
          throw FormatException('KillSwitchResponse.disabledBindings is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      disabledCapabilities: (() {
        final list = _sdkworkAsList(json['disabledCapabilities']);
        if (list == null) {
          throw FormatException('KillSwitchResponse.disabledCapabilities is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      disabledCodecs: (() {
        final list = _sdkworkAsList(json['disabledCodecs']);
        if (list == null) {
          throw FormatException('KillSwitchResponse.disabledCodecs is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      reason: (() {
        final value = json['reason']?.toString();
        if (value == null) {
          throw FormatException('KillSwitchResponse.reason is required');
        }
        return value;
      })(),
      ruleId: (() {
        final value = json['ruleId']?.toString();
        if (value == null) {
          throw FormatException('KillSwitchResponse.ruleId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'active': active,
      'disabledBindings': disabledBindings.map((item) => item).toList(),
      'disabledCapabilities': disabledCapabilities.map((item) => item).toList(),
      'disabledCodecs': disabledCodecs.map((item) => item).toList(),
      'reason': reason,
      'ruleId': ruleId,
    };
  }
}

class MigrateRoutesRequest {
  final String targetNodeId;

  MigrateRoutesRequest({
    required this.targetNodeId
  });

  factory MigrateRoutesRequest.fromJson(Map<String, dynamic> json) {
    return MigrateRoutesRequest(
      targetNodeId: (() {
        final value = json['targetNodeId']?.toString();
        if (value == null) {
          throw FormatException('MigrateRoutesRequest.targetNodeId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'targetNodeId': targetNodeId,
    };
  }
}

class ProtocolGovernanceResponse {
  final BusinessPolicyVocabularyResponse businessPolicyVocabulary;
  final CapabilityProfileResponse capabilityProfile;
  final EffectiveProtocolSnapshotResponse effectiveSnapshot;
  final KillSwitchResponse killSwitch;
  final QuotaProfileResponse quotaProfile;
  final RolloutPolicyResponse rolloutPolicy;
  final SdkCompatibilityBaselineResponse sdkCompatibilityBaseline;

  ProtocolGovernanceResponse({
    required this.businessPolicyVocabulary,
    required this.capabilityProfile,
    required this.effectiveSnapshot,
    required this.killSwitch,
    required this.quotaProfile,
    required this.rolloutPolicy,
    required this.sdkCompatibilityBaseline
  });

  factory ProtocolGovernanceResponse.fromJson(Map<String, dynamic> json) {
    return ProtocolGovernanceResponse(
      businessPolicyVocabulary: (() {
        final map = _sdkworkAsMap(json['businessPolicyVocabulary']);
        if (map == null) {
          throw FormatException('ProtocolGovernanceResponse.businessPolicyVocabulary is required');
        }
        return BusinessPolicyVocabularyResponse.fromJson(map);
      })(),
      capabilityProfile: (() {
        final map = _sdkworkAsMap(json['capabilityProfile']);
        if (map == null) {
          throw FormatException('ProtocolGovernanceResponse.capabilityProfile is required');
        }
        return CapabilityProfileResponse.fromJson(map);
      })(),
      effectiveSnapshot: (() {
        final map = _sdkworkAsMap(json['effectiveSnapshot']);
        if (map == null) {
          throw FormatException('ProtocolGovernanceResponse.effectiveSnapshot is required');
        }
        return EffectiveProtocolSnapshotResponse.fromJson(map);
      })(),
      killSwitch: (() {
        final map = _sdkworkAsMap(json['killSwitch']);
        if (map == null) {
          throw FormatException('ProtocolGovernanceResponse.killSwitch is required');
        }
        return KillSwitchResponse.fromJson(map);
      })(),
      quotaProfile: (() {
        final map = _sdkworkAsMap(json['quotaProfile']);
        if (map == null) {
          throw FormatException('ProtocolGovernanceResponse.quotaProfile is required');
        }
        return QuotaProfileResponse.fromJson(map);
      })(),
      rolloutPolicy: (() {
        final map = _sdkworkAsMap(json['rolloutPolicy']);
        if (map == null) {
          throw FormatException('ProtocolGovernanceResponse.rolloutPolicy is required');
        }
        return RolloutPolicyResponse.fromJson(map);
      })(),
      sdkCompatibilityBaseline: (() {
        final map = _sdkworkAsMap(json['sdkCompatibilityBaseline']);
        if (map == null) {
          throw FormatException('ProtocolGovernanceResponse.sdkCompatibilityBaseline is required');
        }
        return SdkCompatibilityBaselineResponse.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'businessPolicyVocabulary': businessPolicyVocabulary.toJson(),
      'capabilityProfile': capabilityProfile.toJson(),
      'effectiveSnapshot': effectiveSnapshot.toJson(),
      'killSwitch': killSwitch.toJson(),
      'quotaProfile': quotaProfile.toJson(),
      'rolloutPolicy': rolloutPolicy.toJson(),
      'sdkCompatibilityBaseline': sdkCompatibilityBaseline.toJson(),
    };
  }
}

class ProtocolRegistryResponse {
  final List<String> bindings;
  final List<String> codecs;
  final List<ClientCompatibilityResponse> compatibilityMatrix;
  final String protocolVersion;
  final List<ProtocolSchemaResponse> schemas;

  ProtocolRegistryResponse({
    required this.bindings,
    required this.codecs,
    required this.compatibilityMatrix,
    required this.protocolVersion,
    required this.schemas
  });

  factory ProtocolRegistryResponse.fromJson(Map<String, dynamic> json) {
    return ProtocolRegistryResponse(
      bindings: (() {
        final list = _sdkworkAsList(json['bindings']);
        if (list == null) {
          throw FormatException('ProtocolRegistryResponse.bindings is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      codecs: (() {
        final list = _sdkworkAsList(json['codecs']);
        if (list == null) {
          throw FormatException('ProtocolRegistryResponse.codecs is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      compatibilityMatrix: (() {
        final list = _sdkworkAsList(json['compatibilityMatrix']);
        if (list == null) {
          throw FormatException('ProtocolRegistryResponse.compatibilityMatrix is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ClientCompatibilityResponse.fromJson(map);
      })())
            .whereType<ClientCompatibilityResponse>()
            .toList();
      })(),
      protocolVersion: (() {
        final value = json['protocolVersion']?.toString();
        if (value == null) {
          throw FormatException('ProtocolRegistryResponse.protocolVersion is required');
        }
        return value;
      })(),
      schemas: (() {
        final list = _sdkworkAsList(json['schemas']);
        if (list == null) {
          throw FormatException('ProtocolRegistryResponse.schemas is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ProtocolSchemaResponse.fromJson(map);
      })())
            .whereType<ProtocolSchemaResponse>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'bindings': bindings.map((item) => item).toList(),
      'codecs': codecs.map((item) => item).toList(),
      'compatibilityMatrix': compatibilityMatrix.map((item) => item.toJson()).toList(),
      'protocolVersion': protocolVersion,
      'schemas': schemas.map((item) => item.toJson()).toList(),
    };
  }
}

class ProtocolSchemaResponse {
  final List<String> bindingProtocols;
  final String kind;
  final List<String> requiredCapabilities;
  final String schema;
  final String stage;
  final List<String> supportedConsumers;

  ProtocolSchemaResponse({
    required this.bindingProtocols,
    required this.kind,
    required this.requiredCapabilities,
    required this.schema,
    required this.stage,
    required this.supportedConsumers
  });

  factory ProtocolSchemaResponse.fromJson(Map<String, dynamic> json) {
    return ProtocolSchemaResponse(
      bindingProtocols: (() {
        final list = _sdkworkAsList(json['bindingProtocols']);
        if (list == null) {
          throw FormatException('ProtocolSchemaResponse.bindingProtocols is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('ProtocolSchemaResponse.kind is required');
        }
        return value;
      })(),
      requiredCapabilities: (() {
        final list = _sdkworkAsList(json['requiredCapabilities']);
        if (list == null) {
          throw FormatException('ProtocolSchemaResponse.requiredCapabilities is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      schema: (() {
        final value = json['schema']?.toString();
        if (value == null) {
          throw FormatException('ProtocolSchemaResponse.schema is required');
        }
        return value;
      })(),
      stage: (() {
        final value = json['stage']?.toString();
        if (value == null) {
          throw FormatException('ProtocolSchemaResponse.stage is required');
        }
        return value;
      })(),
      supportedConsumers: (() {
        final list = _sdkworkAsList(json['supportedConsumers']);
        if (list == null) {
          throw FormatException('ProtocolSchemaResponse.supportedConsumers is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'bindingProtocols': bindingProtocols.map((item) => item).toList(),
      'kind': kind,
      'requiredCapabilities': requiredCapabilities.map((item) => item).toList(),
      'schema': schema,
      'stage': stage,
      'supportedConsumers': supportedConsumers.map((item) => item).toList(),
    };
  }
}

class ProviderBindingCommitResponse {


  ProviderBindingCommitResponse();

  factory ProviderBindingCommitResponse.fromJson(Map<String, dynamic> json) {
    return ProviderBindingCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class ProviderBindingsResponse {


  ProviderBindingsResponse();

  factory ProviderBindingsResponse.fromJson(Map<String, dynamic> json) {
    return ProviderBindingsResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class ProviderPolicyDiffResponse {


  ProviderPolicyDiffResponse();

  factory ProviderPolicyDiffResponse.fromJson(Map<String, dynamic> json) {
    return ProviderPolicyDiffResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class ProviderPolicyHistoryResponse {


  ProviderPolicyHistoryResponse();

  factory ProviderPolicyHistoryResponse.fromJson(Map<String, dynamic> json) {
    return ProviderPolicyHistoryResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class ProviderPolicyRollbackRequest {
  final int targetVersion;

  ProviderPolicyRollbackRequest({
    required this.targetVersion
  });

  factory ProviderPolicyRollbackRequest.fromJson(Map<String, dynamic> json) {
    return ProviderPolicyRollbackRequest(
      targetVersion: (() {
        final value = json['targetVersion'];
        if (value is! int) {
          throw FormatException('ProviderPolicyRollbackRequest.targetVersion is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'targetVersion': targetVersion,
    };
  }
}

class ProviderRegistrySnapshotResponse {


  ProviderRegistrySnapshotResponse();

  factory ProviderRegistrySnapshotResponse.fromJson(Map<String, dynamic> json) {
    return ProviderRegistrySnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class QuotaProfileResponse {
  final int maxConcurrentSessionsPerTenant;
  final int maxInflightMessages;
  final int maxPayloadBytes;
  final int maxSubscriptionsPerSession;
  final String profileId;

  QuotaProfileResponse({
    required this.maxConcurrentSessionsPerTenant,
    required this.maxInflightMessages,
    required this.maxPayloadBytes,
    required this.maxSubscriptionsPerSession,
    required this.profileId
  });

  factory QuotaProfileResponse.fromJson(Map<String, dynamic> json) {
    return QuotaProfileResponse(
      maxConcurrentSessionsPerTenant: (() {
        final value = json['maxConcurrentSessionsPerTenant'];
        if (value is! int) {
          throw FormatException('QuotaProfileResponse.maxConcurrentSessionsPerTenant is required');
        }
        return value;
      })(),
      maxInflightMessages: (() {
        final value = json['maxInflightMessages'];
        if (value is! int) {
          throw FormatException('QuotaProfileResponse.maxInflightMessages is required');
        }
        return value;
      })(),
      maxPayloadBytes: (() {
        final value = json['maxPayloadBytes'];
        if (value is! int) {
          throw FormatException('QuotaProfileResponse.maxPayloadBytes is required');
        }
        return value;
      })(),
      maxSubscriptionsPerSession: (() {
        final value = json['maxSubscriptionsPerSession'];
        if (value is! int) {
          throw FormatException('QuotaProfileResponse.maxSubscriptionsPerSession is required');
        }
        return value;
      })(),
      profileId: (() {
        final value = json['profileId']?.toString();
        if (value == null) {
          throw FormatException('QuotaProfileResponse.profileId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'maxConcurrentSessionsPerTenant': maxConcurrentSessionsPerTenant,
      'maxInflightMessages': maxInflightMessages,
      'maxPayloadBytes': maxPayloadBytes,
      'maxSubscriptionsPerSession': maxSubscriptionsPerSession,
      'profileId': profileId,
    };
  }
}

class RolloutPolicyResponse {
  final String cellSelector;
  final bool operatorOverride;
  final String policyId;
  final String regionSelector;
  final String releaseChannel;
  final List<String> tenantAllowlist;
  final int trafficPercent;

  RolloutPolicyResponse({
    required this.cellSelector,
    required this.operatorOverride,
    required this.policyId,
    required this.regionSelector,
    required this.releaseChannel,
    required this.tenantAllowlist,
    required this.trafficPercent
  });

  factory RolloutPolicyResponse.fromJson(Map<String, dynamic> json) {
    return RolloutPolicyResponse(
      cellSelector: (() {
        final value = json['cellSelector']?.toString();
        if (value == null) {
          throw FormatException('RolloutPolicyResponse.cellSelector is required');
        }
        return value;
      })(),
      operatorOverride: (() {
        final value = json['operatorOverride'];
        if (value is! bool) {
          throw FormatException('RolloutPolicyResponse.operatorOverride is required');
        }
        return value;
      })(),
      policyId: (() {
        final value = json['policyId']?.toString();
        if (value == null) {
          throw FormatException('RolloutPolicyResponse.policyId is required');
        }
        return value;
      })(),
      regionSelector: (() {
        final value = json['regionSelector']?.toString();
        if (value == null) {
          throw FormatException('RolloutPolicyResponse.regionSelector is required');
        }
        return value;
      })(),
      releaseChannel: (() {
        final value = json['releaseChannel']?.toString();
        if (value == null) {
          throw FormatException('RolloutPolicyResponse.releaseChannel is required');
        }
        return value;
      })(),
      tenantAllowlist: (() {
        final list = _sdkworkAsList(json['tenantAllowlist']);
        if (list == null) {
          throw FormatException('RolloutPolicyResponse.tenantAllowlist is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      trafficPercent: (() {
        final value = json['trafficPercent'];
        if (value is! int) {
          throw FormatException('RolloutPolicyResponse.trafficPercent is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'cellSelector': cellSelector,
      'operatorOverride': operatorOverride,
      'policyId': policyId,
      'regionSelector': regionSelector,
      'releaseChannel': releaseChannel,
      'tenantAllowlist': tenantAllowlist.map((item) => item).toList(),
      'trafficPercent': trafficPercent,
    };
  }
}

class RouteMigrationResult {
  final int migratedRouteCount;
  final String sourceDrainStatus;
  final String sourceNodeId;
  final String sourceRebalanceState;
  final String targetDrainStatus;
  final String targetNodeId;
  final String targetRebalanceState;

  RouteMigrationResult({
    required this.migratedRouteCount,
    required this.sourceDrainStatus,
    required this.sourceNodeId,
    required this.sourceRebalanceState,
    required this.targetDrainStatus,
    required this.targetNodeId,
    required this.targetRebalanceState
  });

  factory RouteMigrationResult.fromJson(Map<String, dynamic> json) {
    return RouteMigrationResult(
      migratedRouteCount: (() {
        final value = json['migratedRouteCount'];
        if (value is! int) {
          throw FormatException('RouteMigrationResult.migratedRouteCount is required');
        }
        return value;
      })(),
      sourceDrainStatus: (() {
        final value = json['sourceDrainStatus']?.toString();
        if (value == null) {
          throw FormatException('RouteMigrationResult.sourceDrainStatus is required');
        }
        return value;
      })(),
      sourceNodeId: (() {
        final value = json['sourceNodeId']?.toString();
        if (value == null) {
          throw FormatException('RouteMigrationResult.sourceNodeId is required');
        }
        return value;
      })(),
      sourceRebalanceState: (() {
        final value = json['sourceRebalanceState']?.toString();
        if (value == null) {
          throw FormatException('RouteMigrationResult.sourceRebalanceState is required');
        }
        return value;
      })(),
      targetDrainStatus: (() {
        final value = json['targetDrainStatus']?.toString();
        if (value == null) {
          throw FormatException('RouteMigrationResult.targetDrainStatus is required');
        }
        return value;
      })(),
      targetNodeId: (() {
        final value = json['targetNodeId']?.toString();
        if (value == null) {
          throw FormatException('RouteMigrationResult.targetNodeId is required');
        }
        return value;
      })(),
      targetRebalanceState: (() {
        final value = json['targetRebalanceState']?.toString();
        if (value == null) {
          throw FormatException('RouteMigrationResult.targetRebalanceState is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'migratedRouteCount': migratedRouteCount,
      'sourceDrainStatus': sourceDrainStatus,
      'sourceNodeId': sourceNodeId,
      'sourceRebalanceState': sourceRebalanceState,
      'targetDrainStatus': targetDrainStatus,
      'targetNodeId': targetNodeId,
      'targetRebalanceState': targetRebalanceState,
    };
  }
}

class RouteNodeLifecycle {
  final String drainStatus;
  final String nodeId;
  final int ownedRouteCount;
  final String rebalanceState;

  RouteNodeLifecycle({
    required this.drainStatus,
    required this.nodeId,
    required this.ownedRouteCount,
    required this.rebalanceState
  });

  factory RouteNodeLifecycle.fromJson(Map<String, dynamic> json) {
    return RouteNodeLifecycle(
      drainStatus: (() {
        final value = json['drainStatus']?.toString();
        if (value == null) {
          throw FormatException('RouteNodeLifecycle.drainStatus is required');
        }
        return value;
      })(),
      nodeId: (() {
        final value = json['nodeId']?.toString();
        if (value == null) {
          throw FormatException('RouteNodeLifecycle.nodeId is required');
        }
        return value;
      })(),
      ownedRouteCount: (() {
        final value = json['ownedRouteCount'];
        if (value is! int) {
          throw FormatException('RouteNodeLifecycle.ownedRouteCount is required');
        }
        return value;
      })(),
      rebalanceState: (() {
        final value = json['rebalanceState']?.toString();
        if (value == null) {
          throw FormatException('RouteNodeLifecycle.rebalanceState is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'drainStatus': drainStatus,
      'nodeId': nodeId,
      'ownedRouteCount': ownedRouteCount,
      'rebalanceState': rebalanceState,
    };
  }
}

class SdkCompatibilityBaselineResponse {
  final String appSdkFamily;
  final String backendSdkFamily;
  final String imSdkFamily;
  final String rtcSdkFamily;
  final List<String> matrixClientTypes;
  final String protocolGovernancePath;
  final String protocolRegistryPath;

  SdkCompatibilityBaselineResponse({
    required this.appSdkFamily,
    required this.backendSdkFamily,
    required this.imSdkFamily,
    required this.rtcSdkFamily,
    required this.matrixClientTypes,
    required this.protocolGovernancePath,
    required this.protocolRegistryPath
  });

  factory SdkCompatibilityBaselineResponse.fromJson(Map<String, dynamic> json) {
    return SdkCompatibilityBaselineResponse(
      appSdkFamily: (() {
        final value = json['appSdkFamily']?.toString();
        if (value == null) {
          throw FormatException('SdkCompatibilityBaselineResponse.appSdkFamily is required');
        }
        return value;
      })(),
      backendSdkFamily: (() {
        final value = json['backendSdkFamily']?.toString();
        if (value == null) {
          throw FormatException('SdkCompatibilityBaselineResponse.backendSdkFamily is required');
        }
        return value;
      })(),
      imSdkFamily: (() {
        final value = json['imSdkFamily']?.toString();
        if (value == null) {
          throw FormatException('SdkCompatibilityBaselineResponse.imSdkFamily is required');
        }
        return value;
      })(),
      rtcSdkFamily: (() {
        final value = json['rtcSdkFamily']?.toString();
        if (value == null) {
          throw FormatException('SdkCompatibilityBaselineResponse.rtcSdkFamily is required');
        }
        return value;
      })(),
      matrixClientTypes: (() {
        final list = _sdkworkAsList(json['matrixClientTypes']);
        if (list == null) {
          throw FormatException('SdkCompatibilityBaselineResponse.matrixClientTypes is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      protocolGovernancePath: (() {
        final value = json['protocolGovernancePath']?.toString();
        if (value == null) {
          throw FormatException('SdkCompatibilityBaselineResponse.protocolGovernancePath is required');
        }
        return value;
      })(),
      protocolRegistryPath: (() {
        final value = json['protocolRegistryPath']?.toString();
        if (value == null) {
          throw FormatException('SdkCompatibilityBaselineResponse.protocolRegistryPath is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'appSdkFamily': appSdkFamily,
      'backendSdkFamily': backendSdkFamily,
      'imSdkFamily': imSdkFamily,
      'rtcSdkFamily': rtcSdkFamily,
      'matrixClientTypes': matrixClientTypes.map((item) => item).toList(),
      'protocolGovernancePath': protocolGovernancePath,
      'protocolRegistryPath': protocolRegistryPath,
    };
  }
}

class AcceptFriendRequestRequest {
  final String acceptedAt;
  final String acceptedByUserId;
  final String eventId;

  AcceptFriendRequestRequest({
    required this.acceptedAt,
    required this.acceptedByUserId,
    required this.eventId
  });

  factory AcceptFriendRequestRequest.fromJson(Map<String, dynamic> json) {
    return AcceptFriendRequestRequest(
      acceptedAt: (() {
        final value = json['acceptedAt']?.toString();
        if (value == null) {
          throw FormatException('AcceptFriendRequestRequest.acceptedAt is required');
        }
        return value;
      })(),
      acceptedByUserId: (() {
        final value = json['acceptedByUserId']?.toString();
        if (value == null) {
          throw FormatException('AcceptFriendRequestRequest.acceptedByUserId is required');
        }
        return value;
      })(),
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('AcceptFriendRequestRequest.eventId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'acceptedAt': acceptedAt,
      'acceptedByUserId': acceptedByUserId,
      'eventId': eventId,
    };
  }
}

class DeclineFriendRequestRequest {
  final String declinedAt;
  final String declinedByUserId;
  final String eventId;

  DeclineFriendRequestRequest({
    required this.declinedAt,
    required this.declinedByUserId,
    required this.eventId
  });

  factory DeclineFriendRequestRequest.fromJson(Map<String, dynamic> json) {
    return DeclineFriendRequestRequest(
      declinedAt: (() {
        final value = json['declinedAt']?.toString();
        if (value == null) {
          throw FormatException('DeclineFriendRequestRequest.declinedAt is required');
        }
        return value;
      })(),
      declinedByUserId: (() {
        final value = json['declinedByUserId']?.toString();
        if (value == null) {
          throw FormatException('DeclineFriendRequestRequest.declinedByUserId is required');
        }
        return value;
      })(),
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('DeclineFriendRequestRequest.eventId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'declinedAt': declinedAt,
      'declinedByUserId': declinedByUserId,
      'eventId': eventId,
    };
  }
}

class CancelFriendRequestRequest {
  final String canceledAt;
  final String canceledByUserId;
  final String eventId;

  CancelFriendRequestRequest({
    required this.canceledAt,
    required this.canceledByUserId,
    required this.eventId
  });

  factory CancelFriendRequestRequest.fromJson(Map<String, dynamic> json) {
    return CancelFriendRequestRequest(
      canceledAt: (() {
        final value = json['canceledAt']?.toString();
        if (value == null) {
          throw FormatException('CancelFriendRequestRequest.canceledAt is required');
        }
        return value;
      })(),
      canceledByUserId: (() {
        final value = json['canceledByUserId']?.toString();
        if (value == null) {
          throw FormatException('CancelFriendRequestRequest.canceledByUserId is required');
        }
        return value;
      })(),
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('CancelFriendRequestRequest.eventId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'canceledAt': canceledAt,
      'canceledByUserId': canceledByUserId,
      'eventId': eventId,
    };
  }
}

class RemoveFriendshipRequest {
  final String eventId;
  final String removedAt;
  final String removedByUserId;

  RemoveFriendshipRequest({
    required this.eventId,
    required this.removedAt,
    required this.removedByUserId
  });

  factory RemoveFriendshipRequest.fromJson(Map<String, dynamic> json) {
    return RemoveFriendshipRequest(
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('RemoveFriendshipRequest.eventId is required');
        }
        return value;
      })(),
      removedAt: (() {
        final value = json['removedAt']?.toString();
        if (value == null) {
          throw FormatException('RemoveFriendshipRequest.removedAt is required');
        }
        return value;
      })(),
      removedByUserId: (() {
        final value = json['removedByUserId']?.toString();
        if (value == null) {
          throw FormatException('RemoveFriendshipRequest.removedByUserId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'eventId': eventId,
      'removedAt': removedAt,
      'removedByUserId': removedByUserId,
    };
  }
}

class SocialDirectChatCommitResponse {


  SocialDirectChatCommitResponse();

  factory SocialDirectChatCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialDirectChatCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialDirectChatSnapshotResponse {


  SocialDirectChatSnapshotResponse();

  factory SocialDirectChatSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialDirectChatSnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialExternalConnectionCommitResponse {


  SocialExternalConnectionCommitResponse();

  factory SocialExternalConnectionCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialExternalConnectionCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialExternalConnectionSnapshotResponse {


  SocialExternalConnectionSnapshotResponse();

  factory SocialExternalConnectionSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialExternalConnectionSnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialExternalMemberLinkCommitResponse {


  SocialExternalMemberLinkCommitResponse();

  factory SocialExternalMemberLinkCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialExternalMemberLinkCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialExternalMemberLinkSnapshotResponse {


  SocialExternalMemberLinkSnapshotResponse();

  factory SocialExternalMemberLinkSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialExternalMemberLinkSnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialFriendRequestCommitResponse {


  SocialFriendRequestCommitResponse();

  factory SocialFriendRequestCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialFriendRequestSnapshotResponse {


  SocialFriendRequestSnapshotResponse();

  factory SocialFriendRequestSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestSnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialFriendshipCommitResponse {


  SocialFriendshipCommitResponse();

  factory SocialFriendshipCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendshipCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialFriendshipSnapshotResponse {


  SocialFriendshipSnapshotResponse();

  factory SocialFriendshipSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendshipSnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialRuntimeRepairResponse {


  SocialRuntimeRepairResponse();

  factory SocialRuntimeRepairResponse.fromJson(Map<String, dynamic> json) {
    return SocialRuntimeRepairResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelPolicyCommitResponse {


  SocialSharedChannelPolicyCommitResponse();

  factory SocialSharedChannelPolicyCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelPolicyCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelPolicySnapshotResponse {


  SocialSharedChannelPolicySnapshotResponse();

  factory SocialSharedChannelPolicySnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelPolicySnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncDeadLetterInventoryResponse {


  SocialSharedChannelSyncDeadLetterInventoryResponse();

  factory SocialSharedChannelSyncDeadLetterInventoryResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncDeadLetterInventoryResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncDeadLetterRequeueResponse {


  SocialSharedChannelSyncDeadLetterRequeueResponse();

  factory SocialSharedChannelSyncDeadLetterRequeueResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncDeadLetterRequeueResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncDeadLetterTargetedRequeueRequest {
  final List<String> requestKeys;

  SocialSharedChannelSyncDeadLetterTargetedRequeueRequest({
    required this.requestKeys
  });

  factory SocialSharedChannelSyncDeadLetterTargetedRequeueRequest.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncDeadLetterTargetedRequeueRequest(
      requestKeys: (() {
        final list = _sdkworkAsList(json['requestKeys']);
        if (list == null) {
          throw FormatException('SocialSharedChannelSyncDeadLetterTargetedRequeueRequest.requestKeys is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'requestKeys': requestKeys.map((item) => item).toList(),
    };
  }
}

class SocialSharedChannelSyncDeadLetterTargetedRequeueResponse {


  SocialSharedChannelSyncDeadLetterTargetedRequeueResponse();

  factory SocialSharedChannelSyncDeadLetterTargetedRequeueResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncDeadLetterTargetedRequeueResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncDeliveredInventoryResponse {


  SocialSharedChannelSyncDeliveredInventoryResponse();

  factory SocialSharedChannelSyncDeliveredInventoryResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncDeliveredInventoryResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncDeliveryStateInventoryResponse {


  SocialSharedChannelSyncDeliveryStateInventoryResponse();

  factory SocialSharedChannelSyncDeliveryStateInventoryResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncDeliveryStateInventoryResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncPendingClaimResponse {


  SocialSharedChannelSyncPendingClaimResponse();

  factory SocialSharedChannelSyncPendingClaimResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingClaimResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncPendingInventoryResponse {


  SocialSharedChannelSyncPendingInventoryResponse();

  factory SocialSharedChannelSyncPendingInventoryResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingInventoryResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncPendingReleaseResponse {


  SocialSharedChannelSyncPendingReleaseResponse();

  factory SocialSharedChannelSyncPendingReleaseResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingReleaseResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncPendingStaleReclaimResponse {


  SocialSharedChannelSyncPendingStaleReclaimResponse();

  factory SocialSharedChannelSyncPendingStaleReclaimResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingStaleReclaimResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncPendingTakeoverResponse {


  SocialSharedChannelSyncPendingTakeoverResponse();

  factory SocialSharedChannelSyncPendingTakeoverResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingTakeoverResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncPendingTargetedClaimRequest {
  final List<String> requestKeys;

  SocialSharedChannelSyncPendingTargetedClaimRequest({
    required this.requestKeys
  });

  factory SocialSharedChannelSyncPendingTargetedClaimRequest.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingTargetedClaimRequest(
      requestKeys: (() {
        final list = _sdkworkAsList(json['requestKeys']);
        if (list == null) {
          throw FormatException('SocialSharedChannelSyncPendingTargetedClaimRequest.requestKeys is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'requestKeys': requestKeys.map((item) => item).toList(),
    };
  }
}

class SocialSharedChannelSyncPendingTargetedReleaseRequest {
  final List<String> requestKeys;

  SocialSharedChannelSyncPendingTargetedReleaseRequest({
    required this.requestKeys
  });

  factory SocialSharedChannelSyncPendingTargetedReleaseRequest.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingTargetedReleaseRequest(
      requestKeys: (() {
        final list = _sdkworkAsList(json['requestKeys']);
        if (list == null) {
          throw FormatException('SocialSharedChannelSyncPendingTargetedReleaseRequest.requestKeys is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'requestKeys': requestKeys.map((item) => item).toList(),
    };
  }
}

class SocialSharedChannelSyncPendingTargetedTakeoverRequest {
  final bool? allowLegacyUntracked;
  final List<String> requestKeys;

  SocialSharedChannelSyncPendingTargetedTakeoverRequest({
    this.allowLegacyUntracked,
    required this.requestKeys
  });

  factory SocialSharedChannelSyncPendingTargetedTakeoverRequest.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingTargetedTakeoverRequest(
      allowLegacyUntracked: json['allowLegacyUntracked'] is bool ? json['allowLegacyUntracked'] : null,
      requestKeys: (() {
        final list = _sdkworkAsList(json['requestKeys']);
        if (list == null) {
          throw FormatException('SocialSharedChannelSyncPendingTargetedTakeoverRequest.requestKeys is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'allowLegacyUntracked': allowLegacyUntracked,
      'requestKeys': requestKeys.map((item) => item).toList(),
    };
  }
}

class SocialSharedChannelSyncRepairResponse {


  SocialSharedChannelSyncRepairResponse();

  factory SocialSharedChannelSyncRepairResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncRepairResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncTargetedRepublishRequest {
  final List<String> requestKeys;

  SocialSharedChannelSyncTargetedRepublishRequest({
    required this.requestKeys
  });

  factory SocialSharedChannelSyncTargetedRepublishRequest.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncTargetedRepublishRequest(
      requestKeys: (() {
        final list = _sdkworkAsList(json['requestKeys']);
        if (list == null) {
          throw FormatException('SocialSharedChannelSyncTargetedRepublishRequest.requestKeys is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'requestKeys': requestKeys.map((item) => item).toList(),
    };
  }
}

class SocialSharedChannelSyncTargetedRepublishResponse {


  SocialSharedChannelSyncTargetedRepublishResponse();

  factory SocialSharedChannelSyncTargetedRepublishResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncTargetedRepublishResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialUserBlockCommitResponse {


  SocialUserBlockCommitResponse();

  factory SocialUserBlockCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialUserBlockCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialUserBlockSnapshotResponse {


  SocialUserBlockSnapshotResponse();

  factory SocialUserBlockSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialUserBlockSnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SubmitFriendRequestRequest {
  final String eventId;
  final String requestId;
  final String? requestMessage;
  final String requestedAt;
  final String requesterUserId;
  final String targetUserId;

  SubmitFriendRequestRequest({
    required this.eventId,
    required this.requestId,
    this.requestMessage,
    required this.requestedAt,
    required this.requesterUserId,
    required this.targetUserId
  });

  factory SubmitFriendRequestRequest.fromJson(Map<String, dynamic> json) {
    return SubmitFriendRequestRequest(
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('SubmitFriendRequestRequest.eventId is required');
        }
        return value;
      })(),
      requestId: (() {
        final value = json['requestId']?.toString();
        if (value == null) {
          throw FormatException('SubmitFriendRequestRequest.requestId is required');
        }
        return value;
      })(),
      requestMessage: json['requestMessage']?.toString(),
      requestedAt: (() {
        final value = json['requestedAt']?.toString();
        if (value == null) {
          throw FormatException('SubmitFriendRequestRequest.requestedAt is required');
        }
        return value;
      })(),
      requesterUserId: (() {
        final value = json['requesterUserId']?.toString();
        if (value == null) {
          throw FormatException('SubmitFriendRequestRequest.requesterUserId is required');
        }
        return value;
      })(),
      targetUserId: (() {
        final value = json['targetUserId']?.toString();
        if (value == null) {
          throw FormatException('SubmitFriendRequestRequest.targetUserId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'eventId': eventId,
      'requestId': requestId,
      'requestMessage': requestMessage,
      'requestedAt': requestedAt,
      'requesterUserId': requesterUserId,
      'targetUserId': targetUserId,
    };
  }
}

class UpsertProviderBindingPolicyRequest {
  final String domain;
  final int? expectedBaseVersion;
  final String pluginId;
  final String? tenantId;

  UpsertProviderBindingPolicyRequest({
    required this.domain,
    this.expectedBaseVersion,
    required this.pluginId,
    this.tenantId
  });

  factory UpsertProviderBindingPolicyRequest.fromJson(Map<String, dynamic> json) {
    return UpsertProviderBindingPolicyRequest(
      domain: (() {
        final value = json['domain']?.toString();
        if (value == null) {
          throw FormatException('UpsertProviderBindingPolicyRequest.domain is required');
        }
        return value;
      })(),
      expectedBaseVersion: json['expectedBaseVersion'] is int ? json['expectedBaseVersion'] : null,
      pluginId: (() {
        final value = json['pluginId']?.toString();
        if (value == null) {
          throw FormatException('UpsertProviderBindingPolicyRequest.pluginId is required');
        }
        return value;
      })(),
      tenantId: json['tenantId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'domain': domain,
      'expectedBaseVersion': expectedBaseVersion,
      'pluginId': pluginId,
      'tenantId': tenantId,
    };
  }
}
