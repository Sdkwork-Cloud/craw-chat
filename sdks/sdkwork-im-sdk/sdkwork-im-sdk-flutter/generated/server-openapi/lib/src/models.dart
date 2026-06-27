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

class AckResponse {
  final bool ok;

  AckResponse({
    required this.ok
  });

  factory AckResponse.fromJson(Map<String, dynamic> json) {
    return AckResponse(
      ok: (() {
        final value = json['ok'];
        if (value is! bool) {
          throw FormatException('AckResponse.ok is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'ok': ok,
    };
  }
}

class PresenceHeartbeatRequest {
  final String? deviceId;

  PresenceHeartbeatRequest({
    this.deviceId
  });

  factory PresenceHeartbeatRequest.fromJson(Map<String, dynamic> json) {
    return PresenceHeartbeatRequest(
      deviceId: json['deviceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'deviceId': deviceId,
    };
  }
}

class PresenceView {
  final String tenantId;
  final String principalId;
  final String principalKind;
  final String deviceId;
  final String status;
  final String updatedAt;

  PresenceView({
    required this.tenantId,
    required this.principalId,
    required this.principalKind,
    required this.deviceId,
    required this.status,
    required this.updatedAt
  });

  factory PresenceView.fromJson(Map<String, dynamic> json) {
    return PresenceView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('PresenceView.tenantId is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('PresenceView.principalId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('PresenceView.principalKind is required');
        }
        return value;
      })(),
      deviceId: (() {
        final value = json['deviceId']?.toString();
        if (value == null) {
          throw FormatException('PresenceView.deviceId is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status']?.toString();
        if (value == null) {
          throw FormatException('PresenceView.status is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('PresenceView.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'principalKind': principalKind,
      'deviceId': deviceId,
      'status': status,
      'updatedAt': updatedAt,
    };
  }
}

class RealtimeSubscriptionSyncRequest {
  final String? deviceId;
  final List<String>? conversations;
  final List<RealtimeSubscriptionItemInput>? items;

  RealtimeSubscriptionSyncRequest({
    this.deviceId,
    this.conversations,
    this.items
  });

  factory RealtimeSubscriptionSyncRequest.fromJson(Map<String, dynamic> json) {
    return RealtimeSubscriptionSyncRequest(
      deviceId: json['deviceId']?.toString(),
      conversations: (() {
        final list = _sdkworkAsList(json['conversations']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : RealtimeSubscriptionItemInput.fromJson(map);
      })())
            .whereType<RealtimeSubscriptionItemInput>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'deviceId': deviceId,
      'conversations': conversations?.map((item) => item).toList(),
      'items': items?.map((item) => item.toJson()).toList(),
    };
  }
}

class RealtimeSubscriptionItemInput {
  final String scopeType;
  final String scopeId;
  final List<String>? eventTypes;

  RealtimeSubscriptionItemInput({
    required this.scopeType,
    required this.scopeId,
    this.eventTypes
  });

  factory RealtimeSubscriptionItemInput.fromJson(Map<String, dynamic> json) {
    return RealtimeSubscriptionItemInput(
      scopeType: (() {
        final value = json['scopeType']?.toString();
        if (value == null) {
          throw FormatException('RealtimeSubscriptionItemInput.scopeType is required');
        }
        return value;
      })(),
      scopeId: (() {
        final value = json['scopeId']?.toString();
        if (value == null) {
          throw FormatException('RealtimeSubscriptionItemInput.scopeId is required');
        }
        return value;
      })(),
      eventTypes: (() {
        final list = _sdkworkAsList(json['eventTypes']);
        if (list == null) {
          return null;
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
      'scopeType': scopeType,
      'scopeId': scopeId,
      'eventTypes': eventTypes?.map((item) => item).toList(),
    };
  }
}

class RealtimeSubscriptionSyncResponse {
  final List<String> subscriptions;

  RealtimeSubscriptionSyncResponse({
    required this.subscriptions
  });

  factory RealtimeSubscriptionSyncResponse.fromJson(Map<String, dynamic> json) {
    return RealtimeSubscriptionSyncResponse(
      subscriptions: (() {
        final list = _sdkworkAsList(json['subscriptions']);
        if (list == null) {
          throw FormatException('RealtimeSubscriptionSyncResponse.subscriptions is required');
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
      'subscriptions': subscriptions.map((item) => item).toList(),
    };
  }
}

class RealtimeEventAckRequest {
  final List<String> eventIds;

  RealtimeEventAckRequest({
    required this.eventIds
  });

  factory RealtimeEventAckRequest.fromJson(Map<String, dynamic> json) {
    return RealtimeEventAckRequest(
      eventIds: (() {
        final list = _sdkworkAsList(json['eventIds']);
        if (list == null) {
          throw FormatException('RealtimeEventAckRequest.eventIds is required');
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
      'eventIds': eventIds.map((item) => item).toList(),
    };
  }
}

class RealtimeEventView {
  final String eventId;
  final String scope;
  final String scopeId;
  final String eventType;
  final String? payload;
  final String occurredAt;

  RealtimeEventView({
    required this.eventId,
    required this.scope,
    required this.scopeId,
    required this.eventType,
    this.payload,
    required this.occurredAt
  });

  factory RealtimeEventView.fromJson(Map<String, dynamic> json) {
    return RealtimeEventView(
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('RealtimeEventView.eventId is required');
        }
        return value;
      })(),
      scope: (() {
        final value = json['scope']?.toString();
        if (value == null) {
          throw FormatException('RealtimeEventView.scope is required');
        }
        return value;
      })(),
      scopeId: (() {
        final value = json['scopeId']?.toString();
        if (value == null) {
          throw FormatException('RealtimeEventView.scopeId is required');
        }
        return value;
      })(),
      eventType: (() {
        final value = json['eventType']?.toString();
        if (value == null) {
          throw FormatException('RealtimeEventView.eventType is required');
        }
        return value;
      })(),
      payload: json['payload']?.toString(),
      occurredAt: (() {
        final value = json['occurredAt']?.toString();
        if (value == null) {
          throw FormatException('RealtimeEventView.occurredAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'eventId': eventId,
      'scope': scope,
      'scopeId': scopeId,
      'eventType': eventType,
      'payload': payload,
      'occurredAt': occurredAt,
    };
  }
}

class RealtimeEventsResponse {
  final List<RealtimeEventView> items;
  final String? nextCursor;
  final bool hasMore;

  RealtimeEventsResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory RealtimeEventsResponse.fromJson(Map<String, dynamic> json) {
    return RealtimeEventsResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('RealtimeEventsResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : RealtimeEventView.fromJson(map);
      })())
            .whereType<RealtimeEventView>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('RealtimeEventsResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class RtcSession {
  final String tenantId;
  final String rtcSessionId;
  final String? conversationId;
  final String initiatorId;
  final String initiatorKind;
  final String? providerPluginId;
  final String? providerSessionId;
  final String? accessEndpoint;
  final String? providerRegion;
  final String rtcMode;
  final String state;
  final String? signalingStreamId;
  final String? artifactMessageId;
  final String startedAt;
  final String? endedAt;

  RtcSession({
    required this.tenantId,
    required this.rtcSessionId,
    this.conversationId,
    required this.initiatorId,
    required this.initiatorKind,
    this.providerPluginId,
    this.providerSessionId,
    this.accessEndpoint,
    this.providerRegion,
    required this.rtcMode,
    required this.state,
    this.signalingStreamId,
    this.artifactMessageId,
    required this.startedAt,
    this.endedAt
  });

  factory RtcSession.fromJson(Map<String, dynamic> json) {
    return RtcSession(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('RtcSession.tenantId is required');
        }
        return value;
      })(),
      rtcSessionId: (() {
        final value = json['rtcSessionId']?.toString();
        if (value == null) {
          throw FormatException('RtcSession.rtcSessionId is required');
        }
        return value;
      })(),
      conversationId: json['conversationId']?.toString(),
      initiatorId: (() {
        final value = json['initiatorId']?.toString();
        if (value == null) {
          throw FormatException('RtcSession.initiatorId is required');
        }
        return value;
      })(),
      initiatorKind: (() {
        final value = json['initiatorKind']?.toString();
        if (value == null) {
          throw FormatException('RtcSession.initiatorKind is required');
        }
        return value;
      })(),
      providerPluginId: json['providerPluginId']?.toString(),
      providerSessionId: json['providerSessionId']?.toString(),
      accessEndpoint: json['accessEndpoint']?.toString(),
      providerRegion: json['providerRegion']?.toString(),
      rtcMode: (() {
        final value = json['rtcMode']?.toString();
        if (value == null) {
          throw FormatException('RtcSession.rtcMode is required');
        }
        return value;
      })(),
      state: (() {
        final value = json['state']?.toString();
        if (value == null) {
          throw FormatException('RtcSession.state is required');
        }
        return value;
      })(),
      signalingStreamId: json['signalingStreamId']?.toString(),
      artifactMessageId: json['artifactMessageId']?.toString(),
      startedAt: (() {
        final value = json['startedAt']?.toString();
        if (value == null) {
          throw FormatException('RtcSession.startedAt is required');
        }
        return value;
      })(),
      endedAt: json['endedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'rtcSessionId': rtcSessionId,
      'conversationId': conversationId,
      'initiatorId': initiatorId,
      'initiatorKind': initiatorKind,
      'providerPluginId': providerPluginId,
      'providerSessionId': providerSessionId,
      'accessEndpoint': accessEndpoint,
      'providerRegion': providerRegion,
      'rtcMode': rtcMode,
      'state': state,
      'signalingStreamId': signalingStreamId,
      'artifactMessageId': artifactMessageId,
      'startedAt': startedAt,
      'endedAt': endedAt,
    };
  }
}

class CreateRtcSessionRequest {
  final String rtcSessionId;
  final String? conversationId;
  final String rtcMode;

  CreateRtcSessionRequest({
    required this.rtcSessionId,
    this.conversationId,
    required this.rtcMode
  });

  factory CreateRtcSessionRequest.fromJson(Map<String, dynamic> json) {
    return CreateRtcSessionRequest(
      rtcSessionId: (() {
        final value = json['rtcSessionId']?.toString();
        if (value == null) {
          throw FormatException('CreateRtcSessionRequest.rtcSessionId is required');
        }
        return value;
      })(),
      conversationId: json['conversationId']?.toString(),
      rtcMode: (() {
        final value = json['rtcMode']?.toString();
        if (value == null) {
          throw FormatException('CreateRtcSessionRequest.rtcMode is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'rtcSessionId': rtcSessionId,
      'conversationId': conversationId,
      'rtcMode': rtcMode,
    };
  }
}

class InviteRtcSessionRequest {
  final String? signalingStreamId;

  InviteRtcSessionRequest({
    this.signalingStreamId
  });

  factory InviteRtcSessionRequest.fromJson(Map<String, dynamic> json) {
    return InviteRtcSessionRequest(
      signalingStreamId: json['signalingStreamId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'signalingStreamId': signalingStreamId,
    };
  }
}

class UpdateRtcSessionRequest {
  final String? artifactMessageId;

  UpdateRtcSessionRequest({
    this.artifactMessageId
  });

  factory UpdateRtcSessionRequest.fromJson(Map<String, dynamic> json) {
    return UpdateRtcSessionRequest(
      artifactMessageId: json['artifactMessageId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'artifactMessageId': artifactMessageId,
    };
  }
}

class PostRtcSignalRequest {
  final String signalType;
  final String? schemaRef;
  final String payload;
  final String? signalingStreamId;

  PostRtcSignalRequest({
    required this.signalType,
    this.schemaRef,
    required this.payload,
    this.signalingStreamId
  });

  factory PostRtcSignalRequest.fromJson(Map<String, dynamic> json) {
    return PostRtcSignalRequest(
      signalType: (() {
        final value = json['signalType']?.toString();
        if (value == null) {
          throw FormatException('PostRtcSignalRequest.signalType is required');
        }
        return value;
      })(),
      schemaRef: json['schemaRef']?.toString(),
      payload: (() {
        final value = json['payload']?.toString();
        if (value == null) {
          throw FormatException('PostRtcSignalRequest.payload is required');
        }
        return value;
      })(),
      signalingStreamId: json['signalingStreamId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'signalType': signalType,
      'schemaRef': schemaRef,
      'payload': payload,
      'signalingStreamId': signalingStreamId,
    };
  }
}

class IssueRtcParticipantCredentialRequest {
  final String participantId;

  IssueRtcParticipantCredentialRequest({
    required this.participantId
  });

  factory IssueRtcParticipantCredentialRequest.fromJson(Map<String, dynamic> json) {
    return IssueRtcParticipantCredentialRequest(
      participantId: (() {
        final value = json['participantId']?.toString();
        if (value == null) {
          throw FormatException('IssueRtcParticipantCredentialRequest.participantId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'participantId': participantId,
    };
  }
}

class RtcSessionMutationResponse {
  final String tenantId;
  final String rtcSessionId;
  final String? conversationId;
  final String initiatorId;
  final String initiatorKind;
  final String? providerPluginId;
  final String? providerSessionId;
  final String? accessEndpoint;
  final String? providerRegion;
  final String rtcMode;
  final String state;
  final String? signalingStreamId;
  final String? artifactMessageId;
  final String startedAt;
  final String? endedAt;
  final String requestKey;
  final String deliveryStatus;
  final String proofVersion;

  RtcSessionMutationResponse({
    required this.tenantId,
    required this.rtcSessionId,
    this.conversationId,
    required this.initiatorId,
    required this.initiatorKind,
    this.providerPluginId,
    this.providerSessionId,
    this.accessEndpoint,
    this.providerRegion,
    required this.rtcMode,
    required this.state,
    this.signalingStreamId,
    this.artifactMessageId,
    required this.startedAt,
    this.endedAt,
    required this.requestKey,
    required this.deliveryStatus,
    required this.proofVersion
  });

  factory RtcSessionMutationResponse.fromJson(Map<String, dynamic> json) {
    return RtcSessionMutationResponse(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('RtcSessionMutationResponse.tenantId is required');
        }
        return value;
      })(),
      rtcSessionId: (() {
        final value = json['rtcSessionId']?.toString();
        if (value == null) {
          throw FormatException('RtcSessionMutationResponse.rtcSessionId is required');
        }
        return value;
      })(),
      conversationId: json['conversationId']?.toString(),
      initiatorId: (() {
        final value = json['initiatorId']?.toString();
        if (value == null) {
          throw FormatException('RtcSessionMutationResponse.initiatorId is required');
        }
        return value;
      })(),
      initiatorKind: (() {
        final value = json['initiatorKind']?.toString();
        if (value == null) {
          throw FormatException('RtcSessionMutationResponse.initiatorKind is required');
        }
        return value;
      })(),
      providerPluginId: json['providerPluginId']?.toString(),
      providerSessionId: json['providerSessionId']?.toString(),
      accessEndpoint: json['accessEndpoint']?.toString(),
      providerRegion: json['providerRegion']?.toString(),
      rtcMode: (() {
        final value = json['rtcMode']?.toString();
        if (value == null) {
          throw FormatException('RtcSessionMutationResponse.rtcMode is required');
        }
        return value;
      })(),
      state: (() {
        final value = json['state']?.toString();
        if (value == null) {
          throw FormatException('RtcSessionMutationResponse.state is required');
        }
        return value;
      })(),
      signalingStreamId: json['signalingStreamId']?.toString(),
      artifactMessageId: json['artifactMessageId']?.toString(),
      startedAt: (() {
        final value = json['startedAt']?.toString();
        if (value == null) {
          throw FormatException('RtcSessionMutationResponse.startedAt is required');
        }
        return value;
      })(),
      endedAt: json['endedAt']?.toString(),
      requestKey: (() {
        final value = json['requestKey']?.toString();
        if (value == null) {
          throw FormatException('RtcSessionMutationResponse.requestKey is required');
        }
        return value;
      })(),
      deliveryStatus: (() {
        final value = json['deliveryStatus']?.toString();
        if (value == null) {
          throw FormatException('RtcSessionMutationResponse.deliveryStatus is required');
        }
        return value;
      })(),
      proofVersion: (() {
        final value = json['proofVersion']?.toString();
        if (value == null) {
          throw FormatException('RtcSessionMutationResponse.proofVersion is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'rtcSessionId': rtcSessionId,
      'conversationId': conversationId,
      'initiatorId': initiatorId,
      'initiatorKind': initiatorKind,
      'providerPluginId': providerPluginId,
      'providerSessionId': providerSessionId,
      'accessEndpoint': accessEndpoint,
      'providerRegion': providerRegion,
      'rtcMode': rtcMode,
      'state': state,
      'signalingStreamId': signalingStreamId,
      'artifactMessageId': artifactMessageId,
      'startedAt': startedAt,
      'endedAt': endedAt,
      'requestKey': requestKey,
      'deliveryStatus': deliveryStatus,
      'proofVersion': proofVersion,
    };
  }
}

class RtcSignalSender {
  final String id;
  final String kind;
  final String? memberId;
  final String? deviceId;
  final String? sessionId;
  final Map<String, dynamic> metadata;

  RtcSignalSender({
    required this.id,
    required this.kind,
    this.memberId,
    this.deviceId,
    this.sessionId,
    required this.metadata
  });

  factory RtcSignalSender.fromJson(Map<String, dynamic> json) {
    return RtcSignalSender(
      id: (() {
        final value = json['id']?.toString();
        if (value == null) {
          throw FormatException('RtcSignalSender.id is required');
        }
        return value;
      })(),
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('RtcSignalSender.kind is required');
        }
        return value;
      })(),
      memberId: json['memberId']?.toString(),
      deviceId: json['deviceId']?.toString(),
      sessionId: json['sessionId']?.toString(),
      metadata: (() {
        final map = _sdkworkAsMap(json['metadata']);
        if (map == null) {
          throw FormatException('RtcSignalSender.metadata is required');
        }
        return map;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'kind': kind,
      'memberId': memberId,
      'deviceId': deviceId,
      'sessionId': sessionId,
      'metadata': metadata,
    };
  }
}

class RtcSignalEvent {
  final String tenantId;
  final String rtcSessionId;
  final int signalSeq;
  final String? conversationId;
  final String rtcMode;
  final String signalType;
  final String? schemaRef;
  final String payload;
  final RtcSignalSender sender;
  final String? signalingStreamId;
  final String occurredAt;

  RtcSignalEvent({
    required this.tenantId,
    required this.rtcSessionId,
    required this.signalSeq,
    this.conversationId,
    required this.rtcMode,
    required this.signalType,
    this.schemaRef,
    required this.payload,
    required this.sender,
    this.signalingStreamId,
    required this.occurredAt
  });

  factory RtcSignalEvent.fromJson(Map<String, dynamic> json) {
    return RtcSignalEvent(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('RtcSignalEvent.tenantId is required');
        }
        return value;
      })(),
      rtcSessionId: (() {
        final value = json['rtcSessionId']?.toString();
        if (value == null) {
          throw FormatException('RtcSignalEvent.rtcSessionId is required');
        }
        return value;
      })(),
      signalSeq: (() {
        final value = json['signalSeq'];
        if (value is! int) {
          throw FormatException('RtcSignalEvent.signalSeq is required');
        }
        return value;
      })(),
      conversationId: json['conversationId']?.toString(),
      rtcMode: (() {
        final value = json['rtcMode']?.toString();
        if (value == null) {
          throw FormatException('RtcSignalEvent.rtcMode is required');
        }
        return value;
      })(),
      signalType: (() {
        final value = json['signalType']?.toString();
        if (value == null) {
          throw FormatException('RtcSignalEvent.signalType is required');
        }
        return value;
      })(),
      schemaRef: json['schemaRef']?.toString(),
      payload: (() {
        final value = json['payload']?.toString();
        if (value == null) {
          throw FormatException('RtcSignalEvent.payload is required');
        }
        return value;
      })(),
      sender: (() {
        final map = _sdkworkAsMap(json['sender']);
        if (map == null) {
          throw FormatException('RtcSignalEvent.sender is required');
        }
        return RtcSignalSender.fromJson(map);
      })(),
      signalingStreamId: json['signalingStreamId']?.toString(),
      occurredAt: (() {
        final value = json['occurredAt']?.toString();
        if (value == null) {
          throw FormatException('RtcSignalEvent.occurredAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'rtcSessionId': rtcSessionId,
      'signalSeq': signalSeq,
      'conversationId': conversationId,
      'rtcMode': rtcMode,
      'signalType': signalType,
      'schemaRef': schemaRef,
      'payload': payload,
      'sender': sender.toJson(),
      'signalingStreamId': signalingStreamId,
      'occurredAt': occurredAt,
    };
  }
}

class RtcParticipantCredential {
  final String tenantId;
  final String rtcSessionId;
  final String participantId;
  final String credential;
  final String expiresAt;

  RtcParticipantCredential({
    required this.tenantId,
    required this.rtcSessionId,
    required this.participantId,
    required this.credential,
    required this.expiresAt
  });

  factory RtcParticipantCredential.fromJson(Map<String, dynamic> json) {
    return RtcParticipantCredential(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('RtcParticipantCredential.tenantId is required');
        }
        return value;
      })(),
      rtcSessionId: (() {
        final value = json['rtcSessionId']?.toString();
        if (value == null) {
          throw FormatException('RtcParticipantCredential.rtcSessionId is required');
        }
        return value;
      })(),
      participantId: (() {
        final value = json['participantId']?.toString();
        if (value == null) {
          throw FormatException('RtcParticipantCredential.participantId is required');
        }
        return value;
      })(),
      credential: (() {
        final value = json['credential']?.toString();
        if (value == null) {
          throw FormatException('RtcParticipantCredential.credential is required');
        }
        return value;
      })(),
      expiresAt: (() {
        final value = json['expiresAt']?.toString();
        if (value == null) {
          throw FormatException('RtcParticipantCredential.expiresAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'rtcSessionId': rtcSessionId,
      'participantId': participantId,
      'credential': credential,
      'expiresAt': expiresAt,
    };
  }
}

class Sender {
  final String id;
  final String kind;
  final String? principalId;
  final String? principalKind;
  final String? displayName;
  final String? avatarUrl;

  Sender({
    required this.id,
    required this.kind,
    this.principalId,
    this.principalKind,
    this.displayName,
    this.avatarUrl
  });

  factory Sender.fromJson(Map<String, dynamic> json) {
    return Sender(
      id: (() {
        final value = json['id']?.toString();
        if (value == null) {
          throw FormatException('Sender.id is required');
        }
        return value;
      })(),
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('Sender.kind is required');
        }
        return value;
      })(),
      principalId: json['principalId']?.toString(),
      principalKind: json['principalKind']?.toString(),
      displayName: json['displayName']?.toString(),
      avatarUrl: json['avatarUrl']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'kind': kind,
      'principalId': principalId,
      'principalKind': principalKind,
      'displayName': displayName,
      'avatarUrl': avatarUrl,
    };
  }
}

class MessageReplyReference {
  final String messageId;
  final String senderDisplayName;
  final String contentPreview;

  MessageReplyReference({
    required this.messageId,
    required this.senderDisplayName,
    required this.contentPreview
  });

  factory MessageReplyReference.fromJson(Map<String, dynamic> json) {
    return MessageReplyReference(
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('MessageReplyReference.messageId is required');
        }
        return value;
      })(),
      senderDisplayName: (() {
        final value = json['senderDisplayName']?.toString();
        if (value == null) {
          throw FormatException('MessageReplyReference.senderDisplayName is required');
        }
        return value;
      })(),
      contentPreview: (() {
        final value = json['contentPreview']?.toString();
        if (value == null) {
          throw FormatException('MessageReplyReference.contentPreview is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'messageId': messageId,
      'senderDisplayName': senderDisplayName,
      'contentPreview': contentPreview,
    };
  }
}

class DriveReference {
  final String driveUri;
  final String spaceId;
  final String nodeId;
  final String? nodeVersion;

  DriveReference({
    required this.driveUri,
    required this.spaceId,
    required this.nodeId,
    this.nodeVersion
  });

  factory DriveReference.fromJson(Map<String, dynamic> json) {
    return DriveReference(
      driveUri: (() {
        final value = json['driveUri']?.toString();
        if (value == null) {
          throw FormatException('DriveReference.driveUri is required');
        }
        return value;
      })(),
      spaceId: (() {
        final value = json['spaceId']?.toString();
        if (value == null) {
          throw FormatException('DriveReference.spaceId is required');
        }
        return value;
      })(),
      nodeId: (() {
        final value = json['nodeId']?.toString();
        if (value == null) {
          throw FormatException('DriveReference.nodeId is required');
        }
        return value;
      })(),
      nodeVersion: json['nodeVersion']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'driveUri': driveUri,
      'spaceId': spaceId,
      'nodeId': nodeId,
      'nodeVersion': nodeVersion,
    };
  }
}

class MediaResource {
  final String? id;
  final String? kind;
  final String? mediaKind;
  final String source;
  final String uri;
  final String? publicUrl;
  final String? url;
  final String? name;
  final String? title;
  final String? fileName;
  final String? mimeType;
  final int? size;
  final String? sizeBytes;
  final String? fileSize;
  final int? durationSeconds;
  final MediaResource? poster;
  final List<MediaResource>? thumbnails;

  MediaResource({
    this.id,
    this.kind,
    this.mediaKind,
    required this.source,
    required this.uri,
    this.publicUrl,
    this.url,
    this.name,
    this.title,
    this.fileName,
    this.mimeType,
    this.size,
    this.sizeBytes,
    this.fileSize,
    this.durationSeconds,
    this.poster,
    this.thumbnails
  });

  factory MediaResource.fromJson(Map<String, dynamic> json) {
    return MediaResource(
      id: json['id']?.toString(),
      kind: json['kind']?.toString(),
      mediaKind: json['mediaKind']?.toString(),
      source: (() {
        final value = json['source']?.toString();
        if (value == null) {
          throw FormatException('MediaResource.source is required');
        }
        return value;
      })(),
      uri: (() {
        final value = json['uri']?.toString();
        if (value == null) {
          throw FormatException('MediaResource.uri is required');
        }
        return value;
      })(),
      publicUrl: json['publicUrl']?.toString(),
      url: json['url']?.toString(),
      name: json['name']?.toString(),
      title: json['title']?.toString(),
      fileName: json['fileName']?.toString(),
      mimeType: json['mimeType']?.toString(),
      size: json['size'] is int ? json['size'] : null,
      sizeBytes: json['sizeBytes']?.toString(),
      fileSize: json['fileSize']?.toString(),
      durationSeconds: json['durationSeconds'] is int ? json['durationSeconds'] : null,
      poster: (() {
        final map = _sdkworkAsMap(json['poster']);
        return map == null ? null : MediaResource.fromJson(map);
      })(),
      thumbnails: (() {
        final list = _sdkworkAsList(json['thumbnails']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MediaResource.fromJson(map);
      })())
            .whereType<MediaResource>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'kind': kind,
      'mediaKind': mediaKind,
      'source': source,
      'uri': uri,
      'publicUrl': publicUrl,
      'url': url,
      'name': name,
      'title': title,
      'fileName': fileName,
      'mimeType': mimeType,
      'size': size,
      'sizeBytes': sizeBytes,
      'fileSize': fileSize,
      'durationSeconds': durationSeconds,
      'poster': poster?.toJson(),
      'thumbnails': thumbnails?.map((item) => item.toJson()).toList(),
    };
  }
}

abstract class ContentPart {
  const ContentPart();

  factory ContentPart.fromJson(Map<String, dynamic> json) {
    switch (json['kind']?.toString()) {
      case 'text':
        return TextContentPart.fromJson(json);
      case 'data':
        return DataContentPart.fromJson(json);
      case 'media':
        return MediaContentPart.fromJson(json);
      case 'signal':
        return SignalContentPart.fromJson(json);
      case 'stream_ref':
        return StreamRefContentPart.fromJson(json);
      default:
        return UnknownContentPart(json);
    }
  }

  Map<String, dynamic> toJson();
}

class UnknownContentPart implements ContentPart {
  final Map<String, dynamic> raw;

  const UnknownContentPart(this.raw);

  @override
  Map<String, dynamic> toJson() {
    return raw;
  }
}

class MessageBody {
  final String? text;
  final List<ContentPart> parts;
  final MessageReplyReference? replyTo;
  final Map<String, dynamic>? renderHints;
  final String? summary;
  final Map<String, dynamic>? metadata;

  MessageBody({
    this.text,
    required this.parts,
    this.replyTo,
    this.renderHints,
    this.summary,
    this.metadata
  });

  factory MessageBody.fromJson(Map<String, dynamic> json) {
    return MessageBody(
      text: json['text']?.toString(),
      parts: (() {
        final list = _sdkworkAsList(json['parts']);
        if (list == null) {
          throw FormatException('MessageBody.parts is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ContentPart.fromJson(map);
      })())
            .whereType<ContentPart>()
            .toList();
      })(),
      replyTo: (() {
        final map = _sdkworkAsMap(json['replyTo']);
        return map == null ? null : MessageReplyReference.fromJson(map);
      })(),
      renderHints: _sdkworkAsMap(json['renderHints']),
      summary: json['summary']?.toString(),
      metadata: _sdkworkAsMap(json['metadata'])
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'text': text,
      'parts': parts.map((item) => item.toJson()).toList(),
      'replyTo': replyTo?.toJson(),
      'renderHints': renderHints,
      'summary': summary,
      'metadata': metadata,
    };
  }
}

class TimelineViewEntry {
  final String tenantId;
  final String conversationId;
  final String messageId;
  final int messageSeq;
  final String? summary;
  final Sender sender;
  final MessageBody body;
  final String messageType;
  final String deliveryMode;
  final String? clientMsgId;
  final String? streamSessionId;
  final String? rtcSessionId;
  final String occurredAt;
  final String? committedAt;

  TimelineViewEntry({
    required this.tenantId,
    required this.conversationId,
    required this.messageId,
    required this.messageSeq,
    this.summary,
    required this.sender,
    required this.body,
    required this.messageType,
    required this.deliveryMode,
    this.clientMsgId,
    this.streamSessionId,
    this.rtcSessionId,
    required this.occurredAt,
    this.committedAt
  });

  factory TimelineViewEntry.fromJson(Map<String, dynamic> json) {
    return TimelineViewEntry(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('TimelineViewEntry.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('TimelineViewEntry.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('TimelineViewEntry.messageId is required');
        }
        return value;
      })(),
      messageSeq: (() {
        final value = json['messageSeq'];
        if (value is! int) {
          throw FormatException('TimelineViewEntry.messageSeq is required');
        }
        return value;
      })(),
      summary: json['summary']?.toString(),
      sender: (() {
        final map = _sdkworkAsMap(json['sender']);
        if (map == null) {
          throw FormatException('TimelineViewEntry.sender is required');
        }
        return Sender.fromJson(map);
      })(),
      body: (() {
        final map = _sdkworkAsMap(json['body']);
        if (map == null) {
          throw FormatException('TimelineViewEntry.body is required');
        }
        return MessageBody.fromJson(map);
      })(),
      messageType: (() {
        final value = json['messageType']?.toString();
        if (value == null) {
          throw FormatException('TimelineViewEntry.messageType is required');
        }
        return value;
      })(),
      deliveryMode: (() {
        final value = json['deliveryMode']?.toString();
        if (value == null) {
          throw FormatException('TimelineViewEntry.deliveryMode is required');
        }
        return value;
      })(),
      clientMsgId: json['clientMsgId']?.toString(),
      streamSessionId: json['streamSessionId']?.toString(),
      rtcSessionId: json['rtcSessionId']?.toString(),
      occurredAt: (() {
        final value = json['occurredAt']?.toString();
        if (value == null) {
          throw FormatException('TimelineViewEntry.occurredAt is required');
        }
        return value;
      })(),
      committedAt: json['committedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'summary': summary,
      'sender': sender.toJson(),
      'body': body.toJson(),
      'messageType': messageType,
      'deliveryMode': deliveryMode,
      'clientMsgId': clientMsgId,
      'streamSessionId': streamSessionId,
      'rtcSessionId': rtcSessionId,
      'occurredAt': occurredAt,
      'committedAt': committedAt,
    };
  }
}

class TimelineResponse {
  final List<TimelineViewEntry> items;
  final int? nextAfterSeq;
  final bool hasMore;

  TimelineResponse({
    required this.items,
    this.nextAfterSeq,
    required this.hasMore
  });

  factory TimelineResponse.fromJson(Map<String, dynamic> json) {
    return TimelineResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('TimelineResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : TimelineViewEntry.fromJson(map);
      })())
            .whereType<TimelineViewEntry>()
            .toList();
      })(),
      nextAfterSeq: json['nextAfterSeq'] is int ? json['nextAfterSeq'] : null,
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('TimelineResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextAfterSeq': nextAfterSeq,
      'hasMore': hasMore,
    };
  }
}

class PostMessageRequest {
  final String? text;
  final List<ContentPart>? parts;
  final MessageReplyReference? replyTo;
  final String? clientMsgId;
  final String? summary;
  final Map<String, dynamic>? renderHints;

  PostMessageRequest({
    this.text,
    this.parts,
    this.replyTo,
    this.clientMsgId,
    this.summary,
    this.renderHints
  });

  factory PostMessageRequest.fromJson(Map<String, dynamic> json) {
    return PostMessageRequest(
      text: json['text']?.toString(),
      parts: (() {
        final list = _sdkworkAsList(json['parts']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ContentPart.fromJson(map);
      })())
            .whereType<ContentPart>()
            .toList();
      })(),
      replyTo: (() {
        final map = _sdkworkAsMap(json['replyTo']);
        return map == null ? null : MessageReplyReference.fromJson(map);
      })(),
      clientMsgId: json['clientMsgId']?.toString(),
      summary: json['summary']?.toString(),
      renderHints: _sdkworkAsMap(json['renderHints'])
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'text': text,
      'parts': parts?.map((item) => item.toJson()).toList(),
      'replyTo': replyTo?.toJson(),
      'clientMsgId': clientMsgId,
      'summary': summary,
      'renderHints': renderHints,
    };
  }
}

class EditMessageRequest {
  final String? text;
  final List<ContentPart>? parts;
  final MessageReplyReference? replyTo;

  EditMessageRequest({
    this.text,
    this.parts,
    this.replyTo
  });

  factory EditMessageRequest.fromJson(Map<String, dynamic> json) {
    return EditMessageRequest(
      text: json['text']?.toString(),
      parts: (() {
        final list = _sdkworkAsList(json['parts']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ContentPart.fromJson(map);
      })())
            .whereType<ContentPart>()
            .toList();
      })(),
      replyTo: (() {
        final map = _sdkworkAsMap(json['replyTo']);
        return map == null ? null : MessageReplyReference.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'text': text,
      'parts': parts?.map((item) => item.toJson()).toList(),
      'replyTo': replyTo?.toJson(),
    };
  }
}

class PostedMessageResponse {
  final String conversationId;
  final String messageId;
  final int messageSeq;
  final MessageBody body;
  final String occurredAt;

  PostedMessageResponse({
    required this.conversationId,
    required this.messageId,
    required this.messageSeq,
    required this.body,
    required this.occurredAt
  });

  factory PostedMessageResponse.fromJson(Map<String, dynamic> json) {
    return PostedMessageResponse(
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('PostedMessageResponse.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('PostedMessageResponse.messageId is required');
        }
        return value;
      })(),
      messageSeq: (() {
        final value = json['messageSeq'];
        if (value is! int) {
          throw FormatException('PostedMessageResponse.messageSeq is required');
        }
        return value;
      })(),
      body: (() {
        final map = _sdkworkAsMap(json['body']);
        if (map == null) {
          throw FormatException('PostedMessageResponse.body is required');
        }
        return MessageBody.fromJson(map);
      })(),
      occurredAt: (() {
        final value = json['occurredAt']?.toString();
        if (value == null) {
          throw FormatException('PostedMessageResponse.occurredAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'body': body.toJson(),
      'occurredAt': occurredAt,
    };
  }
}

class MessageReactionRequest {
  final String reactionKey;

  MessageReactionRequest({
    required this.reactionKey
  });

  factory MessageReactionRequest.fromJson(Map<String, dynamic> json) {
    return MessageReactionRequest(
      reactionKey: (() {
        final value = json['reactionKey']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionRequest.reactionKey is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'reactionKey': reactionKey,
    };
  }
}

class MessageReactionCountView {
  final String reactionKey;
  final int count;

  MessageReactionCountView({
    required this.reactionKey,
    required this.count
  });

  factory MessageReactionCountView.fromJson(Map<String, dynamic> json) {
    return MessageReactionCountView(
      reactionKey: (() {
        final value = json['reactionKey']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionCountView.reactionKey is required');
        }
        return value;
      })(),
      count: (() {
        final value = json['count'];
        if (value is! int) {
          throw FormatException('MessageReactionCountView.count is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'reactionKey': reactionKey,
      'count': count,
    };
  }
}

class InteractionActorView {
  final String id;
  final String kind;

  InteractionActorView({
    required this.id,
    required this.kind
  });

  factory InteractionActorView.fromJson(Map<String, dynamic> json) {
    return InteractionActorView(
      id: (() {
        final value = json['id']?.toString();
        if (value == null) {
          throw FormatException('InteractionActorView.id is required');
        }
        return value;
      })(),
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('InteractionActorView.kind is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'kind': kind,
    };
  }
}

class MessagePinView {
  final InteractionActorView pinnedBy;
  final String pinnedAt;

  MessagePinView({
    required this.pinnedBy,
    required this.pinnedAt
  });

  factory MessagePinView.fromJson(Map<String, dynamic> json) {
    return MessagePinView(
      pinnedBy: (() {
        final map = _sdkworkAsMap(json['pinnedBy']);
        if (map == null) {
          throw FormatException('MessagePinView.pinnedBy is required');
        }
        return InteractionActorView.fromJson(map);
      })(),
      pinnedAt: (() {
        final value = json['pinnedAt']?.toString();
        if (value == null) {
          throw FormatException('MessagePinView.pinnedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'pinnedBy': pinnedBy.toJson(),
      'pinnedAt': pinnedAt,
    };
  }
}

class MessageInteractionSummaryView {
  final String tenantId;
  final String conversationId;
  final String messageId;
  final int messageSeq;
  final int totalReactionCount;
  final List<MessageReactionCountView> reactionCounts;
  final MessagePinView? pin;

  MessageInteractionSummaryView({
    required this.tenantId,
    required this.conversationId,
    required this.messageId,
    required this.messageSeq,
    required this.totalReactionCount,
    required this.reactionCounts,
    this.pin
  });

  factory MessageInteractionSummaryView.fromJson(Map<String, dynamic> json) {
    return MessageInteractionSummaryView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('MessageInteractionSummaryView.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('MessageInteractionSummaryView.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('MessageInteractionSummaryView.messageId is required');
        }
        return value;
      })(),
      messageSeq: (() {
        final value = json['messageSeq'];
        if (value is! int) {
          throw FormatException('MessageInteractionSummaryView.messageSeq is required');
        }
        return value;
      })(),
      totalReactionCount: (() {
        final value = json['totalReactionCount'];
        if (value is! int) {
          throw FormatException('MessageInteractionSummaryView.totalReactionCount is required');
        }
        return value;
      })(),
      reactionCounts: (() {
        final list = _sdkworkAsList(json['reactionCounts']);
        if (list == null) {
          throw FormatException('MessageInteractionSummaryView.reactionCounts is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MessageReactionCountView.fromJson(map);
      })())
            .whereType<MessageReactionCountView>()
            .toList();
      })(),
      pin: (() {
        final map = _sdkworkAsMap(json['pin']);
        return map == null ? null : MessagePinView.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'totalReactionCount': totalReactionCount,
      'reactionCounts': reactionCounts.map((item) => item.toJson()).toList(),
      'pin': pin?.toJson(),
    };
  }
}

class MessageReactionMutationResult {
  final String tenantId;
  final String conversationId;
  final String messageId;
  final String reactionKey;
  final int count;
  final String updatedAt;

  MessageReactionMutationResult({
    required this.tenantId,
    required this.conversationId,
    required this.messageId,
    required this.reactionKey,
    required this.count,
    required this.updatedAt
  });

  factory MessageReactionMutationResult.fromJson(Map<String, dynamic> json) {
    return MessageReactionMutationResult(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionMutationResult.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionMutationResult.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionMutationResult.messageId is required');
        }
        return value;
      })(),
      reactionKey: (() {
        final value = json['reactionKey']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionMutationResult.reactionKey is required');
        }
        return value;
      })(),
      count: (() {
        final value = json['count'];
        if (value is! int) {
          throw FormatException('MessageReactionMutationResult.count is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionMutationResult.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageId': messageId,
      'reactionKey': reactionKey,
      'count': count,
      'updatedAt': updatedAt,
    };
  }
}

class MessagePinMutationResult {
  final String tenantId;
  final String conversationId;
  final String messageId;
  final bool isPinned;
  final String updatedAt;

  MessagePinMutationResult({
    required this.tenantId,
    required this.conversationId,
    required this.messageId,
    required this.isPinned,
    required this.updatedAt
  });

  factory MessagePinMutationResult.fromJson(Map<String, dynamic> json) {
    return MessagePinMutationResult(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('MessagePinMutationResult.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('MessagePinMutationResult.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('MessagePinMutationResult.messageId is required');
        }
        return value;
      })(),
      isPinned: (() {
        final value = json['isPinned'];
        if (value is! bool) {
          throw FormatException('MessagePinMutationResult.isPinned is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('MessagePinMutationResult.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageId': messageId,
      'isPinned': isPinned,
      'updatedAt': updatedAt,
    };
  }
}

class MessageVisibilityMutationResult {
  final String tenantId;
  final String conversationId;
  final String messageId;
  final int messageSeq;
  final String principalKind;
  final String principalId;
  final bool isDeleted;
  final String updatedAt;

  MessageVisibilityMutationResult({
    required this.tenantId,
    required this.conversationId,
    required this.messageId,
    required this.messageSeq,
    required this.principalKind,
    required this.principalId,
    required this.isDeleted,
    required this.updatedAt
  });

  factory MessageVisibilityMutationResult.fromJson(Map<String, dynamic> json) {
    return MessageVisibilityMutationResult(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('MessageVisibilityMutationResult.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('MessageVisibilityMutationResult.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('MessageVisibilityMutationResult.messageId is required');
        }
        return value;
      })(),
      messageSeq: (() {
        final value = json['messageSeq'];
        if (value is! int) {
          throw FormatException('MessageVisibilityMutationResult.messageSeq is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('MessageVisibilityMutationResult.principalKind is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('MessageVisibilityMutationResult.principalId is required');
        }
        return value;
      })(),
      isDeleted: (() {
        final value = json['isDeleted'];
        if (value is! bool) {
          throw FormatException('MessageVisibilityMutationResult.isDeleted is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('MessageVisibilityMutationResult.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'principalKind': principalKind,
      'principalId': principalId,
      'isDeleted': isDeleted,
      'updatedAt': updatedAt,
    };
  }
}

class FavoriteMessageRequest {
  final String conversationId;
  final String favoriteType;
  final String title;
  final String contentPreview;
  final String sourceDisplayName;

  FavoriteMessageRequest({
    required this.conversationId,
    required this.favoriteType,
    required this.title,
    required this.contentPreview,
    required this.sourceDisplayName
  });

  factory FavoriteMessageRequest.fromJson(Map<String, dynamic> json) {
    return FavoriteMessageRequest(
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('FavoriteMessageRequest.conversationId is required');
        }
        return value;
      })(),
      favoriteType: (() {
        final value = json['favoriteType']?.toString();
        if (value == null) {
          throw FormatException('FavoriteMessageRequest.favoriteType is required');
        }
        return value;
      })(),
      title: (() {
        final value = json['title']?.toString();
        if (value == null) {
          throw FormatException('FavoriteMessageRequest.title is required');
        }
        return value;
      })(),
      contentPreview: (() {
        final value = json['contentPreview']?.toString();
        if (value == null) {
          throw FormatException('FavoriteMessageRequest.contentPreview is required');
        }
        return value;
      })(),
      sourceDisplayName: (() {
        final value = json['sourceDisplayName']?.toString();
        if (value == null) {
          throw FormatException('FavoriteMessageRequest.sourceDisplayName is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'favoriteType': favoriteType,
      'title': title,
      'contentPreview': contentPreview,
      'sourceDisplayName': sourceDisplayName,
    };
  }
}

class MessageFavoriteView {
  final String tenantId;
  final String principalKind;
  final String principalId;
  final String favoriteId;
  final String favoriteType;
  final String conversationId;
  final String messageId;
  final int messageSeq;
  final String title;
  final String contentPreview;
  final String sourceDisplayName;
  final String favoritedAt;

  MessageFavoriteView({
    required this.tenantId,
    required this.principalKind,
    required this.principalId,
    required this.favoriteId,
    required this.favoriteType,
    required this.conversationId,
    required this.messageId,
    required this.messageSeq,
    required this.title,
    required this.contentPreview,
    required this.sourceDisplayName,
    required this.favoritedAt
  });

  factory MessageFavoriteView.fromJson(Map<String, dynamic> json) {
    return MessageFavoriteView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.tenantId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.principalKind is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.principalId is required');
        }
        return value;
      })(),
      favoriteId: (() {
        final value = json['favoriteId']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.favoriteId is required');
        }
        return value;
      })(),
      favoriteType: (() {
        final value = json['favoriteType']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.favoriteType is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.messageId is required');
        }
        return value;
      })(),
      messageSeq: (() {
        final value = json['messageSeq'];
        if (value is! int) {
          throw FormatException('MessageFavoriteView.messageSeq is required');
        }
        return value;
      })(),
      title: (() {
        final value = json['title']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.title is required');
        }
        return value;
      })(),
      contentPreview: (() {
        final value = json['contentPreview']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.contentPreview is required');
        }
        return value;
      })(),
      sourceDisplayName: (() {
        final value = json['sourceDisplayName']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.sourceDisplayName is required');
        }
        return value;
      })(),
      favoritedAt: (() {
        final value = json['favoritedAt']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.favoritedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalKind': principalKind,
      'principalId': principalId,
      'favoriteId': favoriteId,
      'favoriteType': favoriteType,
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'title': title,
      'contentPreview': contentPreview,
      'sourceDisplayName': sourceDisplayName,
      'favoritedAt': favoritedAt,
    };
  }
}

class FavoriteMessagesResponse {
  final List<MessageFavoriteView> items;
  final String? nextCursor;
  final bool hasMore;

  FavoriteMessagesResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory FavoriteMessagesResponse.fromJson(Map<String, dynamic> json) {
    return FavoriteMessagesResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('FavoriteMessagesResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MessageFavoriteView.fromJson(map);
      })())
            .whereType<MessageFavoriteView>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('FavoriteMessagesResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class DeleteMessageFavoriteResponse {
  final String favoriteId;
  final bool deleted;

  DeleteMessageFavoriteResponse({
    required this.favoriteId,
    required this.deleted
  });

  factory DeleteMessageFavoriteResponse.fromJson(Map<String, dynamic> json) {
    return DeleteMessageFavoriteResponse(
      favoriteId: (() {
        final value = json['favoriteId']?.toString();
        if (value == null) {
          throw FormatException('DeleteMessageFavoriteResponse.favoriteId is required');
        }
        return value;
      })(),
      deleted: (() {
        final value = json['deleted'];
        if (value is! bool) {
          throw FormatException('DeleteMessageFavoriteResponse.deleted is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'favoriteId': favoriteId,
      'deleted': deleted,
    };
  }
}

class ConversationPreferencesView {
  final String tenantId;
  final String conversationId;
  final String principalKind;
  final String principalId;
  final bool isPinned;
  final bool isMuted;
  final bool isMarkedUnread;
  final bool isHidden;
  final String updatedAt;

  ConversationPreferencesView({
    required this.tenantId,
    required this.conversationId,
    required this.principalKind,
    required this.principalId,
    required this.isPinned,
    required this.isMuted,
    required this.isMarkedUnread,
    required this.isHidden,
    required this.updatedAt
  });

  factory ConversationPreferencesView.fromJson(Map<String, dynamic> json) {
    return ConversationPreferencesView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ConversationPreferencesView.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('ConversationPreferencesView.conversationId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('ConversationPreferencesView.principalKind is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('ConversationPreferencesView.principalId is required');
        }
        return value;
      })(),
      isPinned: (() {
        final value = json['isPinned'];
        if (value is! bool) {
          throw FormatException('ConversationPreferencesView.isPinned is required');
        }
        return value;
      })(),
      isMuted: (() {
        final value = json['isMuted'];
        if (value is! bool) {
          throw FormatException('ConversationPreferencesView.isMuted is required');
        }
        return value;
      })(),
      isMarkedUnread: (() {
        final value = json['isMarkedUnread'];
        if (value is! bool) {
          throw FormatException('ConversationPreferencesView.isMarkedUnread is required');
        }
        return value;
      })(),
      isHidden: (() {
        final value = json['isHidden'];
        if (value is! bool) {
          throw FormatException('ConversationPreferencesView.isHidden is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('ConversationPreferencesView.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'principalKind': principalKind,
      'principalId': principalId,
      'isPinned': isPinned,
      'isMuted': isMuted,
      'isMarkedUnread': isMarkedUnread,
      'isHidden': isHidden,
      'updatedAt': updatedAt,
    };
  }
}

class UpdateConversationPreferencesRequest {
  final bool? isPinned;
  final bool? isMuted;
  final bool? isMarkedUnread;
  final bool? isHidden;

  UpdateConversationPreferencesRequest({
    this.isPinned,
    this.isMuted,
    this.isMarkedUnread,
    this.isHidden
  });

  factory UpdateConversationPreferencesRequest.fromJson(Map<String, dynamic> json) {
    return UpdateConversationPreferencesRequest(
      isPinned: json['isPinned'] is bool ? json['isPinned'] : null,
      isMuted: json['isMuted'] is bool ? json['isMuted'] : null,
      isMarkedUnread: json['isMarkedUnread'] is bool ? json['isMarkedUnread'] : null,
      isHidden: json['isHidden'] is bool ? json['isHidden'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'isPinned': isPinned,
      'isMuted': isMuted,
      'isMarkedUnread': isMarkedUnread,
      'isHidden': isHidden,
    };
  }
}

class ConversationProfileView {
  final String tenantId;
  final String conversationId;
  final String displayName;
  final String avatarUrl;
  final String notice;
  final String updatedAt;
  final String? updatedByPrincipalKind;
  final String? updatedByPrincipalId;

  ConversationProfileView({
    required this.tenantId,
    required this.conversationId,
    required this.displayName,
    required this.avatarUrl,
    required this.notice,
    required this.updatedAt,
    this.updatedByPrincipalKind,
    this.updatedByPrincipalId
  });

  factory ConversationProfileView.fromJson(Map<String, dynamic> json) {
    return ConversationProfileView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ConversationProfileView.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('ConversationProfileView.conversationId is required');
        }
        return value;
      })(),
      displayName: (() {
        final value = json['displayName']?.toString();
        if (value == null) {
          throw FormatException('ConversationProfileView.displayName is required');
        }
        return value;
      })(),
      avatarUrl: (() {
        final value = json['avatarUrl']?.toString();
        if (value == null) {
          throw FormatException('ConversationProfileView.avatarUrl is required');
        }
        return value;
      })(),
      notice: (() {
        final value = json['notice']?.toString();
        if (value == null) {
          throw FormatException('ConversationProfileView.notice is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('ConversationProfileView.updatedAt is required');
        }
        return value;
      })(),
      updatedByPrincipalKind: json['updatedByPrincipalKind']?.toString(),
      updatedByPrincipalId: json['updatedByPrincipalId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'displayName': displayName,
      'avatarUrl': avatarUrl,
      'notice': notice,
      'updatedAt': updatedAt,
      'updatedByPrincipalKind': updatedByPrincipalKind,
      'updatedByPrincipalId': updatedByPrincipalId,
    };
  }
}

class UpdateConversationProfileRequest {
  final String? displayName;
  final String? avatarUrl;
  final String? notice;

  UpdateConversationProfileRequest({
    this.displayName,
    this.avatarUrl,
    this.notice
  });

  factory UpdateConversationProfileRequest.fromJson(Map<String, dynamic> json) {
    return UpdateConversationProfileRequest(
      displayName: json['displayName']?.toString(),
      avatarUrl: json['avatarUrl']?.toString(),
      notice: json['notice']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'displayName': displayName,
      'avatarUrl': avatarUrl,
      'notice': notice,
    };
  }
}

class ConversationSummaryView {
  final String tenantId;
  final String conversationId;
  final int messageCount;
  final int lastMessageSeq;
  final String? lastSummary;
  final String? lastMessageAt;

  ConversationSummaryView({
    required this.tenantId,
    required this.conversationId,
    required this.messageCount,
    required this.lastMessageSeq,
    this.lastSummary,
    this.lastMessageAt
  });

  factory ConversationSummaryView.fromJson(Map<String, dynamic> json) {
    return ConversationSummaryView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ConversationSummaryView.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('ConversationSummaryView.conversationId is required');
        }
        return value;
      })(),
      messageCount: (() {
        final value = json['messageCount'];
        if (value is! int) {
          throw FormatException('ConversationSummaryView.messageCount is required');
        }
        return value;
      })(),
      lastMessageSeq: (() {
        final value = json['lastMessageSeq'];
        if (value is! int) {
          throw FormatException('ConversationSummaryView.lastMessageSeq is required');
        }
        return value;
      })(),
      lastSummary: json['lastSummary']?.toString(),
      lastMessageAt: json['lastMessageAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageCount': messageCount,
      'lastMessageSeq': lastMessageSeq,
      'lastSummary': lastSummary,
      'lastMessageAt': lastMessageAt,
    };
  }
}

class ConversationInboxPeerView {
  final String principalKind;
  final String principalId;
  final String? userId;
  final String? chatId;
  final String? displayName;
  final String? avatarUrl;
  final String? relationshipState;

  ConversationInboxPeerView({
    required this.principalKind,
    required this.principalId,
    this.userId,
    this.chatId,
    this.displayName,
    this.avatarUrl,
    this.relationshipState
  });

  factory ConversationInboxPeerView.fromJson(Map<String, dynamic> json) {
    return ConversationInboxPeerView(
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('ConversationInboxPeerView.principalKind is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('ConversationInboxPeerView.principalId is required');
        }
        return value;
      })(),
      userId: json['userId']?.toString(),
      chatId: json['chatId']?.toString(),
      displayName: json['displayName']?.toString(),
      avatarUrl: json['avatarUrl']?.toString(),
      relationshipState: json['relationshipState']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'principalKind': principalKind,
      'principalId': principalId,
      'userId': userId,
      'chatId': chatId,
      'displayName': displayName,
      'avatarUrl': avatarUrl,
      'relationshipState': relationshipState,
    };
  }
}

class ConversationInboxPreferencesView {
  final bool isPinned;
  final bool isMuted;
  final bool isMarkedUnread;
  final bool isHidden;

  ConversationInboxPreferencesView({
    required this.isPinned,
    required this.isMuted,
    required this.isMarkedUnread,
    required this.isHidden
  });

  factory ConversationInboxPreferencesView.fromJson(Map<String, dynamic> json) {
    return ConversationInboxPreferencesView(
      isPinned: (() {
        final value = json['isPinned'];
        if (value is! bool) {
          throw FormatException('ConversationInboxPreferencesView.isPinned is required');
        }
        return value;
      })(),
      isMuted: (() {
        final value = json['isMuted'];
        if (value is! bool) {
          throw FormatException('ConversationInboxPreferencesView.isMuted is required');
        }
        return value;
      })(),
      isMarkedUnread: (() {
        final value = json['isMarkedUnread'];
        if (value is! bool) {
          throw FormatException('ConversationInboxPreferencesView.isMarkedUnread is required');
        }
        return value;
      })(),
      isHidden: (() {
        final value = json['isHidden'];
        if (value is! bool) {
          throw FormatException('ConversationInboxPreferencesView.isHidden is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'isPinned': isPinned,
      'isMuted': isMuted,
      'isMarkedUnread': isMarkedUnread,
      'isHidden': isHidden,
    };
  }
}

class ConversationInboxEntry {
  final String tenantId;
  final String conversationId;
  final bool? agentHandoff;
  final String conversationType;
  final String? displayName;
  final String? avatarUrl;
  final String? displaySource;
  final ConversationInboxPeerView? peer;
  final ConversationInboxPreferencesView? preferences;
  final String lastActivityAt;
  final String? lastMessageId;
  final String? lastSenderId;
  final int messageCount;
  final int lastMessageSeq;
  final String? lastSummary;
  final String? lastMessageAt;
  final int unreadCount;

  ConversationInboxEntry({
    required this.tenantId,
    required this.conversationId,
    this.agentHandoff,
    required this.conversationType,
    this.displayName,
    this.avatarUrl,
    this.displaySource,
    this.peer,
    this.preferences,
    required this.lastActivityAt,
    this.lastMessageId,
    this.lastSenderId,
    required this.messageCount,
    required this.lastMessageSeq,
    this.lastSummary,
    this.lastMessageAt,
    required this.unreadCount
  });

  factory ConversationInboxEntry.fromJson(Map<String, dynamic> json) {
    return ConversationInboxEntry(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ConversationInboxEntry.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('ConversationInboxEntry.conversationId is required');
        }
        return value;
      })(),
      agentHandoff: json['agentHandoff'] is bool ? json['agentHandoff'] : null,
      conversationType: (() {
        final value = json['conversationType']?.toString();
        if (value == null) {
          throw FormatException('ConversationInboxEntry.conversationType is required');
        }
        return value;
      })(),
      displayName: json['displayName']?.toString(),
      avatarUrl: json['avatarUrl']?.toString(),
      displaySource: json['displaySource']?.toString(),
      peer: (() {
        final map = _sdkworkAsMap(json['peer']);
        return map == null ? null : ConversationInboxPeerView.fromJson(map);
      })(),
      preferences: (() {
        final map = _sdkworkAsMap(json['preferences']);
        return map == null ? null : ConversationInboxPreferencesView.fromJson(map);
      })(),
      lastActivityAt: (() {
        final value = json['lastActivityAt']?.toString();
        if (value == null) {
          throw FormatException('ConversationInboxEntry.lastActivityAt is required');
        }
        return value;
      })(),
      lastMessageId: json['lastMessageId']?.toString(),
      lastSenderId: json['lastSenderId']?.toString(),
      messageCount: (() {
        final value = json['messageCount'];
        if (value is! int) {
          throw FormatException('ConversationInboxEntry.messageCount is required');
        }
        return value;
      })(),
      lastMessageSeq: (() {
        final value = json['lastMessageSeq'];
        if (value is! int) {
          throw FormatException('ConversationInboxEntry.lastMessageSeq is required');
        }
        return value;
      })(),
      lastSummary: json['lastSummary']?.toString(),
      lastMessageAt: json['lastMessageAt']?.toString(),
      unreadCount: (() {
        final value = json['unreadCount'];
        if (value is! int) {
          throw FormatException('ConversationInboxEntry.unreadCount is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'agentHandoff': agentHandoff,
      'conversationType': conversationType,
      'displayName': displayName,
      'avatarUrl': avatarUrl,
      'displaySource': displaySource,
      'peer': peer?.toJson(),
      'preferences': preferences?.toJson(),
      'lastActivityAt': lastActivityAt,
      'lastMessageId': lastMessageId,
      'lastSenderId': lastSenderId,
      'messageCount': messageCount,
      'lastMessageSeq': lastMessageSeq,
      'lastSummary': lastSummary,
      'lastMessageAt': lastMessageAt,
      'unreadCount': unreadCount,
    };
  }
}

class InboxResponse {
  final List<ConversationInboxEntry> items;
  final String? nextCursor;
  final bool hasMore;

  InboxResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory InboxResponse.fromJson(Map<String, dynamic> json) {
    return InboxResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('InboxResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ConversationInboxEntry.fromJson(map);
      })())
            .whereType<ConversationInboxEntry>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('InboxResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class ContactView {
  final String tenantId;
  final String ownerUserId;
  final String targetUserId;
  final String contactType;
  final String relationshipState;
  final String friendshipId;
  final String? directChatId;
  final String? conversationId;
  final String establishedAt;
  final String lastInteractionAt;

  ContactView({
    required this.tenantId,
    required this.ownerUserId,
    required this.targetUserId,
    required this.contactType,
    required this.relationshipState,
    required this.friendshipId,
    this.directChatId,
    this.conversationId,
    required this.establishedAt,
    required this.lastInteractionAt
  });

  factory ContactView.fromJson(Map<String, dynamic> json) {
    return ContactView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ContactView.tenantId is required');
        }
        return value;
      })(),
      ownerUserId: (() {
        final value = json['ownerUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactView.ownerUserId is required');
        }
        return value;
      })(),
      targetUserId: (() {
        final value = json['targetUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactView.targetUserId is required');
        }
        return value;
      })(),
      contactType: (() {
        final value = json['contactType']?.toString();
        if (value == null) {
          throw FormatException('ContactView.contactType is required');
        }
        return value;
      })(),
      relationshipState: (() {
        final value = json['relationshipState']?.toString();
        if (value == null) {
          throw FormatException('ContactView.relationshipState is required');
        }
        return value;
      })(),
      friendshipId: (() {
        final value = json['friendshipId']?.toString();
        if (value == null) {
          throw FormatException('ContactView.friendshipId is required');
        }
        return value;
      })(),
      directChatId: json['directChatId']?.toString(),
      conversationId: json['conversationId']?.toString(),
      establishedAt: (() {
        final value = json['establishedAt']?.toString();
        if (value == null) {
          throw FormatException('ContactView.establishedAt is required');
        }
        return value;
      })(),
      lastInteractionAt: (() {
        final value = json['lastInteractionAt']?.toString();
        if (value == null) {
          throw FormatException('ContactView.lastInteractionAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'ownerUserId': ownerUserId,
      'targetUserId': targetUserId,
      'contactType': contactType,
      'relationshipState': relationshipState,
      'friendshipId': friendshipId,
      'directChatId': directChatId,
      'conversationId': conversationId,
      'establishedAt': establishedAt,
      'lastInteractionAt': lastInteractionAt,
    };
  }
}

class ContactsResponse {
  final List<ContactView> items;
  final String? nextCursor;
  final bool hasMore;

  ContactsResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory ContactsResponse.fromJson(Map<String, dynamic> json) {
    return ContactsResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('ContactsResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ContactView.fromJson(map);
      })())
            .whereType<ContactView>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('ContactsResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class ContactPreferencesView {
  final String tenantId;
  final String ownerUserId;
  final String targetUserId;
  final bool isStarred;
  final String remark;
  final bool isBlocked;
  final String updatedAt;

  ContactPreferencesView({
    required this.tenantId,
    required this.ownerUserId,
    required this.targetUserId,
    required this.isStarred,
    required this.remark,
    required this.isBlocked,
    required this.updatedAt
  });

  factory ContactPreferencesView.fromJson(Map<String, dynamic> json) {
    return ContactPreferencesView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ContactPreferencesView.tenantId is required');
        }
        return value;
      })(),
      ownerUserId: (() {
        final value = json['ownerUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactPreferencesView.ownerUserId is required');
        }
        return value;
      })(),
      targetUserId: (() {
        final value = json['targetUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactPreferencesView.targetUserId is required');
        }
        return value;
      })(),
      isStarred: (() {
        final value = json['isStarred'];
        if (value is! bool) {
          throw FormatException('ContactPreferencesView.isStarred is required');
        }
        return value;
      })(),
      remark: (() {
        final value = json['remark']?.toString();
        if (value == null) {
          throw FormatException('ContactPreferencesView.remark is required');
        }
        return value;
      })(),
      isBlocked: (() {
        final value = json['isBlocked'];
        if (value is! bool) {
          throw FormatException('ContactPreferencesView.isBlocked is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('ContactPreferencesView.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'ownerUserId': ownerUserId,
      'targetUserId': targetUserId,
      'isStarred': isStarred,
      'remark': remark,
      'isBlocked': isBlocked,
      'updatedAt': updatedAt,
    };
  }
}

class UpdateContactPreferencesRequest {
  final bool? isStarred;
  final String? remark;
  final bool? isBlocked;

  UpdateContactPreferencesRequest({
    this.isStarred,
    this.remark,
    this.isBlocked
  });

  factory UpdateContactPreferencesRequest.fromJson(Map<String, dynamic> json) {
    return UpdateContactPreferencesRequest(
      isStarred: json['isStarred'] is bool ? json['isStarred'] : null,
      remark: json['remark']?.toString(),
      isBlocked: json['isBlocked'] is bool ? json['isBlocked'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'isStarred': isStarred,
      'remark': remark,
      'isBlocked': isBlocked,
    };
  }
}

class ContactTagView {
  final String tenantId;
  final String ownerUserId;
  final String tagId;
  final String name;
  final String color;
  final int count;
  final String bg;
  final String border;
  final String createdAt;
  final String updatedAt;

  ContactTagView({
    required this.tenantId,
    required this.ownerUserId,
    required this.tagId,
    required this.name,
    required this.color,
    required this.count,
    required this.bg,
    required this.border,
    required this.createdAt,
    required this.updatedAt
  });

  factory ContactTagView.fromJson(Map<String, dynamic> json) {
    return ContactTagView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.tenantId is required');
        }
        return value;
      })(),
      ownerUserId: (() {
        final value = json['ownerUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.ownerUserId is required');
        }
        return value;
      })(),
      tagId: (() {
        final value = json['tagId']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.tagId is required');
        }
        return value;
      })(),
      name: (() {
        final value = json['name']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.name is required');
        }
        return value;
      })(),
      color: (() {
        final value = json['color']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.color is required');
        }
        return value;
      })(),
      count: (() {
        final value = json['count'];
        if (value is! int) {
          throw FormatException('ContactTagView.count is required');
        }
        return value;
      })(),
      bg: (() {
        final value = json['bg']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.bg is required');
        }
        return value;
      })(),
      border: (() {
        final value = json['border']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.border is required');
        }
        return value;
      })(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.createdAt is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'ownerUserId': ownerUserId,
      'tagId': tagId,
      'name': name,
      'color': color,
      'count': count,
      'bg': bg,
      'border': border,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}

class ContactTagsResponse {
  final List<ContactTagView> items;
  final String? nextCursor;
  final bool hasMore;

  ContactTagsResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory ContactTagsResponse.fromJson(Map<String, dynamic> json) {
    return ContactTagsResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('ContactTagsResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ContactTagView.fromJson(map);
      })())
            .whereType<ContactTagView>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('ContactTagsResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class CreateContactTagRequest {
  final String name;
  final String color;
  final int? count;
  final String? bg;
  final String? border;

  CreateContactTagRequest({
    required this.name,
    required this.color,
    this.count,
    this.bg,
    this.border
  });

  factory CreateContactTagRequest.fromJson(Map<String, dynamic> json) {
    return CreateContactTagRequest(
      name: (() {
        final value = json['name']?.toString();
        if (value == null) {
          throw FormatException('CreateContactTagRequest.name is required');
        }
        return value;
      })(),
      color: (() {
        final value = json['color']?.toString();
        if (value == null) {
          throw FormatException('CreateContactTagRequest.color is required');
        }
        return value;
      })(),
      count: json['count'] is int ? json['count'] : null,
      bg: json['bg']?.toString(),
      border: json['border']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'name': name,
      'color': color,
      'count': count,
      'bg': bg,
      'border': border,
    };
  }
}

class UpdateContactTagRequest {
  final String? name;
  final String? color;
  final int? count;
  final String? bg;
  final String? border;

  UpdateContactTagRequest({
    this.name,
    this.color,
    this.count,
    this.bg,
    this.border
  });

  factory UpdateContactTagRequest.fromJson(Map<String, dynamic> json) {
    return UpdateContactTagRequest(
      name: json['name']?.toString(),
      color: json['color']?.toString(),
      count: json['count'] is int ? json['count'] : null,
      bg: json['bg']?.toString(),
      border: json['border']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'name': name,
      'color': color,
      'count': count,
      'bg': bg,
      'border': border,
    };
  }
}

class DeleteContactTagResponse {
  final String tagId;
  final bool deleted;

  DeleteContactTagResponse({
    required this.tagId,
    required this.deleted
  });

  factory DeleteContactTagResponse.fromJson(Map<String, dynamic> json) {
    return DeleteContactTagResponse(
      tagId: (() {
        final value = json['tagId']?.toString();
        if (value == null) {
          throw FormatException('DeleteContactTagResponse.tagId is required');
        }
        return value;
      })(),
      deleted: (() {
        final value = json['deleted'];
        if (value is! bool) {
          throw FormatException('DeleteContactTagResponse.deleted is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tagId': tagId,
      'deleted': deleted,
    };
  }
}

class ContactRecommendationView {
  final String tenantId;
  final String ownerUserId;
  final String targetUserId;
  final String recommendationId;
  final String? targetConversationId;
  final String createdAt;

  ContactRecommendationView({
    required this.tenantId,
    required this.ownerUserId,
    required this.targetUserId,
    required this.recommendationId,
    this.targetConversationId,
    required this.createdAt
  });

  factory ContactRecommendationView.fromJson(Map<String, dynamic> json) {
    return ContactRecommendationView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ContactRecommendationView.tenantId is required');
        }
        return value;
      })(),
      ownerUserId: (() {
        final value = json['ownerUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactRecommendationView.ownerUserId is required');
        }
        return value;
      })(),
      targetUserId: (() {
        final value = json['targetUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactRecommendationView.targetUserId is required');
        }
        return value;
      })(),
      recommendationId: (() {
        final value = json['recommendationId']?.toString();
        if (value == null) {
          throw FormatException('ContactRecommendationView.recommendationId is required');
        }
        return value;
      })(),
      targetConversationId: json['targetConversationId']?.toString(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('ContactRecommendationView.createdAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'ownerUserId': ownerUserId,
      'targetUserId': targetUserId,
      'recommendationId': recommendationId,
      'targetConversationId': targetConversationId,
      'createdAt': createdAt,
    };
  }
}

class CreateContactRecommendationRequest {
  final String? targetConversationId;

  CreateContactRecommendationRequest({
    this.targetConversationId
  });

  factory CreateContactRecommendationRequest.fromJson(Map<String, dynamic> json) {
    return CreateContactRecommendationRequest(
      targetConversationId: json['targetConversationId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'targetConversationId': targetConversationId,
    };
  }
}

class SocialUserSearchResult {
  final String tenantId;
  final String userId;
  final String chatId;
  final String displayName;
  final String relationshipState;
  final String? avatarUrl;
  final String? email;
  final String? phone;
  final Map<String, dynamic>? metadata;

  SocialUserSearchResult({
    required this.tenantId,
    required this.userId,
    required this.chatId,
    required this.displayName,
    required this.relationshipState,
    this.avatarUrl,
    this.email,
    this.phone,
    this.metadata
  });

  factory SocialUserSearchResult.fromJson(Map<String, dynamic> json) {
    return SocialUserSearchResult(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('SocialUserSearchResult.tenantId is required');
        }
        return value;
      })(),
      userId: (() {
        final value = json['userId']?.toString();
        if (value == null) {
          throw FormatException('SocialUserSearchResult.userId is required');
        }
        return value;
      })(),
      chatId: (() {
        final value = json['chatId']?.toString();
        if (value == null) {
          throw FormatException('SocialUserSearchResult.chatId is required');
        }
        return value;
      })(),
      displayName: (() {
        final value = json['displayName']?.toString();
        if (value == null) {
          throw FormatException('SocialUserSearchResult.displayName is required');
        }
        return value;
      })(),
      relationshipState: (() {
        final value = json['relationshipState']?.toString();
        if (value == null) {
          throw FormatException('SocialUserSearchResult.relationshipState is required');
        }
        return value;
      })(),
      avatarUrl: json['avatarUrl']?.toString(),
      email: json['email']?.toString(),
      phone: json['phone']?.toString(),
      metadata: _sdkworkAsMap(json['metadata'])
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'userId': userId,
      'chatId': chatId,
      'displayName': displayName,
      'relationshipState': relationshipState,
      'avatarUrl': avatarUrl,
      'email': email,
      'phone': phone,
      'metadata': metadata,
    };
  }
}

class SocialUserSearchResponse {
  final List<SocialUserSearchResult> items;
  final String? nextCursor;
  final bool hasMore;

  SocialUserSearchResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory SocialUserSearchResponse.fromJson(Map<String, dynamic> json) {
    return SocialUserSearchResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SocialUserSearchResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : SocialUserSearchResult.fromJson(map);
      })())
            .whereType<SocialUserSearchResult>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('SocialUserSearchResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class SubmitFriendRequestRequest {
  final String targetUserId;
  final String? requestMessage;

  SubmitFriendRequestRequest({
    required this.targetUserId,
    this.requestMessage
  });

  factory SubmitFriendRequestRequest.fromJson(Map<String, dynamic> json) {
    return SubmitFriendRequestRequest(
      targetUserId: (() {
        final value = json['targetUserId']?.toString();
        if (value == null) {
          throw FormatException('SubmitFriendRequestRequest.targetUserId is required');
        }
        return value;
      })(),
      requestMessage: json['requestMessage']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'targetUserId': targetUserId,
      'requestMessage': requestMessage,
    };
  }
}

class FriendRequest {
  final String tenantId;
  final String requestId;
  final String requesterUserId;
  final String targetUserId;
  final String status;
  final String? requestMessage;
  final String createdAt;
  final String updatedAt;

  FriendRequest({
    required this.tenantId,
    required this.requestId,
    required this.requesterUserId,
    required this.targetUserId,
    required this.status,
    this.requestMessage,
    required this.createdAt,
    required this.updatedAt
  });

  factory FriendRequest.fromJson(Map<String, dynamic> json) {
    return FriendRequest(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.tenantId is required');
        }
        return value;
      })(),
      requestId: (() {
        final value = json['requestId']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.requestId is required');
        }
        return value;
      })(),
      requesterUserId: (() {
        final value = json['requesterUserId']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.requesterUserId is required');
        }
        return value;
      })(),
      targetUserId: (() {
        final value = json['targetUserId']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.targetUserId is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.status is required');
        }
        return value;
      })(),
      requestMessage: json['requestMessage']?.toString(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.createdAt is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'requestId': requestId,
      'requesterUserId': requesterUserId,
      'targetUserId': targetUserId,
      'status': status,
      'requestMessage': requestMessage,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}

class Friendship {
  final String tenantId;
  final String friendshipId;
  final String initiatorUserId;
  final String leftUserId;
  final String rightUserId;
  final String userHighId;
  final String userLowId;
  final String status;
  final String createdAt;

  Friendship({
    required this.tenantId,
    required this.friendshipId,
    required this.initiatorUserId,
    required this.leftUserId,
    required this.rightUserId,
    required this.userHighId,
    required this.userLowId,
    required this.status,
    required this.createdAt
  });

  factory Friendship.fromJson(Map<String, dynamic> json) {
    return Friendship(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.tenantId is required');
        }
        return value;
      })(),
      friendshipId: (() {
        final value = json['friendshipId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.friendshipId is required');
        }
        return value;
      })(),
      initiatorUserId: (() {
        final value = json['initiatorUserId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.initiatorUserId is required');
        }
        return value;
      })(),
      leftUserId: (() {
        final value = json['leftUserId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.leftUserId is required');
        }
        return value;
      })(),
      rightUserId: (() {
        final value = json['rightUserId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.rightUserId is required');
        }
        return value;
      })(),
      userHighId: (() {
        final value = json['userHighId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.userHighId is required');
        }
        return value;
      })(),
      userLowId: (() {
        final value = json['userLowId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.userLowId is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status']?.toString();
        if (value == null) {
          throw FormatException('Friendship.status is required');
        }
        return value;
      })(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('Friendship.createdAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'friendshipId': friendshipId,
      'initiatorUserId': initiatorUserId,
      'leftUserId': leftUserId,
      'rightUserId': rightUserId,
      'userHighId': userHighId,
      'userLowId': userLowId,
      'status': status,
      'createdAt': createdAt,
    };
  }
}

class DirectChat {
  final String tenantId;
  final String directChatId;
  final String conversationId;
  final String status;

  DirectChat({
    required this.tenantId,
    required this.directChatId,
    required this.conversationId,
    required this.status
  });

  factory DirectChat.fromJson(Map<String, dynamic> json) {
    return DirectChat(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('DirectChat.tenantId is required');
        }
        return value;
      })(),
      directChatId: (() {
        final value = json['directChatId']?.toString();
        if (value == null) {
          throw FormatException('DirectChat.directChatId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('DirectChat.conversationId is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status']?.toString();
        if (value == null) {
          throw FormatException('DirectChat.status is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'directChatId': directChatId,
      'conversationId': conversationId,
      'status': status,
    };
  }
}

class SocialFriendRequestMutationResponse {
  final FriendRequest friendRequest;

  SocialFriendRequestMutationResponse({
    required this.friendRequest
  });

  factory SocialFriendRequestMutationResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestMutationResponse(
      friendRequest: (() {
        final map = _sdkworkAsMap(json['friendRequest']);
        if (map == null) {
          throw FormatException('SocialFriendRequestMutationResponse.friendRequest is required');
        }
        return FriendRequest.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'friendRequest': friendRequest.toJson(),
    };
  }
}

class SocialFriendRequestListResponse {
  final List<FriendRequest> items;
  final String? nextCursor;

  SocialFriendRequestListResponse({
    required this.items,
    this.nextCursor
  });

  factory SocialFriendRequestListResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestListResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SocialFriendRequestListResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : FriendRequest.fromJson(map);
      })())
            .whereType<FriendRequest>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
    };
  }
}

class SocialFriendRequestAcceptanceResponse {
  final FriendRequest friendRequest;
  final Friendship friendship;
  final DirectChat directChat;
  final CreateConversationResult conversation;

  SocialFriendRequestAcceptanceResponse({
    required this.friendRequest,
    required this.friendship,
    required this.directChat,
    required this.conversation
  });

  factory SocialFriendRequestAcceptanceResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestAcceptanceResponse(
      friendRequest: (() {
        final map = _sdkworkAsMap(json['friendRequest']);
        if (map == null) {
          throw FormatException('SocialFriendRequestAcceptanceResponse.friendRequest is required');
        }
        return FriendRequest.fromJson(map);
      })(),
      friendship: (() {
        final map = _sdkworkAsMap(json['friendship']);
        if (map == null) {
          throw FormatException('SocialFriendRequestAcceptanceResponse.friendship is required');
        }
        return Friendship.fromJson(map);
      })(),
      directChat: (() {
        final map = _sdkworkAsMap(json['directChat']);
        if (map == null) {
          throw FormatException('SocialFriendRequestAcceptanceResponse.directChat is required');
        }
        return DirectChat.fromJson(map);
      })(),
      conversation: (() {
        final map = _sdkworkAsMap(json['conversation']);
        if (map == null) {
          throw FormatException('SocialFriendRequestAcceptanceResponse.conversation is required');
        }
        return CreateConversationResult.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'friendRequest': friendRequest.toJson(),
      'friendship': friendship.toJson(),
      'directChat': directChat.toJson(),
      'conversation': conversation.toJson(),
    };
  }
}

class SocialFriendshipMutationResponse {
  final Friendship friendship;

  SocialFriendshipMutationResponse({
    required this.friendship
  });

  factory SocialFriendshipMutationResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendshipMutationResponse(
      friendship: (() {
        final map = _sdkworkAsMap(json['friendship']);
        if (map == null) {
          throw FormatException('SocialFriendshipMutationResponse.friendship is required');
        }
        return Friendship.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'friendship': friendship.toJson(),
    };
  }
}

class CreateConversationRequest {
  final String? conversationId;
  final String? conversationType;
  final String? kind;
  final String? title;
  final List<String>? memberIds;

  CreateConversationRequest({
    this.conversationId,
    this.conversationType,
    this.kind,
    this.title,
    this.memberIds
  });

  factory CreateConversationRequest.fromJson(Map<String, dynamic> json) {
    return CreateConversationRequest(
      conversationId: json['conversationId']?.toString(),
      conversationType: json['conversationType']?.toString(),
      kind: json['kind']?.toString(),
      title: json['title']?.toString(),
      memberIds: (() {
        final list = _sdkworkAsList(json['memberIds']);
        if (list == null) {
          return null;
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
      'conversationId': conversationId,
      'conversationType': conversationType,
      'kind': kind,
      'title': title,
      'memberIds': memberIds?.map((item) => item).toList(),
    };
  }
}

class CreateAgentDialogRequest {
  final String agentId;
  final String? conversationId;
  final String? title;

  CreateAgentDialogRequest({
    required this.agentId,
    this.conversationId,
    this.title
  });

  factory CreateAgentDialogRequest.fromJson(Map<String, dynamic> json) {
    return CreateAgentDialogRequest(
      agentId: (() {
        final value = json['agentId']?.toString();
        if (value == null) {
          throw FormatException('CreateAgentDialogRequest.agentId is required');
        }
        return value;
      })(),
      conversationId: json['conversationId']?.toString(),
      title: json['title']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'agentId': agentId,
      'conversationId': conversationId,
      'title': title,
    };
  }
}

class BindDirectChatRequest {
  final String? conversationId;
  final String? directChatId;
  final String? leftActorId;
  final String? leftActorKind;
  final String? rightActorId;
  final String? rightActorKind;
  final String? targetUserId;

  BindDirectChatRequest({
    this.conversationId,
    this.directChatId,
    this.leftActorId,
    this.leftActorKind,
    this.rightActorId,
    this.rightActorKind,
    this.targetUserId
  });

  factory BindDirectChatRequest.fromJson(Map<String, dynamic> json) {
    return BindDirectChatRequest(
      conversationId: json['conversationId']?.toString(),
      directChatId: json['directChatId']?.toString(),
      leftActorId: json['leftActorId']?.toString(),
      leftActorKind: json['leftActorKind']?.toString(),
      rightActorId: json['rightActorId']?.toString(),
      rightActorKind: json['rightActorKind']?.toString(),
      targetUserId: json['targetUserId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'directChatId': directChatId,
      'leftActorId': leftActorId,
      'leftActorKind': leftActorKind,
      'rightActorId': rightActorId,
      'rightActorKind': rightActorKind,
      'targetUserId': targetUserId,
    };
  }
}

class CreateConversationResult {
  final String tenantId;
  final String conversationId;
  final String kind;
  final String createdAt;

  CreateConversationResult({
    required this.tenantId,
    required this.conversationId,
    required this.kind,
    required this.createdAt
  });

  factory CreateConversationResult.fromJson(Map<String, dynamic> json) {
    return CreateConversationResult(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('CreateConversationResult.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('CreateConversationResult.conversationId is required');
        }
        return value;
      })(),
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('CreateConversationResult.kind is required');
        }
        return value;
      })(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('CreateConversationResult.createdAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'kind': kind,
      'createdAt': createdAt,
    };
  }
}

class CreateRoomRequest {
  final String conversationId;
  final String roomId;
  final String roomKind;

  CreateRoomRequest({
    required this.conversationId,
    required this.roomId,
    required this.roomKind
  });

  factory CreateRoomRequest.fromJson(Map<String, dynamic> json) {
    return CreateRoomRequest(
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('CreateRoomRequest.conversationId is required');
        }
        return value;
      })(),
      roomId: (() {
        final value = json['roomId']?.toString();
        if (value == null) {
          throw FormatException('CreateRoomRequest.roomId is required');
        }
        return value;
      })(),
      roomKind: (() {
        final value = json['roomKind']?.toString();
        if (value == null) {
          throw FormatException('CreateRoomRequest.roomKind is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'roomId': roomId,
      'roomKind': roomKind,
    };
  }
}

class RoomView {
  final String roomId;
  final String roomKind;
  final String conversationId;
  final int activeMemberCount;
  final int maxMembers;

  RoomView({
    required this.roomId,
    required this.roomKind,
    required this.conversationId,
    required this.activeMemberCount,
    required this.maxMembers
  });

  factory RoomView.fromJson(Map<String, dynamic> json) {
    return RoomView(
      roomId: (() {
        final value = json['roomId']?.toString();
        if (value == null) {
          throw FormatException('RoomView.roomId is required');
        }
        return value;
      })(),
      roomKind: (() {
        final value = json['roomKind']?.toString();
        if (value == null) {
          throw FormatException('RoomView.roomKind is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('RoomView.conversationId is required');
        }
        return value;
      })(),
      activeMemberCount: (() {
        final value = json['activeMemberCount'];
        if (value is! int) {
          throw FormatException('RoomView.activeMemberCount is required');
        }
        return value;
      })(),
      maxMembers: (() {
        final value = json['maxMembers'];
        if (value is! int) {
          throw FormatException('RoomView.maxMembers is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'roomId': roomId,
      'roomKind': roomKind,
      'conversationId': conversationId,
      'activeMemberCount': activeMemberCount,
      'maxMembers': maxMembers,
    };
  }
}

class EnterRoomResponse {
  final ConversationMember member;

  EnterRoomResponse({
    required this.member
  });

  factory EnterRoomResponse.fromJson(Map<String, dynamic> json) {
    return EnterRoomResponse(
      member: (() {
        final map = _sdkworkAsMap(json['member']);
        if (map == null) {
          throw FormatException('EnterRoomResponse.member is required');
        }
        return ConversationMember.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'member': member.toJson(),
    };
  }
}

class AddConversationMemberRequest {
  final String principalId;
  final String principalKind;
  final String role;
  final Map<String, dynamic>? attributes;

  AddConversationMemberRequest({
    required this.principalId,
    required this.principalKind,
    required this.role,
    this.attributes
  });

  factory AddConversationMemberRequest.fromJson(Map<String, dynamic> json) {
    return AddConversationMemberRequest(
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('AddConversationMemberRequest.principalId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('AddConversationMemberRequest.principalKind is required');
        }
        return value;
      })(),
      role: (() {
        final value = json['role']?.toString();
        if (value == null) {
          throw FormatException('AddConversationMemberRequest.role is required');
        }
        return value;
      })(),
      attributes: _sdkworkAsMap(json['attributes'])
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'principalId': principalId,
      'principalKind': principalKind,
      'role': role,
      'attributes': attributes,
    };
  }
}

class RemoveConversationMemberRequest {
  final String memberId;

  RemoveConversationMemberRequest({
    required this.memberId
  });

  factory RemoveConversationMemberRequest.fromJson(Map<String, dynamic> json) {
    return RemoveConversationMemberRequest(
      memberId: (() {
        final value = json['memberId']?.toString();
        if (value == null) {
          throw FormatException('RemoveConversationMemberRequest.memberId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'memberId': memberId,
    };
  }
}

class TransferConversationOwnerRequest {
  final String memberId;

  TransferConversationOwnerRequest({
    required this.memberId
  });

  factory TransferConversationOwnerRequest.fromJson(Map<String, dynamic> json) {
    return TransferConversationOwnerRequest(
      memberId: (() {
        final value = json['memberId']?.toString();
        if (value == null) {
          throw FormatException('TransferConversationOwnerRequest.memberId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'memberId': memberId,
    };
  }
}

class ChangeConversationMemberRoleRequest {
  final String memberId;
  final String role;

  ChangeConversationMemberRoleRequest({
    required this.memberId,
    required this.role
  });

  factory ChangeConversationMemberRoleRequest.fromJson(Map<String, dynamic> json) {
    return ChangeConversationMemberRoleRequest(
      memberId: (() {
        final value = json['memberId']?.toString();
        if (value == null) {
          throw FormatException('ChangeConversationMemberRoleRequest.memberId is required');
        }
        return value;
      })(),
      role: (() {
        final value = json['role']?.toString();
        if (value == null) {
          throw FormatException('ChangeConversationMemberRoleRequest.role is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'memberId': memberId,
      'role': role,
    };
  }
}

class ConversationMember {
  final String tenantId;
  final String conversationId;
  final String memberId;
  final String principalId;
  final String principalKind;
  final String role;
  final String state;
  final String joinedAt;

  ConversationMember({
    required this.tenantId,
    required this.conversationId,
    required this.memberId,
    required this.principalId,
    required this.principalKind,
    required this.role,
    required this.state,
    required this.joinedAt
  });

  factory ConversationMember.fromJson(Map<String, dynamic> json) {
    return ConversationMember(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.conversationId is required');
        }
        return value;
      })(),
      memberId: (() {
        final value = json['memberId']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.memberId is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.principalId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.principalKind is required');
        }
        return value;
      })(),
      role: (() {
        final value = json['role']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.role is required');
        }
        return value;
      })(),
      state: (() {
        final value = json['state']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.state is required');
        }
        return value;
      })(),
      joinedAt: (() {
        final value = json['joinedAt']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.joinedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'memberId': memberId,
      'principalId': principalId,
      'principalKind': principalKind,
      'role': role,
      'state': state,
      'joinedAt': joinedAt,
    };
  }
}

class ListMembersResponse {
  final List<ConversationMember> items;
  final String? nextCursor;
  final bool hasMore;

  ListMembersResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory ListMembersResponse.fromJson(Map<String, dynamic> json) {
    return ListMembersResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('ListMembersResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ConversationMember.fromJson(map);
      })())
            .whereType<ConversationMember>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('ListMembersResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class MemberDirectoryResponse {
  final List<ConversationMember> items;

  MemberDirectoryResponse({
    required this.items
  });

  factory MemberDirectoryResponse.fromJson(Map<String, dynamic> json) {
    return MemberDirectoryResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('MemberDirectoryResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ConversationMember.fromJson(map);
      })())
            .whereType<ConversationMember>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
    };
  }
}

class ReadCursorView {
  final String tenantId;
  final String conversationId;
  final String principalId;
  final int readSeq;
  final String updatedAt;

  ReadCursorView({
    required this.tenantId,
    required this.conversationId,
    required this.principalId,
    required this.readSeq,
    required this.updatedAt
  });

  factory ReadCursorView.fromJson(Map<String, dynamic> json) {
    return ReadCursorView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ReadCursorView.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('ReadCursorView.conversationId is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('ReadCursorView.principalId is required');
        }
        return value;
      })(),
      readSeq: (() {
        final value = json['readSeq'];
        if (value is! int) {
          throw FormatException('ReadCursorView.readSeq is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('ReadCursorView.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'principalId': principalId,
      'readSeq': readSeq,
      'updatedAt': updatedAt,
    };
  }
}

class UpdateReadCursorRequest {
  final int readSeq;

  UpdateReadCursorRequest({
    required this.readSeq
  });

  factory UpdateReadCursorRequest.fromJson(Map<String, dynamic> json) {
    return UpdateReadCursorRequest(
      readSeq: (() {
        final value = json['readSeq'];
        if (value is! int) {
          throw FormatException('UpdateReadCursorRequest.readSeq is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'readSeq': readSeq,
    };
  }
}

class PinnedMessagesResponse {
  final List<MessageInteractionSummaryView> items;

  PinnedMessagesResponse({
    required this.items
  });

  factory PinnedMessagesResponse.fromJson(Map<String, dynamic> json) {
    return PinnedMessagesResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('PinnedMessagesResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MessageInteractionSummaryView.fromJson(map);
      })())
            .whereType<MessageInteractionSummaryView>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
    };
  }
}

class StreamView {
  final String tenantId;
  final String streamId;
  final String state;
  final String openedAt;

  StreamView({
    required this.tenantId,
    required this.streamId,
    required this.state,
    required this.openedAt
  });

  factory StreamView.fromJson(Map<String, dynamic> json) {
    return StreamView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('StreamView.tenantId is required');
        }
        return value;
      })(),
      streamId: (() {
        final value = json['streamId']?.toString();
        if (value == null) {
          throw FormatException('StreamView.streamId is required');
        }
        return value;
      })(),
      state: (() {
        final value = json['state']?.toString();
        if (value == null) {
          throw FormatException('StreamView.state is required');
        }
        return value;
      })(),
      openedAt: (() {
        final value = json['openedAt']?.toString();
        if (value == null) {
          throw FormatException('StreamView.openedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'streamId': streamId,
      'state': state,
      'openedAt': openedAt,
    };
  }
}

class OpenStreamRequest {
  final String streamType;
  final String? conversationId;

  OpenStreamRequest({
    required this.streamType,
    this.conversationId
  });

  factory OpenStreamRequest.fromJson(Map<String, dynamic> json) {
    return OpenStreamRequest(
      streamType: (() {
        final value = json['streamType']?.toString();
        if (value == null) {
          throw FormatException('OpenStreamRequest.streamType is required');
        }
        return value;
      })(),
      conversationId: json['conversationId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'streamType': streamType,
      'conversationId': conversationId,
    };
  }
}

class StreamFrameView {
  final String streamId;
  final int frameSeq;
  final String payload;
  final String createdAt;

  StreamFrameView({
    required this.streamId,
    required this.frameSeq,
    required this.payload,
    required this.createdAt
  });

  factory StreamFrameView.fromJson(Map<String, dynamic> json) {
    return StreamFrameView(
      streamId: (() {
        final value = json['streamId']?.toString();
        if (value == null) {
          throw FormatException('StreamFrameView.streamId is required');
        }
        return value;
      })(),
      frameSeq: (() {
        final value = json['frameSeq'];
        if (value is! int) {
          throw FormatException('StreamFrameView.frameSeq is required');
        }
        return value;
      })(),
      payload: (() {
        final value = json['payload']?.toString();
        if (value == null) {
          throw FormatException('StreamFrameView.payload is required');
        }
        return value;
      })(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('StreamFrameView.createdAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'streamId': streamId,
      'frameSeq': frameSeq,
      'payload': payload,
      'createdAt': createdAt,
    };
  }
}

class StreamFramesResponse {
  final List<StreamFrameView> items;
  final String? nextCursor;
  final bool hasMore;

  StreamFramesResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory StreamFramesResponse.fromJson(Map<String, dynamic> json) {
    return StreamFramesResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('StreamFramesResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : StreamFrameView.fromJson(map);
      })())
            .whereType<StreamFrameView>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('StreamFramesResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class AppendStreamFrameRequest {
  final String payload;

  AppendStreamFrameRequest({
    required this.payload
  });

  factory AppendStreamFrameRequest.fromJson(Map<String, dynamic> json) {
    return AppendStreamFrameRequest(
      payload: (() {
        final value = json['payload']?.toString();
        if (value == null) {
          throw FormatException('AppendStreamFrameRequest.payload is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'payload': payload,
    };
  }
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

class SpaceCreateRequest {
  final String spaceName;
  final String spaceType;
  final String? description;

  SpaceCreateRequest({
    required this.spaceName,
    required this.spaceType,
    this.description
  });

  factory SpaceCreateRequest.fromJson(Map<String, dynamic> json) {
    return SpaceCreateRequest(
      spaceName: (() {
        final value = json['spaceName']?.toString();
        if (value == null) {
          throw FormatException('SpaceCreateRequest.spaceName is required');
        }
        return value;
      })(),
      spaceType: (() {
        final value = json['spaceType']?.toString();
        if (value == null) {
          throw FormatException('SpaceCreateRequest.spaceType is required');
        }
        return value;
      })(),
      description: json['description']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'spaceName': spaceName,
      'spaceType': spaceType,
      'description': description,
    };
  }
}

class SpaceUpdateRequest {
  final String? spaceName;
  final String? description;

  SpaceUpdateRequest({
    this.spaceName,
    this.description
  });

  factory SpaceUpdateRequest.fromJson(Map<String, dynamic> json) {
    return SpaceUpdateRequest(
      spaceName: json['spaceName']?.toString(),
      description: json['description']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'spaceName': spaceName,
      'description': description,
    };
  }
}

class SpaceView {
  final String spaceId;
  final String spaceName;
  final String spaceType;
  final String ownerUserId;
  final String createdAt;

  SpaceView({
    required this.spaceId,
    required this.spaceName,
    required this.spaceType,
    required this.ownerUserId,
    required this.createdAt
  });

  factory SpaceView.fromJson(Map<String, dynamic> json) {
    return SpaceView(
      spaceId: (() {
        final value = json['spaceId']?.toString();
        if (value == null) {
          throw FormatException('SpaceView.spaceId is required');
        }
        return value;
      })(),
      spaceName: (() {
        final value = json['spaceName']?.toString();
        if (value == null) {
          throw FormatException('SpaceView.spaceName is required');
        }
        return value;
      })(),
      spaceType: (() {
        final value = json['spaceType']?.toString();
        if (value == null) {
          throw FormatException('SpaceView.spaceType is required');
        }
        return value;
      })(),
      ownerUserId: (() {
        final value = json['ownerUserId']?.toString();
        if (value == null) {
          throw FormatException('SpaceView.ownerUserId is required');
        }
        return value;
      })(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('SpaceView.createdAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'spaceId': spaceId,
      'spaceName': spaceName,
      'spaceType': spaceType,
      'ownerUserId': ownerUserId,
      'createdAt': createdAt,
    };
  }
}

class SpaceListResponse {
  final List<SpaceView> items;

  SpaceListResponse({
    required this.items
  });

  factory SpaceListResponse.fromJson(Map<String, dynamic> json) {
    return SpaceListResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SpaceListResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : SpaceView.fromJson(map);
      })())
            .whereType<SpaceView>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
    };
  }
}

class SpaceMemberCreateRequest {
  final String userId;
  final String? role;

  SpaceMemberCreateRequest({
    required this.userId,
    this.role
  });

  factory SpaceMemberCreateRequest.fromJson(Map<String, dynamic> json) {
    return SpaceMemberCreateRequest(
      userId: (() {
        final value = json['userId']?.toString();
        if (value == null) {
          throw FormatException('SpaceMemberCreateRequest.userId is required');
        }
        return value;
      })(),
      role: json['role']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'userId': userId,
      'role': role,
    };
  }
}

class SpaceMemberUpdateRequest {
  final String? role;

  SpaceMemberUpdateRequest({
    this.role
  });

  factory SpaceMemberUpdateRequest.fromJson(Map<String, dynamic> json) {
    return SpaceMemberUpdateRequest(
      role: json['role']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'role': role,
    };
  }
}

class SpaceMemberView {
  final String userId;
  final String role;

  SpaceMemberView({
    required this.userId,
    required this.role
  });

  factory SpaceMemberView.fromJson(Map<String, dynamic> json) {
    return SpaceMemberView(
      userId: (() {
        final value = json['userId']?.toString();
        if (value == null) {
          throw FormatException('SpaceMemberView.userId is required');
        }
        return value;
      })(),
      role: (() {
        final value = json['role']?.toString();
        if (value == null) {
          throw FormatException('SpaceMemberView.role is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'userId': userId,
      'role': role,
    };
  }
}

class SpaceMemberListResponse {
  final List<SpaceMemberView> items;

  SpaceMemberListResponse({
    required this.items
  });

  factory SpaceMemberListResponse.fromJson(Map<String, dynamic> json) {
    return SpaceMemberListResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SpaceMemberListResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : SpaceMemberView.fromJson(map);
      })())
            .whereType<SpaceMemberView>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
    };
  }
}

class SpaceGroupCreateRequest {
  final String groupName;
  final String? description;

  SpaceGroupCreateRequest({
    required this.groupName,
    this.description
  });

  factory SpaceGroupCreateRequest.fromJson(Map<String, dynamic> json) {
    return SpaceGroupCreateRequest(
      groupName: (() {
        final value = json['groupName']?.toString();
        if (value == null) {
          throw FormatException('SpaceGroupCreateRequest.groupName is required');
        }
        return value;
      })(),
      description: json['description']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'groupName': groupName,
      'description': description,
    };
  }
}

class SpaceGroupUpdateRequest {
  final String? groupName;
  final String? description;

  SpaceGroupUpdateRequest({
    this.groupName,
    this.description
  });

  factory SpaceGroupUpdateRequest.fromJson(Map<String, dynamic> json) {
    return SpaceGroupUpdateRequest(
      groupName: json['groupName']?.toString(),
      description: json['description']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'groupName': groupName,
      'description': description,
    };
  }
}

class SpaceGroupView {
  final String groupId;
  final String groupName;

  SpaceGroupView({
    required this.groupId,
    required this.groupName
  });

  factory SpaceGroupView.fromJson(Map<String, dynamic> json) {
    return SpaceGroupView(
      groupId: (() {
        final value = json['groupId']?.toString();
        if (value == null) {
          throw FormatException('SpaceGroupView.groupId is required');
        }
        return value;
      })(),
      groupName: (() {
        final value = json['groupName']?.toString();
        if (value == null) {
          throw FormatException('SpaceGroupView.groupName is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'groupId': groupId,
      'groupName': groupName,
    };
  }
}

class SpaceGroupListResponse {
  final List<SpaceGroupView> items;

  SpaceGroupListResponse({
    required this.items
  });

  factory SpaceGroupListResponse.fromJson(Map<String, dynamic> json) {
    return SpaceGroupListResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SpaceGroupListResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : SpaceGroupView.fromJson(map);
      })())
            .whereType<SpaceGroupView>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
    };
  }
}

class SpaceGroupMemberCreateRequest {
  final String userId;
  final String? role;
  final String? nickname;

  SpaceGroupMemberCreateRequest({
    required this.userId,
    this.role,
    this.nickname
  });

  factory SpaceGroupMemberCreateRequest.fromJson(Map<String, dynamic> json) {
    return SpaceGroupMemberCreateRequest(
      userId: (() {
        final value = json['userId']?.toString();
        if (value == null) {
          throw FormatException('SpaceGroupMemberCreateRequest.userId is required');
        }
        return value;
      })(),
      role: json['role']?.toString(),
      nickname: json['nickname']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'userId': userId,
      'role': role,
      'nickname': nickname,
    };
  }
}

class SpaceGroupMemberUpdateRequest {
  final String? role;
  final String? nickname;
  final String? muteUntil;

  SpaceGroupMemberUpdateRequest({
    this.role,
    this.nickname,
    this.muteUntil
  });

  factory SpaceGroupMemberUpdateRequest.fromJson(Map<String, dynamic> json) {
    return SpaceGroupMemberUpdateRequest(
      role: json['role']?.toString(),
      nickname: json['nickname']?.toString(),
      muteUntil: json['muteUntil']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'role': role,
      'nickname': nickname,
      'muteUntil': muteUntil,
    };
  }
}

class SpaceGroupMemberView {
  final String userId;
  final String role;
  final String? nickname;
  final String? muteUntil;
  final String joinedAt;

  SpaceGroupMemberView({
    required this.userId,
    required this.role,
    this.nickname,
    this.muteUntil,
    required this.joinedAt
  });

  factory SpaceGroupMemberView.fromJson(Map<String, dynamic> json) {
    return SpaceGroupMemberView(
      userId: (() {
        final value = json['userId']?.toString();
        if (value == null) {
          throw FormatException('SpaceGroupMemberView.userId is required');
        }
        return value;
      })(),
      role: (() {
        final value = json['role']?.toString();
        if (value == null) {
          throw FormatException('SpaceGroupMemberView.role is required');
        }
        return value;
      })(),
      nickname: json['nickname']?.toString(),
      muteUntil: json['muteUntil']?.toString(),
      joinedAt: (() {
        final value = json['joinedAt']?.toString();
        if (value == null) {
          throw FormatException('SpaceGroupMemberView.joinedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'userId': userId,
      'role': role,
      'nickname': nickname,
      'muteUntil': muteUntil,
      'joinedAt': joinedAt,
    };
  }
}

class SpaceGroupMemberListResponse {
  final List<SpaceGroupMemberView> items;

  SpaceGroupMemberListResponse({
    required this.items
  });

  factory SpaceGroupMemberListResponse.fromJson(Map<String, dynamic> json) {
    return SpaceGroupMemberListResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SpaceGroupMemberListResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : SpaceGroupMemberView.fromJson(map);
      })())
            .whereType<SpaceGroupMemberView>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
    };
  }
}

class SpaceChannelCreateRequest {
  final String channelName;
  final String channelType;

  SpaceChannelCreateRequest({
    required this.channelName,
    required this.channelType
  });

  factory SpaceChannelCreateRequest.fromJson(Map<String, dynamic> json) {
    return SpaceChannelCreateRequest(
      channelName: (() {
        final value = json['channelName']?.toString();
        if (value == null) {
          throw FormatException('SpaceChannelCreateRequest.channelName is required');
        }
        return value;
      })(),
      channelType: (() {
        final value = json['channelType']?.toString();
        if (value == null) {
          throw FormatException('SpaceChannelCreateRequest.channelType is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'channelName': channelName,
      'channelType': channelType,
    };
  }
}

class SpaceChannelUpdateRequest {
  final String? channelName;

  SpaceChannelUpdateRequest({
    this.channelName
  });

  factory SpaceChannelUpdateRequest.fromJson(Map<String, dynamic> json) {
    return SpaceChannelUpdateRequest(
      channelName: json['channelName']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'channelName': channelName,
    };
  }
}

class SpaceChannelView {
  final String channelId;
  final String channelName;
  final String channelType;

  SpaceChannelView({
    required this.channelId,
    required this.channelName,
    required this.channelType
  });

  factory SpaceChannelView.fromJson(Map<String, dynamic> json) {
    return SpaceChannelView(
      channelId: (() {
        final value = json['channelId']?.toString();
        if (value == null) {
          throw FormatException('SpaceChannelView.channelId is required');
        }
        return value;
      })(),
      channelName: (() {
        final value = json['channelName']?.toString();
        if (value == null) {
          throw FormatException('SpaceChannelView.channelName is required');
        }
        return value;
      })(),
      channelType: (() {
        final value = json['channelType']?.toString();
        if (value == null) {
          throw FormatException('SpaceChannelView.channelType is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'channelId': channelId,
      'channelName': channelName,
      'channelType': channelType,
    };
  }
}

class SpaceChannelListResponse {
  final List<SpaceChannelView> items;

  SpaceChannelListResponse({
    required this.items
  });

  factory SpaceChannelListResponse.fromJson(Map<String, dynamic> json) {
    return SpaceChannelListResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SpaceChannelListResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : SpaceChannelView.fromJson(map);
      })())
            .whereType<SpaceChannelView>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
    };
  }
}

class SpaceChannelAccessRuleCreateRequest {
  final String ruleType;
  final String? principalKind;
  final String? principalId;
  final String permission;

  SpaceChannelAccessRuleCreateRequest({
    required this.ruleType,
    this.principalKind,
    this.principalId,
    required this.permission
  });

  factory SpaceChannelAccessRuleCreateRequest.fromJson(Map<String, dynamic> json) {
    return SpaceChannelAccessRuleCreateRequest(
      ruleType: (() {
        final value = json['ruleType']?.toString();
        if (value == null) {
          throw FormatException('SpaceChannelAccessRuleCreateRequest.ruleType is required');
        }
        return value;
      })(),
      principalKind: json['principalKind']?.toString(),
      principalId: json['principalId']?.toString(),
      permission: (() {
        final value = json['permission']?.toString();
        if (value == null) {
          throw FormatException('SpaceChannelAccessRuleCreateRequest.permission is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'ruleType': ruleType,
      'principalKind': principalKind,
      'principalId': principalId,
      'permission': permission,
    };
  }
}

class SpaceChannelAccessRuleView {
  final String ruleId;
  final String channelId;
  final String ruleType;
  final String? principalKind;
  final String? principalId;
  final String permission;
  final String createdAt;

  SpaceChannelAccessRuleView({
    required this.ruleId,
    required this.channelId,
    required this.ruleType,
    this.principalKind,
    this.principalId,
    required this.permission,
    required this.createdAt
  });

  factory SpaceChannelAccessRuleView.fromJson(Map<String, dynamic> json) {
    return SpaceChannelAccessRuleView(
      ruleId: (() {
        final value = json['ruleId']?.toString();
        if (value == null) {
          throw FormatException('SpaceChannelAccessRuleView.ruleId is required');
        }
        return value;
      })(),
      channelId: (() {
        final value = json['channelId']?.toString();
        if (value == null) {
          throw FormatException('SpaceChannelAccessRuleView.channelId is required');
        }
        return value;
      })(),
      ruleType: (() {
        final value = json['ruleType']?.toString();
        if (value == null) {
          throw FormatException('SpaceChannelAccessRuleView.ruleType is required');
        }
        return value;
      })(),
      principalKind: json['principalKind']?.toString(),
      principalId: json['principalId']?.toString(),
      permission: (() {
        final value = json['permission']?.toString();
        if (value == null) {
          throw FormatException('SpaceChannelAccessRuleView.permission is required');
        }
        return value;
      })(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('SpaceChannelAccessRuleView.createdAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'ruleId': ruleId,
      'channelId': channelId,
      'ruleType': ruleType,
      'principalKind': principalKind,
      'principalId': principalId,
      'permission': permission,
      'createdAt': createdAt,
    };
  }
}

class SpaceChannelAccessRuleListResponse {
  final List<SpaceChannelAccessRuleView> items;

  SpaceChannelAccessRuleListResponse({
    required this.items
  });

  factory SpaceChannelAccessRuleListResponse.fromJson(Map<String, dynamic> json) {
    return SpaceChannelAccessRuleListResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SpaceChannelAccessRuleListResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : SpaceChannelAccessRuleView.fromJson(map);
      })())
            .whereType<SpaceChannelAccessRuleView>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
    };
  }
}

class SpaceInviteCreateRequest {
  final int? maxUses;

  SpaceInviteCreateRequest({
    this.maxUses
  });

  factory SpaceInviteCreateRequest.fromJson(Map<String, dynamic> json) {
    return SpaceInviteCreateRequest(
      maxUses: json['maxUses'] is int ? json['maxUses'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'maxUses': maxUses,
    };
  }
}

class SpaceInviteView {
  final String inviteCode;
  final String spaceId;

  SpaceInviteView({
    required this.inviteCode,
    required this.spaceId
  });

  factory SpaceInviteView.fromJson(Map<String, dynamic> json) {
    return SpaceInviteView(
      inviteCode: (() {
        final value = json['inviteCode']?.toString();
        if (value == null) {
          throw FormatException('SpaceInviteView.inviteCode is required');
        }
        return value;
      })(),
      spaceId: (() {
        final value = json['spaceId']?.toString();
        if (value == null) {
          throw FormatException('SpaceInviteView.spaceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'inviteCode': inviteCode,
      'spaceId': spaceId,
    };
  }
}

class SpaceInviteListResponse {
  final List<SpaceInviteView> items;

  SpaceInviteListResponse({
    required this.items
  });

  factory SpaceInviteListResponse.fromJson(Map<String, dynamic> json) {
    return SpaceInviteListResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SpaceInviteListResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : SpaceInviteView.fromJson(map);
      })())
            .whereType<SpaceInviteView>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
    };
  }
}

class SpaceBanCreateRequest {
  final String userId;
  final String? reason;

  SpaceBanCreateRequest({
    required this.userId,
    this.reason
  });

  factory SpaceBanCreateRequest.fromJson(Map<String, dynamic> json) {
    return SpaceBanCreateRequest(
      userId: (() {
        final value = json['userId']?.toString();
        if (value == null) {
          throw FormatException('SpaceBanCreateRequest.userId is required');
        }
        return value;
      })(),
      reason: json['reason']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'userId': userId,
      'reason': reason,
    };
  }
}

class SpaceBanView {
  final String userId;
  final String? reason;

  SpaceBanView({
    required this.userId,
    this.reason
  });

  factory SpaceBanView.fromJson(Map<String, dynamic> json) {
    return SpaceBanView(
      userId: (() {
        final value = json['userId']?.toString();
        if (value == null) {
          throw FormatException('SpaceBanView.userId is required');
        }
        return value;
      })(),
      reason: json['reason']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'userId': userId,
      'reason': reason,
    };
  }
}

class SpaceBanListResponse {
  final List<SpaceBanView> items;

  SpaceBanListResponse({
    required this.items
  });

  factory SpaceBanListResponse.fromJson(Map<String, dynamic> json) {
    return SpaceBanListResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SpaceBanListResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : SpaceBanView.fromJson(map);
      })())
            .whereType<SpaceBanView>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
    };
  }
}

class TextContentPart implements ContentPart {
  final String kind;
  final String text;

  TextContentPart({
    required this.kind,
    required this.text
  });

  factory TextContentPart.fromJson(Map<String, dynamic> json) {
    return TextContentPart(
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('TextContentPart.kind is required');
        }
        return value;
      })(),
      text: (() {
        final value = json['text']?.toString();
        if (value == null) {
          throw FormatException('TextContentPart.text is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'kind': kind,
      'text': text,
    };
  }
}

class DataContentPart implements ContentPart {
  final String kind;
  final String schemaRef;
  final String encoding;
  final String payload;

  DataContentPart({
    required this.kind,
    required this.schemaRef,
    required this.encoding,
    required this.payload
  });

  factory DataContentPart.fromJson(Map<String, dynamic> json) {
    return DataContentPart(
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('DataContentPart.kind is required');
        }
        return value;
      })(),
      schemaRef: (() {
        final value = json['schemaRef']?.toString();
        if (value == null) {
          throw FormatException('DataContentPart.schemaRef is required');
        }
        return value;
      })(),
      encoding: (() {
        final value = json['encoding']?.toString();
        if (value == null) {
          throw FormatException('DataContentPart.encoding is required');
        }
        return value;
      })(),
      payload: (() {
        final value = json['payload']?.toString();
        if (value == null) {
          throw FormatException('DataContentPart.payload is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'kind': kind,
      'schemaRef': schemaRef,
      'encoding': encoding,
      'payload': payload,
    };
  }
}

class MediaContentPart implements ContentPart {
  final String kind;
  final DriveReference drive;
  final MediaResource resource;
  final String? mediaRole;

  MediaContentPart({
    required this.kind,
    required this.drive,
    required this.resource,
    this.mediaRole
  });

  factory MediaContentPart.fromJson(Map<String, dynamic> json) {
    return MediaContentPart(
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('MediaContentPart.kind is required');
        }
        return value;
      })(),
      drive: (() {
        final map = _sdkworkAsMap(json['drive']);
        if (map == null) {
          throw FormatException('MediaContentPart.drive is required');
        }
        return DriveReference.fromJson(map);
      })(),
      resource: (() {
        final map = _sdkworkAsMap(json['resource']);
        if (map == null) {
          throw FormatException('MediaContentPart.resource is required');
        }
        return MediaResource.fromJson(map);
      })(),
      mediaRole: json['mediaRole']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'kind': kind,
      'drive': drive.toJson(),
      'resource': resource.toJson(),
      'mediaRole': mediaRole,
    };
  }
}

class SignalContentPart implements ContentPart {
  final String kind;
  final String signalType;
  final String? schemaRef;
  final String payload;

  SignalContentPart({
    required this.kind,
    required this.signalType,
    this.schemaRef,
    required this.payload
  });

  factory SignalContentPart.fromJson(Map<String, dynamic> json) {
    return SignalContentPart(
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('SignalContentPart.kind is required');
        }
        return value;
      })(),
      signalType: (() {
        final value = json['signalType']?.toString();
        if (value == null) {
          throw FormatException('SignalContentPart.signalType is required');
        }
        return value;
      })(),
      schemaRef: json['schemaRef']?.toString(),
      payload: (() {
        final value = json['payload']?.toString();
        if (value == null) {
          throw FormatException('SignalContentPart.payload is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'kind': kind,
      'signalType': signalType,
      'schemaRef': schemaRef,
      'payload': payload,
    };
  }
}

class StreamRefContentPart implements ContentPart {
  final String kind;
  final String streamId;
  final String streamType;
  final String state;

  StreamRefContentPart({
    required this.kind,
    required this.streamId,
    required this.streamType,
    required this.state
  });

  factory StreamRefContentPart.fromJson(Map<String, dynamic> json) {
    return StreamRefContentPart(
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('StreamRefContentPart.kind is required');
        }
        return value;
      })(),
      streamId: (() {
        final value = json['streamId']?.toString();
        if (value == null) {
          throw FormatException('StreamRefContentPart.streamId is required');
        }
        return value;
      })(),
      streamType: (() {
        final value = json['streamType']?.toString();
        if (value == null) {
          throw FormatException('StreamRefContentPart.streamType is required');
        }
        return value;
      })(),
      state: (() {
        final value = json['state']?.toString();
        if (value == null) {
          throw FormatException('StreamRefContentPart.state is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'kind': kind,
      'streamId': streamId,
      'streamType': streamType,
      'state': state,
    };
  }
}
