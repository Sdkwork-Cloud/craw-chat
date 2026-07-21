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
  final String? detail;
  final String? instance;
  final int code;
  final String traceId;
  final String? i18nKey;
  final String? locale;
  final List<FieldError>? errors;

  ProblemDetail({
    required this.type,
    required this.title,
    required this.status,
    this.detail,
    this.instance,
    required this.code,
    required this.traceId,
    this.i18nKey,
    this.locale,
    this.errors
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
      detail: json['detail']?.toString(),
      instance: json['instance']?.toString(),
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ProblemDetail.code is required');
        }
        return value;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ProblemDetail.traceId is required');
        }
        return value;
      })(),
      i18nKey: json['i18nKey']?.toString(),
      locale: json['locale']?.toString(),
      errors: (() {
        final list = _sdkworkAsList(json['errors']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : FieldError.fromJson(map);
      })())
            .whereType<FieldError>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'type': type,
      'title': title,
      'status': status,
      'detail': detail,
      'instance': instance,
      'code': code,
      'traceId': traceId,
      'i18nKey': i18nKey,
      'locale': locale,
      'errors': errors?.map((item) => item.toJson()).toList(),
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
  final String policyVersion;

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
        final value = json['policyVersion']?.toString();
        if (value == null) {
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

class ProviderPolicyRollbackRequest {
  final String targetVersion;

  ProviderPolicyRollbackRequest({
    required this.targetVersion
  });

  factory ProviderPolicyRollbackRequest.fromJson(Map<String, dynamic> json) {
    return ProviderPolicyRollbackRequest(
      targetVersion: (() {
        final value = json['targetVersion']?.toString();
        if (value == null) {
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
  final String maxConcurrentSessionsPerTenant;
  final String maxInflightMessages;
  final String maxPayloadBytes;
  final String maxSubscriptionsPerSession;
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
        final value = json['maxConcurrentSessionsPerTenant']?.toString();
        if (value == null) {
          throw FormatException('QuotaProfileResponse.maxConcurrentSessionsPerTenant is required');
        }
        return value;
      })(),
      maxInflightMessages: (() {
        final value = json['maxInflightMessages']?.toString();
        if (value == null) {
          throw FormatException('QuotaProfileResponse.maxInflightMessages is required');
        }
        return value;
      })(),
      maxPayloadBytes: (() {
        final value = json['maxPayloadBytes']?.toString();
        if (value == null) {
          throw FormatException('QuotaProfileResponse.maxPayloadBytes is required');
        }
        return value;
      })(),
      maxSubscriptionsPerSession: (() {
        final value = json['maxSubscriptionsPerSession']?.toString();
        if (value == null) {
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
  final String trafficPercent;

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
        final value = json['trafficPercent']?.toString();
        if (value == null) {
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
  final String migratedRouteCount;
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
        final value = json['migratedRouteCount']?.toString();
        if (value == null) {
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
  final String ownedRouteCount;
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
        final value = json['ownedRouteCount']?.toString();
        if (value == null) {
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

class SocialSharedChannelSyncPendingClaimResponse {


  SocialSharedChannelSyncPendingClaimResponse();

  factory SocialSharedChannelSyncPendingClaimResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingClaimResponse();
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
  final String? requestMessage;
  final String requestedAt;
  final String requesterUserId;
  final String targetUserId;

  SubmitFriendRequestRequest({
    required this.eventId,
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
      'requestMessage': requestMessage,
      'requestedAt': requestedAt,
      'requesterUserId': requesterUserId,
      'targetUserId': targetUserId,
    };
  }
}

class UpsertProviderBindingPolicyRequest {
  final String domain;
  final String? expectedBaseVersion;
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
      expectedBaseVersion: json['expectedBaseVersion']?.toString(),
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

class LagItem {
  final String component;
  final String scopeId;
  final String currentOffset;
  final String committedOffset;
  final String lag;

  LagItem({
    required this.component,
    required this.scopeId,
    required this.currentOffset,
    required this.committedOffset,
    required this.lag
  });

  factory LagItem.fromJson(Map<String, dynamic> json) {
    return LagItem(
      component: (() {
        final value = json['component']?.toString();
        if (value == null) {
          throw FormatException('LagItem.component is required');
        }
        return value;
      })(),
      scopeId: (() {
        final value = json['scopeId']?.toString();
        if (value == null) {
          throw FormatException('LagItem.scopeId is required');
        }
        return value;
      })(),
      currentOffset: (() {
        final value = json['currentOffset']?.toString();
        if (value == null) {
          throw FormatException('LagItem.currentOffset is required');
        }
        return value;
      })(),
      committedOffset: (() {
        final value = json['committedOffset']?.toString();
        if (value == null) {
          throw FormatException('LagItem.committedOffset is required');
        }
        return value;
      })(),
      lag: (() {
        final value = json['lag']?.toString();
        if (value == null) {
          throw FormatException('LagItem.lag is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'component': component,
      'scopeId': scopeId,
      'currentOffset': currentOffset,
      'committedOffset': committedOffset,
      'lag': lag,
    };
  }
}

class ProviderBindingItem {
  final String domain;
  final String defaultPluginId;
  final String selectedPluginId;
  final String selectionSource;
  final bool tenantOverrideAllowed;

  ProviderBindingItem({
    required this.domain,
    required this.defaultPluginId,
    required this.selectedPluginId,
    required this.selectionSource,
    required this.tenantOverrideAllowed
  });

  factory ProviderBindingItem.fromJson(Map<String, dynamic> json) {
    return ProviderBindingItem(
      domain: (() {
        final value = json['domain']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingItem.domain is required');
        }
        return value;
      })(),
      defaultPluginId: (() {
        final value = json['defaultPluginId']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingItem.defaultPluginId is required');
        }
        return value;
      })(),
      selectedPluginId: (() {
        final value = json['selectedPluginId']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingItem.selectedPluginId is required');
        }
        return value;
      })(),
      selectionSource: (() {
        final value = json['selectionSource']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingItem.selectionSource is required');
        }
        return value;
      })(),
      tenantOverrideAllowed: (() {
        final value = json['tenantOverrideAllowed'];
        if (value is! bool) {
          throw FormatException('ProviderBindingItem.tenantOverrideAllowed is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'domain': domain,
      'defaultPluginId': defaultPluginId,
      'selectedPluginId': selectedPluginId,
      'selectionSource': selectionSource,
      'tenantOverrideAllowed': tenantOverrideAllowed,
    };
  }
}

class ProviderBindingSnapshot {
  final String interfaceVersion;
  final String tenantId;
  final List<ProviderBindingItem> effectiveBindings;
  final List<String> precedence;

  ProviderBindingSnapshot({
    required this.interfaceVersion,
    required this.tenantId,
    required this.effectiveBindings,
    required this.precedence
  });

  factory ProviderBindingSnapshot.fromJson(Map<String, dynamic> json) {
    return ProviderBindingSnapshot(
      interfaceVersion: (() {
        final value = json['interfaceVersion']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingSnapshot.interfaceVersion is required');
        }
        return value;
      })(),
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingSnapshot.tenantId is required');
        }
        return value;
      })(),
      effectiveBindings: (() {
        final list = _sdkworkAsList(json['effectiveBindings']);
        if (list == null) {
          throw FormatException('ProviderBindingSnapshot.effectiveBindings is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ProviderBindingItem.fromJson(map);
      })())
            .whereType<ProviderBindingItem>()
            .toList();
      })(),
      precedence: (() {
        final list = _sdkworkAsList(json['precedence']);
        if (list == null) {
          throw FormatException('ProviderBindingSnapshot.precedence is required');
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
      'interfaceVersion': interfaceVersion,
      'tenantId': tenantId,
      'effectiveBindings': effectiveBindings.map((item) => item.toJson()).toList(),
      'precedence': precedence.map((item) => item).toList(),
    };
  }
}

class ProviderBindingDriftItem {
  final String tenantId;
  final String domain;
  final String baselineSelectedPluginId;
  final String selectedPluginId;
  final String baselineSelectionSource;
  final String selectionSource;
  final String driftKind;

  ProviderBindingDriftItem({
    required this.tenantId,
    required this.domain,
    required this.baselineSelectedPluginId,
    required this.selectedPluginId,
    required this.baselineSelectionSource,
    required this.selectionSource,
    required this.driftKind
  });

  factory ProviderBindingDriftItem.fromJson(Map<String, dynamic> json) {
    return ProviderBindingDriftItem(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingDriftItem.tenantId is required');
        }
        return value;
      })(),
      domain: (() {
        final value = json['domain']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingDriftItem.domain is required');
        }
        return value;
      })(),
      baselineSelectedPluginId: (() {
        final value = json['baselineSelectedPluginId']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingDriftItem.baselineSelectedPluginId is required');
        }
        return value;
      })(),
      selectedPluginId: (() {
        final value = json['selectedPluginId']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingDriftItem.selectedPluginId is required');
        }
        return value;
      })(),
      baselineSelectionSource: (() {
        final value = json['baselineSelectionSource']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingDriftItem.baselineSelectionSource is required');
        }
        return value;
      })(),
      selectionSource: (() {
        final value = json['selectionSource']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingDriftItem.selectionSource is required');
        }
        return value;
      })(),
      driftKind: (() {
        final value = json['driftKind']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingDriftItem.driftKind is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'domain': domain,
      'baselineSelectedPluginId': baselineSelectedPluginId,
      'selectedPluginId': selectedPluginId,
      'baselineSelectionSource': baselineSelectionSource,
      'selectionSource': selectionSource,
      'driftKind': driftKind,
    };
  }
}

class LagPageData {
  final List<LagItem> items;
  final PageInfo pageInfo;

  LagPageData({
    required this.items,
    required this.pageInfo
  });

  factory LagPageData.fromJson(Map<String, dynamic> json) {
    return LagPageData(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('LagPageData.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : LagItem.fromJson(map);
      })())
            .whereType<LagItem>()
            .toList();
      })(),
      pageInfo: (() {
        final map = _sdkworkAsMap(json['pageInfo']);
        if (map == null) {
          throw FormatException('LagPageData.pageInfo is required');
        }
        return PageInfo.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'pageInfo': pageInfo.toJson(),
    };
  }
}

class ProviderBindingSnapshotPageData {
  final List<ProviderBindingSnapshot> items;
  final PageInfo pageInfo;

  ProviderBindingSnapshotPageData({
    required this.items,
    required this.pageInfo
  });

  factory ProviderBindingSnapshotPageData.fromJson(Map<String, dynamic> json) {
    return ProviderBindingSnapshotPageData(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('ProviderBindingSnapshotPageData.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ProviderBindingSnapshot.fromJson(map);
      })())
            .whereType<ProviderBindingSnapshot>()
            .toList();
      })(),
      pageInfo: (() {
        final map = _sdkworkAsMap(json['pageInfo']);
        if (map == null) {
          throw FormatException('ProviderBindingSnapshotPageData.pageInfo is required');
        }
        return PageInfo.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'pageInfo': pageInfo.toJson(),
    };
  }
}

class ProviderBindingDriftPageData {
  final List<ProviderBindingDriftItem> items;
  final PageInfo pageInfo;

  ProviderBindingDriftPageData({
    required this.items,
    required this.pageInfo
  });

  factory ProviderBindingDriftPageData.fromJson(Map<String, dynamic> json) {
    return ProviderBindingDriftPageData(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('ProviderBindingDriftPageData.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ProviderBindingDriftItem.fromJson(map);
      })())
            .whereType<ProviderBindingDriftItem>()
            .toList();
      })(),
      pageInfo: (() {
        final map = _sdkworkAsMap(json['pageInfo']);
        if (map == null) {
          throw FormatException('ProviderBindingDriftPageData.pageInfo is required');
        }
        return PageInfo.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'pageInfo': pageInfo.toJson(),
    };
  }
}

class LagListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  LagListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory LagListResponse.fromJson(Map<String, dynamic> json) {
    return LagListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('LagListResponse.code is required');
        }
        return value;
      })(),
      data: json['data'],
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('LagListResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ProviderBindingSnapshotListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ProviderBindingSnapshotListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ProviderBindingSnapshotListResponse.fromJson(Map<String, dynamic> json) {
    return ProviderBindingSnapshotListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ProviderBindingSnapshotListResponse.code is required');
        }
        return value;
      })(),
      data: json['data'],
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingSnapshotListResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ProviderBindingDriftListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ProviderBindingDriftListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ProviderBindingDriftListResponse.fromJson(Map<String, dynamic> json) {
    return ProviderBindingDriftListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ProviderBindingDriftListResponse.code is required');
        }
        return value;
      })(),
      data: json['data'],
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ProviderBindingDriftListResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SdkWorkApiResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SdkWorkApiResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SdkWorkApiResponse.fromJson(Map<String, dynamic> json) {
    return SdkWorkApiResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SdkWorkApiResponse.code is required');
        }
        return value;
      })(),
      data: json['data'],
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SdkWorkApiResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SdkWorkPageData {
  final List<Map<String, dynamic>> items;
  final PageInfo pageInfo;

  SdkWorkPageData({
    required this.items,
    required this.pageInfo
  });

  factory SdkWorkPageData.fromJson(Map<String, dynamic> json) {
    return SdkWorkPageData(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SdkWorkPageData.items is required');
        }
        return list
            .map((item) => _sdkworkAsMap(item))
            .whereType<Map<String, dynamic>>()
            .toList();
      })(),
      pageInfo: (() {
        final map = _sdkworkAsMap(json['pageInfo']);
        if (map == null) {
          throw FormatException('SdkWorkPageData.pageInfo is required');
        }
        return PageInfo.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item).toList(),
      'pageInfo': pageInfo.toJson(),
    };
  }
}

class PageInfo {
  final String mode;
  final int? page;
  final int? pageSize;
  final String? totalItems;
  final int? totalPages;
  final String? nextCursor;
  final bool? hasMore;

  PageInfo({
    required this.mode,
    this.page,
    this.pageSize,
    this.totalItems,
    this.totalPages,
    this.nextCursor,
    this.hasMore
  });

  factory PageInfo.fromJson(Map<String, dynamic> json) {
    return PageInfo(
      mode: (() {
        final value = json['mode']?.toString();
        if (value == null) {
          throw FormatException('PageInfo.mode is required');
        }
        return value;
      })(),
      page: json['page'] is int ? json['page'] : null,
      pageSize: json['pageSize'] is int ? json['pageSize'] : null,
      totalItems: json['totalItems']?.toString(),
      totalPages: json['totalPages'] is int ? json['totalPages'] : null,
      nextCursor: json['nextCursor']?.toString(),
      hasMore: json['hasMore'] is bool ? json['hasMore'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'mode': mode,
      'page': page,
      'pageSize': pageSize,
      'totalItems': totalItems,
      'totalPages': totalPages,
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class FieldError {
  final String field;
  final String message;
  final int? code;
  final String? i18nKey;
  final Map<String, dynamic>? params;

  FieldError({
    required this.field,
    required this.message,
    this.code,
    this.i18nKey,
    this.params
  });

  factory FieldError.fromJson(Map<String, dynamic> json) {
    return FieldError(
      field: (() {
        final value = json['field']?.toString();
        if (value == null) {
          throw FormatException('FieldError.field is required');
        }
        return value;
      })(),
      message: (() {
        final value = json['message']?.toString();
        if (value == null) {
          throw FormatException('FieldError.message is required');
        }
        return value;
      })(),
      code: json['code'] is int ? json['code'] : null,
      i18nKey: json['i18nKey']?.toString(),
      params: (() {
        final map = _sdkworkAsMap(json['params']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'field': field,
      'message': message,
      'code': code,
      'i18nKey': i18nKey,
      'params': params?.map((key, item) => MapEntry(key, item)),
    };
  }
}

class SdkWorkListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SdkWorkListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SdkWorkListResponse.fromJson(Map<String, dynamic> json) {
    return SdkWorkListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SdkWorkListResponse.code is required');
        }
        return value;
      })(),
      data: json['data'],
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SdkWorkListResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SdkWorkResourceData {
  final Map<String, dynamic> item;

  SdkWorkResourceData({
    required this.item
  });

  factory SdkWorkResourceData.fromJson(Map<String, dynamic> json) {
    return SdkWorkResourceData(
      item: (() {
        final map = _sdkworkAsMap(json['item']);
        if (map == null) {
          throw FormatException('SdkWorkResourceData.item is required');
        }
        return map;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'item': item,
    };
  }
}

class SdkWorkCommandData {
  final bool accepted;
  final String? resourceId;
  final String? status;

  SdkWorkCommandData({
    required this.accepted,
    this.resourceId,
    this.status
  });

  factory SdkWorkCommandData.fromJson(Map<String, dynamic> json) {
    return SdkWorkCommandData(
      accepted: (() {
        final value = json['accepted'];
        if (value is! bool) {
          throw FormatException('SdkWorkCommandData.accepted is required');
        }
        return value;
      })(),
      resourceId: json['resourceId']?.toString(),
      status: json['status']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'accepted': accepted,
      'resourceId': resourceId,
      'status': status,
    };
  }
}

class SdkWorkResourceResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SdkWorkResourceResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SdkWorkResourceResponse.fromJson(Map<String, dynamic> json) {
    return SdkWorkResourceResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SdkWorkResourceResponse.code is required');
        }
        return value;
      })(),
      data: json['data'],
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SdkWorkResourceResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SdkWorkCommandResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SdkWorkCommandResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SdkWorkCommandResponse.fromJson(Map<String, dynamic> json) {
    return SdkWorkCommandResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SdkWorkCommandResponse.code is required');
        }
        return value;
      })(),
      data: json['data'],
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SdkWorkCommandResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class HealthRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  HealthRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory HealthRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return HealthRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('HealthRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('HealthRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('HealthRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ClusterRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ClusterRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ClusterRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ClusterRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ClusterRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ClusterRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ClusterRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ReplayStatusRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ReplayStatusRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ReplayStatusRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ReplayStatusRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ReplayStatusRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ReplayStatusRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ReplayStatusRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class CommercialReadinessRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  CommercialReadinessRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory CommercialReadinessRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return CommercialReadinessRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('CommercialReadinessRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('CommercialReadinessRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('CommercialReadinessRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class RuntimeDirRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  RuntimeDirRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory RuntimeDirRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return RuntimeDirRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('RuntimeDirRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('RuntimeDirRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('RuntimeDirRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class DiagnosticsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  DiagnosticsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory DiagnosticsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return DiagnosticsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('DiagnosticsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('DiagnosticsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('DiagnosticsRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class RecordsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  RecordsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory RecordsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return RecordsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('RecordsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('RecordsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('RecordsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ExportRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ExportRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ExportRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ExportRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ExportRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ExportRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ExportRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class GovernanceRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  GovernanceRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory GovernanceRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return GovernanceRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('GovernanceRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('GovernanceRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('GovernanceRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class NodesActivateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  NodesActivateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory NodesActivateResponse.fromJson(Map<String, dynamic> json) {
    return NodesActivateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('NodesActivateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('NodesActivateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('NodesActivateResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class NodesDrainResponse {
  final int code;
  final dynamic data;
  final String traceId;

  NodesDrainResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory NodesDrainResponse.fromJson(Map<String, dynamic> json) {
    return NodesDrainResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('NodesDrainResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('NodesDrainResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('NodesDrainResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class NodesRoutesMigrateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  NodesRoutesMigrateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory NodesRoutesMigrateResponse.fromJson(Map<String, dynamic> json) {
    return NodesRoutesMigrateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('NodesRoutesMigrateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('NodesRoutesMigrateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('NodesRoutesMigrateResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ProtocolGovernanceRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ProtocolGovernanceRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ProtocolGovernanceRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ProtocolGovernanceRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ProtocolGovernanceRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ProtocolGovernanceRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ProtocolGovernanceRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ProtocolRegistryRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ProtocolRegistryRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ProtocolRegistryRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ProtocolRegistryRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ProtocolRegistryRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ProtocolRegistryRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ProtocolRegistryRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ProviderPoliciesPreviewResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ProviderPoliciesPreviewResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ProviderPoliciesPreviewResponse.fromJson(Map<String, dynamic> json) {
    return ProviderPoliciesPreviewResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ProviderPoliciesPreviewResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ProviderPoliciesPreviewResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ProviderPoliciesPreviewResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ProviderPoliciesRollbackResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ProviderPoliciesRollbackResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ProviderPoliciesRollbackResponse.fromJson(Map<String, dynamic> json) {
    return ProviderPoliciesRollbackResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ProviderPoliciesRollbackResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ProviderPoliciesRollbackResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ProviderPoliciesRollbackResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ProviderRegistryRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ProviderRegistryRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ProviderRegistryRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ProviderRegistryRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ProviderRegistryRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ProviderRegistryRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ProviderRegistryRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ControlProviderBindingsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ControlProviderBindingsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ControlProviderBindingsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ControlProviderBindingsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ControlProviderBindingsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ControlProviderBindingsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ControlProviderBindingsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialDirectChatsBindingsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialDirectChatsBindingsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialDirectChatsBindingsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialDirectChatsBindingsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialDirectChatsBindingsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialDirectChatsBindingsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialDirectChatsBindingsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialDirectChatsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialDirectChatsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialDirectChatsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SocialDirectChatsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialDirectChatsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialDirectChatsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialDirectChatsRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialExternalConnectionsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialExternalConnectionsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialExternalConnectionsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialExternalConnectionsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialExternalConnectionsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialExternalConnectionsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialExternalConnectionsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialExternalConnectionsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialExternalConnectionsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialExternalConnectionsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SocialExternalConnectionsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialExternalConnectionsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialExternalConnectionsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialExternalConnectionsRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialExternalMemberLinksCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialExternalMemberLinksCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialExternalMemberLinksCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialExternalMemberLinksCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialExternalMemberLinksCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialExternalMemberLinksCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialExternalMemberLinksCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialExternalMemberLinksRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialExternalMemberLinksRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialExternalMemberLinksRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SocialExternalMemberLinksRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialExternalMemberLinksRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialExternalMemberLinksRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialExternalMemberLinksRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialFriendRequestsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialFriendRequestsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialFriendRequestsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialFriendRequestsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialFriendRequestsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialFriendRequestsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialFriendRequestsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialFriendRequestsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialFriendRequestsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialFriendRequestsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialFriendRequestsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialFriendRequestsRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialFriendRequestsAcceptResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialFriendRequestsAcceptResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialFriendRequestsAcceptResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestsAcceptResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialFriendRequestsAcceptResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialFriendRequestsAcceptResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialFriendRequestsAcceptResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialFriendRequestsDeclineResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialFriendRequestsDeclineResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialFriendRequestsDeclineResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestsDeclineResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialFriendRequestsDeclineResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialFriendRequestsDeclineResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialFriendRequestsDeclineResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialFriendRequestsCancelResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialFriendRequestsCancelResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialFriendRequestsCancelResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestsCancelResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialFriendRequestsCancelResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialFriendRequestsCancelResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialFriendRequestsCancelResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialFriendshipsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialFriendshipsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialFriendshipsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialFriendshipsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialFriendshipsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialFriendshipsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialFriendshipsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialFriendshipsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialFriendshipsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialFriendshipsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendshipsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialFriendshipsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialFriendshipsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialFriendshipsRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialFriendshipsRemoveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialFriendshipsRemoveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialFriendshipsRemoveResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendshipsRemoveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialFriendshipsRemoveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialFriendshipsRemoveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialFriendshipsRemoveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialRuntimeRepairDerivedSnapshotCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialRuntimeRepairDerivedSnapshotCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialRuntimeRepairDerivedSnapshotCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialRuntimeRepairDerivedSnapshotCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialRuntimeRepairDerivedSnapshotCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialRuntimeRepairDerivedSnapshotCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialRuntimeRepairDerivedSnapshotCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialRuntimeRepairSharedChannelSyncCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialRuntimeRepairSharedChannelSyncCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialRuntimeRepairSharedChannelSyncCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialRuntimeRepairSharedChannelSyncCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialRuntimeRepairSharedChannelSyncCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialRuntimeRepairSharedChannelSyncCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialRuntimeRepairSharedChannelSyncCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialSharedChannelPoliciesCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialSharedChannelPoliciesCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialSharedChannelPoliciesCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelPoliciesCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialSharedChannelPoliciesCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialSharedChannelPoliciesCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialSharedChannelPoliciesCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialSharedChannelPoliciesRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialSharedChannelPoliciesRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialSharedChannelPoliciesRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelPoliciesRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialSharedChannelPoliciesRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialSharedChannelPoliciesRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialSharedChannelPoliciesRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialUserBlocksCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialUserBlocksCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialUserBlocksCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialUserBlocksCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialUserBlocksCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialUserBlocksCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialUserBlocksCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SocialUserBlocksRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialUserBlocksRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialUserBlocksRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SocialUserBlocksRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialUserBlocksRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialUserBlocksRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialUserBlocksRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ApiKeyGroupsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ApiKeyGroupsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ApiKeyGroupsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ApiKeyGroupsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ApiKeyGroupsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ApiKeyGroupsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ApiKeyGroupsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ApiKeyGroupsUpdateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ApiKeyGroupsUpdateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ApiKeyGroupsUpdateResponse.fromJson(Map<String, dynamic> json) {
    return ApiKeyGroupsUpdateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ApiKeyGroupsUpdateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ApiKeyGroupsUpdateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ApiKeyGroupsUpdateResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ApiKeyGroupsStatusResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ApiKeyGroupsStatusResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ApiKeyGroupsStatusResponse.fromJson(Map<String, dynamic> json) {
    return ApiKeyGroupsStatusResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ApiKeyGroupsStatusResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ApiKeyGroupsStatusResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ApiKeyGroupsStatusResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ApiKeysCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ApiKeysCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ApiKeysCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ApiKeysCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ApiKeysCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ApiKeysCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ApiKeysCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ApiKeysUpdateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ApiKeysUpdateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ApiKeysUpdateResponse.fromJson(Map<String, dynamic> json) {
    return ApiKeysUpdateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ApiKeysUpdateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ApiKeysUpdateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ApiKeysUpdateResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ApiKeysStatusResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ApiKeysStatusResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ApiKeysStatusResponse.fromJson(Map<String, dynamic> json) {
    return ApiKeysStatusResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ApiKeysStatusResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ApiKeysStatusResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ApiKeysStatusResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class BillingEventsSummaryRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  BillingEventsSummaryRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory BillingEventsSummaryRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return BillingEventsSummaryRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('BillingEventsSummaryRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('BillingEventsSummaryRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('BillingEventsSummaryRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class BillingSummaryRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  BillingSummaryRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory BillingSummaryRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return BillingSummaryRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('BillingSummaryRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('BillingSummaryRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('BillingSummaryRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ChannelModelsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ChannelModelsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ChannelModelsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ChannelModelsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ChannelModelsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ChannelModelsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ChannelModelsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ChannelsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ChannelsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ChannelsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ChannelsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ChannelsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ChannelsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ChannelsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class CredentialsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  CredentialsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory CredentialsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return CredentialsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('CredentialsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('CredentialsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('CredentialsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ExtensionsRuntimeReloadsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ExtensionsRuntimeReloadsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ExtensionsRuntimeReloadsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ExtensionsRuntimeReloadsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ExtensionsRuntimeReloadsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ExtensionsRuntimeReloadsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ExtensionsRuntimeReloadsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class GatewayRateLimitPoliciesCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  GatewayRateLimitPoliciesCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory GatewayRateLimitPoliciesCreateResponse201.fromJson(Map<String, dynamic> json) {
    return GatewayRateLimitPoliciesCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('GatewayRateLimitPoliciesCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('GatewayRateLimitPoliciesCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('GatewayRateLimitPoliciesCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MarketingCampaignsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  MarketingCampaignsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory MarketingCampaignsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return MarketingCampaignsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('MarketingCampaignsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('MarketingCampaignsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('MarketingCampaignsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MarketingCampaignsStatusResponse {
  final int code;
  final dynamic data;
  final String traceId;

  MarketingCampaignsStatusResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory MarketingCampaignsStatusResponse.fromJson(Map<String, dynamic> json) {
    return MarketingCampaignsStatusResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('MarketingCampaignsStatusResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('MarketingCampaignsStatusResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('MarketingCampaignsStatusResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ModelPricesCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ModelPricesCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ModelPricesCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ModelPricesCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ModelPricesCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ModelPricesCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ModelPricesCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ModelsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ModelsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ModelsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ModelsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ModelsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ModelsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ModelsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class ProvidersCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ProvidersCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ProvidersCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ProvidersCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ProvidersCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ProvidersCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ProvidersCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class RoutingHealthSnapshotsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  RoutingHealthSnapshotsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory RoutingHealthSnapshotsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return RoutingHealthSnapshotsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('RoutingHealthSnapshotsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('RoutingHealthSnapshotsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('RoutingHealthSnapshotsRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class RoutingProfilesCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  RoutingProfilesCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory RoutingProfilesCreateResponse201.fromJson(Map<String, dynamic> json) {
    return RoutingProfilesCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('RoutingProfilesCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('RoutingProfilesCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('RoutingProfilesCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class StorageConfigRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  StorageConfigRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory StorageConfigRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return StorageConfigRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('StorageConfigRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('StorageConfigRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('StorageConfigRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class StorageConfigCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  StorageConfigCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory StorageConfigCreateResponse201.fromJson(Map<String, dynamic> json) {
    return StorageConfigCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('StorageConfigCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('StorageConfigCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('StorageConfigCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class StorageConfigTenantsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  StorageConfigTenantsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory StorageConfigTenantsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return StorageConfigTenantsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('StorageConfigTenantsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('StorageConfigTenantsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('StorageConfigTenantsRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class StorageConfigTenantsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  StorageConfigTenantsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory StorageConfigTenantsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return StorageConfigTenantsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('StorageConfigTenantsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('StorageConfigTenantsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('StorageConfigTenantsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class StorageEffectiveTenantsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  StorageEffectiveTenantsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory StorageEffectiveTenantsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return StorageEffectiveTenantsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('StorageEffectiveTenantsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('StorageEffectiveTenantsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('StorageEffectiveTenantsRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class StorageValidationCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  StorageValidationCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory StorageValidationCreateResponse201.fromJson(Map<String, dynamic> json) {
    return StorageValidationCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('StorageValidationCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('StorageValidationCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('StorageValidationCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class StorageValidationTenantsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  StorageValidationTenantsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory StorageValidationTenantsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return StorageValidationTenantsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('StorageValidationTenantsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('StorageValidationTenantsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('StorageValidationTenantsCreateResponse201.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class UsageSummaryRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  UsageSummaryRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory UsageSummaryRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return UsageSummaryRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('UsageSummaryRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('UsageSummaryRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('UsageSummaryRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}
