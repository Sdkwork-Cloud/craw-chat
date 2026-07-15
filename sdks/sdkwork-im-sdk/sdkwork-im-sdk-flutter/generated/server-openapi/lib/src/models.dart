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
      case 'mention':
        return MentionContentPart.fromJson(json);
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

class ConversationMessageEntry {
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

  ConversationMessageEntry({
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

  factory ConversationMessageEntry.fromJson(Map<String, dynamic> json) {
    return ConversationMessageEntry(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ConversationMessageEntry.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('ConversationMessageEntry.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('ConversationMessageEntry.messageId is required');
        }
        return value;
      })(),
      messageSeq: (() {
        final value = json['messageSeq'];
        if (value is! int) {
          throw FormatException('ConversationMessageEntry.messageSeq is required');
        }
        return value;
      })(),
      summary: json['summary']?.toString(),
      sender: (() {
        final map = _sdkworkAsMap(json['sender']);
        if (map == null) {
          throw FormatException('ConversationMessageEntry.sender is required');
        }
        return Sender.fromJson(map);
      })(),
      body: (() {
        final map = _sdkworkAsMap(json['body']);
        if (map == null) {
          throw FormatException('ConversationMessageEntry.body is required');
        }
        return MessageBody.fromJson(map);
      })(),
      messageType: (() {
        final value = json['messageType']?.toString();
        if (value == null) {
          throw FormatException('ConversationMessageEntry.messageType is required');
        }
        return value;
      })(),
      deliveryMode: (() {
        final value = json['deliveryMode']?.toString();
        if (value == null) {
          throw FormatException('ConversationMessageEntry.deliveryMode is required');
        }
        return value;
      })(),
      clientMsgId: json['clientMsgId']?.toString(),
      streamSessionId: json['streamSessionId']?.toString(),
      rtcSessionId: json['rtcSessionId']?.toString(),
      occurredAt: (() {
        final value = json['occurredAt']?.toString();
        if (value == null) {
          throw FormatException('ConversationMessageEntry.occurredAt is required');
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

class ConversationMessageListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationMessageListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationMessageListResponse.fromJson(Map<String, dynamic> json) {
    return ConversationMessageListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationMessageListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationMessageListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationMessageListResponse.traceId is required');
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
  final String? summary;
  final Map<String, dynamic>? renderHints;
  final String? idempotencyKey;

  EditMessageRequest({
    this.text,
    this.parts,
    this.replyTo,
    this.summary,
    this.renderHints,
    this.idempotencyKey
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
      })(),
      summary: json['summary']?.toString(),
      renderHints: _sdkworkAsMap(json['renderHints']),
      idempotencyKey: json['idempotencyKey']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'text': text,
      'parts': parts?.map((item) => item.toJson()).toList(),
      'replyTo': replyTo?.toJson(),
      'summary': summary,
      'renderHints': renderHints,
      'idempotencyKey': idempotencyKey,
    };
  }
}

class RecallMessageRequest {
  final String? idempotencyKey;

  RecallMessageRequest({
    this.idempotencyKey
  });

  factory RecallMessageRequest.fromJson(Map<String, dynamic> json) {
    return RecallMessageRequest(
      idempotencyKey: json['idempotencyKey']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'idempotencyKey': idempotencyKey,
    };
  }
}

class PostMessageResult {
  final String messageId;
  final int messageSeq;
  final String eventId;
  final String? requestKey;
  final String deliveryStatus;
  final String? proofVersion;

  PostMessageResult({
    required this.messageId,
    required this.messageSeq,
    required this.eventId,
    this.requestKey,
    required this.deliveryStatus,
    this.proofVersion
  });

  factory PostMessageResult.fromJson(Map<String, dynamic> json) {
    return PostMessageResult(
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('PostMessageResult.messageId is required');
        }
        return value;
      })(),
      messageSeq: (() {
        final value = json['messageSeq'];
        if (value is! int) {
          throw FormatException('PostMessageResult.messageSeq is required');
        }
        return value;
      })(),
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('PostMessageResult.eventId is required');
        }
        return value;
      })(),
      requestKey: json['requestKey']?.toString(),
      deliveryStatus: (() {
        final value = json['deliveryStatus']?.toString();
        if (value == null) {
          throw FormatException('PostMessageResult.deliveryStatus is required');
        }
        return value;
      })(),
      proofVersion: json['proofVersion']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'messageId': messageId,
      'messageSeq': messageSeq,
      'eventId': eventId,
      'requestKey': requestKey,
      'deliveryStatus': deliveryStatus,
      'proofVersion': proofVersion,
    };
  }
}

class MessageMutationResult {
  final String conversationId;
  final String messageId;
  final int messageSeq;
  final String eventId;

  MessageMutationResult({
    required this.conversationId,
    required this.messageId,
    required this.messageSeq,
    required this.eventId
  });

  factory MessageMutationResult.fromJson(Map<String, dynamic> json) {
    return MessageMutationResult(
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('MessageMutationResult.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('MessageMutationResult.messageId is required');
        }
        return value;
      })(),
      messageSeq: (() {
        final value = json['messageSeq'];
        if (value is! int) {
          throw FormatException('MessageMutationResult.messageSeq is required');
        }
        return value;
      })(),
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('MessageMutationResult.eventId is required');
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
      'eventId': eventId,
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

class BlockUserRequest {
  final String blockedUserId;
  final String scope;
  final String? directChatId;
  final String? expiresAt;

  BlockUserRequest({
    required this.blockedUserId,
    required this.scope,
    this.directChatId,
    this.expiresAt
  });

  factory BlockUserRequest.fromJson(Map<String, dynamic> json) {
    return BlockUserRequest(
      blockedUserId: (() {
        final value = json['blockedUserId']?.toString();
        if (value == null) {
          throw FormatException('BlockUserRequest.blockedUserId is required');
        }
        return value;
      })(),
      scope: (() {
        final value = json['scope']?.toString();
        if (value == null) {
          throw FormatException('BlockUserRequest.scope is required');
        }
        return value;
      })(),
      directChatId: json['directChatId']?.toString(),
      expiresAt: json['expiresAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'blockedUserId': blockedUserId,
      'scope': scope,
      'directChatId': directChatId,
      'expiresAt': expiresAt,
    };
  }
}

class UserBlock {
  final String tenantId;
  final String blockId;
  final String blockerUserId;
  final String blockedUserId;
  final String scope;
  final String status;
  final String? directChatId;
  final String? expiresAt;
  final String createdAt;
  final String updatedAt;

  UserBlock({
    required this.tenantId,
    required this.blockId,
    required this.blockerUserId,
    required this.blockedUserId,
    required this.scope,
    required this.status,
    this.directChatId,
    this.expiresAt,
    required this.createdAt,
    required this.updatedAt
  });

  factory UserBlock.fromJson(Map<String, dynamic> json) {
    return UserBlock(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('UserBlock.tenantId is required');
        }
        return value;
      })(),
      blockId: (() {
        final value = json['blockId']?.toString();
        if (value == null) {
          throw FormatException('UserBlock.blockId is required');
        }
        return value;
      })(),
      blockerUserId: (() {
        final value = json['blockerUserId']?.toString();
        if (value == null) {
          throw FormatException('UserBlock.blockerUserId is required');
        }
        return value;
      })(),
      blockedUserId: (() {
        final value = json['blockedUserId']?.toString();
        if (value == null) {
          throw FormatException('UserBlock.blockedUserId is required');
        }
        return value;
      })(),
      scope: (() {
        final value = json['scope']?.toString();
        if (value == null) {
          throw FormatException('UserBlock.scope is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status']?.toString();
        if (value == null) {
          throw FormatException('UserBlock.status is required');
        }
        return value;
      })(),
      directChatId: json['directChatId']?.toString(),
      expiresAt: json['expiresAt']?.toString(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('UserBlock.createdAt is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('UserBlock.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'blockId': blockId,
      'blockerUserId': blockerUserId,
      'blockedUserId': blockedUserId,
      'scope': scope,
      'status': status,
      'directChatId': directChatId,
      'expiresAt': expiresAt,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}

class SocialWritePersistence {
  final bool journalAuthority;
  final String snapshotStatus;

  SocialWritePersistence({
    required this.journalAuthority,
    required this.snapshotStatus
  });

  factory SocialWritePersistence.fromJson(Map<String, dynamic> json) {
    return SocialWritePersistence(
      journalAuthority: (() {
        final value = json['journalAuthority'];
        if (value is! bool) {
          throw FormatException('SocialWritePersistence.journalAuthority is required');
        }
        return value;
      })(),
      snapshotStatus: (() {
        final value = json['snapshotStatus']?.toString();
        if (value == null) {
          throw FormatException('SocialWritePersistence.snapshotStatus is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'journalAuthority': journalAuthority,
      'snapshotStatus': snapshotStatus,
    };
  }
}

class EventActor {
  final String actorId;
  final String actorKind;
  final String? actorSessionId;

  EventActor({
    required this.actorId,
    required this.actorKind,
    this.actorSessionId
  });

  factory EventActor.fromJson(Map<String, dynamic> json) {
    return EventActor(
      actorId: (() {
        final value = json['actorId']?.toString();
        if (value == null) {
          throw FormatException('EventActor.actorId is required');
        }
        return value;
      })(),
      actorKind: (() {
        final value = json['actorKind']?.toString();
        if (value == null) {
          throw FormatException('EventActor.actorKind is required');
        }
        return value;
      })(),
      actorSessionId: json['actorSessionId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'actorId': actorId,
      'actorKind': actorKind,
      'actorSessionId': actorSessionId,
    };
  }
}

class CommitEnvelopeResponse {
  final String eventId;
  final String tenantId;
  final String eventType;
  final int eventVersion;
  final String aggregateType;
  final String aggregateId;
  final String scopeType;
  final String scopeId;
  final String orderingKey;
  final int orderingSeq;
  final String? causationId;
  final String? correlationId;
  final String? idempotencyKey;
  final EventActor actor;
  final String occurredAt;
  final String committedAt;
  final String? payloadSchema;
  final String payload;
  final String retentionClass;
  final String auditClass;

  CommitEnvelopeResponse({
    required this.eventId,
    required this.tenantId,
    required this.eventType,
    required this.eventVersion,
    required this.aggregateType,
    required this.aggregateId,
    required this.scopeType,
    required this.scopeId,
    required this.orderingKey,
    required this.orderingSeq,
    this.causationId,
    this.correlationId,
    this.idempotencyKey,
    required this.actor,
    required this.occurredAt,
    required this.committedAt,
    this.payloadSchema,
    required this.payload,
    required this.retentionClass,
    required this.auditClass
  });

  factory CommitEnvelopeResponse.fromJson(Map<String, dynamic> json) {
    return CommitEnvelopeResponse(
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('CommitEnvelopeResponse.eventId is required');
        }
        return value;
      })(),
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('CommitEnvelopeResponse.tenantId is required');
        }
        return value;
      })(),
      eventType: (() {
        final value = json['eventType']?.toString();
        if (value == null) {
          throw FormatException('CommitEnvelopeResponse.eventType is required');
        }
        return value;
      })(),
      eventVersion: (() {
        final value = json['eventVersion'];
        if (value is! int) {
          throw FormatException('CommitEnvelopeResponse.eventVersion is required');
        }
        return value;
      })(),
      aggregateType: (() {
        final value = json['aggregateType']?.toString();
        if (value == null) {
          throw FormatException('CommitEnvelopeResponse.aggregateType is required');
        }
        return value;
      })(),
      aggregateId: (() {
        final value = json['aggregateId']?.toString();
        if (value == null) {
          throw FormatException('CommitEnvelopeResponse.aggregateId is required');
        }
        return value;
      })(),
      scopeType: (() {
        final value = json['scopeType']?.toString();
        if (value == null) {
          throw FormatException('CommitEnvelopeResponse.scopeType is required');
        }
        return value;
      })(),
      scopeId: (() {
        final value = json['scopeId']?.toString();
        if (value == null) {
          throw FormatException('CommitEnvelopeResponse.scopeId is required');
        }
        return value;
      })(),
      orderingKey: (() {
        final value = json['orderingKey']?.toString();
        if (value == null) {
          throw FormatException('CommitEnvelopeResponse.orderingKey is required');
        }
        return value;
      })(),
      orderingSeq: (() {
        final value = json['orderingSeq'];
        if (value is! int) {
          throw FormatException('CommitEnvelopeResponse.orderingSeq is required');
        }
        return value;
      })(),
      causationId: json['causationId']?.toString(),
      correlationId: json['correlationId']?.toString(),
      idempotencyKey: json['idempotencyKey']?.toString(),
      actor: (() {
        final map = _sdkworkAsMap(json['actor']);
        if (map == null) {
          throw FormatException('CommitEnvelopeResponse.actor is required');
        }
        return EventActor.fromJson(map);
      })(),
      occurredAt: (() {
        final value = json['occurredAt']?.toString();
        if (value == null) {
          throw FormatException('CommitEnvelopeResponse.occurredAt is required');
        }
        return value;
      })(),
      committedAt: (() {
        final value = json['committedAt']?.toString();
        if (value == null) {
          throw FormatException('CommitEnvelopeResponse.committedAt is required');
        }
        return value;
      })(),
      payloadSchema: json['payloadSchema']?.toString(),
      payload: (() {
        final value = json['payload']?.toString();
        if (value == null) {
          throw FormatException('CommitEnvelopeResponse.payload is required');
        }
        return value;
      })(),
      retentionClass: (() {
        final value = json['retentionClass']?.toString();
        if (value == null) {
          throw FormatException('CommitEnvelopeResponse.retentionClass is required');
        }
        return value;
      })(),
      auditClass: (() {
        final value = json['auditClass']?.toString();
        if (value == null) {
          throw FormatException('CommitEnvelopeResponse.auditClass is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'eventId': eventId,
      'tenantId': tenantId,
      'eventType': eventType,
      'eventVersion': eventVersion,
      'aggregateType': aggregateType,
      'aggregateId': aggregateId,
      'scopeType': scopeType,
      'scopeId': scopeId,
      'orderingKey': orderingKey,
      'orderingSeq': orderingSeq,
      'causationId': causationId,
      'correlationId': correlationId,
      'idempotencyKey': idempotencyKey,
      'actor': actor.toJson(),
      'occurredAt': occurredAt,
      'committedAt': committedAt,
      'payloadSchema': payloadSchema,
      'payload': payload,
      'retentionClass': retentionClass,
      'auditClass': auditClass,
    };
  }
}

class OpenApiUserBlockResponse {
  final UserBlock userBlock;
  final CommitEnvelopeResponse latestCommit;
  final SocialWritePersistence persistence;

  OpenApiUserBlockResponse({
    required this.userBlock,
    required this.latestCommit,
    required this.persistence
  });

  factory OpenApiUserBlockResponse.fromJson(Map<String, dynamic> json) {
    return OpenApiUserBlockResponse(
      userBlock: (() {
        final map = _sdkworkAsMap(json['userBlock']);
        if (map == null) {
          throw FormatException('OpenApiUserBlockResponse.userBlock is required');
        }
        return UserBlock.fromJson(map);
      })(),
      latestCommit: (() {
        final map = _sdkworkAsMap(json['latestCommit']);
        if (map == null) {
          throw FormatException('OpenApiUserBlockResponse.latestCommit is required');
        }
        return CommitEnvelopeResponse.fromJson(map);
      })(),
      persistence: (() {
        final map = _sdkworkAsMap(json['persistence']);
        if (map == null) {
          throw FormatException('OpenApiUserBlockResponse.persistence is required');
        }
        return SocialWritePersistence.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'userBlock': userBlock.toJson(),
      'latestCommit': latestCommit.toJson(),
      'persistence': persistence.toJson(),
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
  final String friendRequestId;
  final String requesterUserId;
  final String targetUserId;
  final String status;
  final String? requestMessage;
  final String? expiredAt;
  final String createdAt;
  final String updatedAt;

  FriendRequest({
    required this.tenantId,
    required this.friendRequestId,
    required this.requesterUserId,
    required this.targetUserId,
    required this.status,
    this.requestMessage,
    this.expiredAt,
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
      friendRequestId: (() {
        final value = json['friendRequestId']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.friendRequestId is required');
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
      expiredAt: json['expiredAt']?.toString(),
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
      'friendRequestId': friendRequestId,
      'requesterUserId': requesterUserId,
      'targetUserId': targetUserId,
      'status': status,
      'requestMessage': requestMessage,
      'expiredAt': expiredAt,
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

class SocialFriendRequestAcceptedConversation {
  final String tenantId;
  final String conversationId;
  final String kind;
  final String createdAt;

  SocialFriendRequestAcceptedConversation({
    required this.tenantId,
    required this.conversationId,
    required this.kind,
    required this.createdAt
  });

  factory SocialFriendRequestAcceptedConversation.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestAcceptedConversation(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('SocialFriendRequestAcceptedConversation.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('SocialFriendRequestAcceptedConversation.conversationId is required');
        }
        return value;
      })(),
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('SocialFriendRequestAcceptedConversation.kind is required');
        }
        return value;
      })(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('SocialFriendRequestAcceptedConversation.createdAt is required');
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

class SocialFriendRequestPendingCountResponse {
  final int count;

  SocialFriendRequestPendingCountResponse({
    required this.count
  });

  factory SocialFriendRequestPendingCountResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestPendingCountResponse(
      count: (() {
        final value = json['count'];
        if (value is! int) {
          throw FormatException('SocialFriendRequestPendingCountResponse.count is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'count': count,
    };
  }
}

class SocialFriendRequestAcceptanceResponse {
  final FriendRequest friendRequest;
  final Friendship friendship;
  final DirectChat directChat;
  final SocialFriendRequestAcceptedConversation conversation;

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
        return SocialFriendRequestAcceptedConversation.fromJson(map);
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
  final String conversationType;
  final String? groupName;
  final String? clientRequestKey;
  final bool? initializeKnowledgebase;
  final List<String>? memberUserIds;
  final List<ConversationAgentAssignment>? agentAssignments;
  final String? policyVersion;
  final List<String>? capabilityFlags;
  final String? historyVisibility;
  final String? retentionPolicyRef;

  CreateConversationRequest({
    this.conversationId,
    required this.conversationType,
    this.groupName,
    this.clientRequestKey,
    this.initializeKnowledgebase,
    this.memberUserIds,
    this.agentAssignments,
    this.policyVersion,
    this.capabilityFlags,
    this.historyVisibility,
    this.retentionPolicyRef
  });

  factory CreateConversationRequest.fromJson(Map<String, dynamic> json) {
    return CreateConversationRequest(
      conversationId: json['conversationId']?.toString(),
      conversationType: (() {
        final value = json['conversationType']?.toString();
        if (value == null) {
          throw FormatException('CreateConversationRequest.conversationType is required');
        }
        return value;
      })(),
      groupName: json['groupName']?.toString(),
      clientRequestKey: json['clientRequestKey']?.toString(),
      initializeKnowledgebase: json['initializeKnowledgebase'] is bool ? json['initializeKnowledgebase'] : null,
      memberUserIds: (() {
        final list = _sdkworkAsList(json['memberUserIds']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      agentAssignments: (() {
        final list = _sdkworkAsList(json['agentAssignments']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ConversationAgentAssignment.fromJson(map);
      })())
            .whereType<ConversationAgentAssignment>()
            .toList();
      })(),
      policyVersion: json['policyVersion']?.toString(),
      capabilityFlags: (() {
        final list = _sdkworkAsList(json['capabilityFlags']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      historyVisibility: json['historyVisibility']?.toString(),
      retentionPolicyRef: json['retentionPolicyRef']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'conversationType': conversationType,
      'groupName': groupName,
      'clientRequestKey': clientRequestKey,
      'initializeKnowledgebase': initializeKnowledgebase,
      'memberUserIds': memberUserIds?.map((item) => item).toList(),
      'agentAssignments': agentAssignments?.map((item) => item.toJson()).toList(),
      'policyVersion': policyVersion,
      'capabilityFlags': capabilityFlags?.map((item) => item).toList(),
      'historyVisibility': historyVisibility,
      'retentionPolicyRef': retentionPolicyRef,
    };
  }
}

class ConversationAgentAssignment {
  final String agentId;
  final String? revisionId;

  ConversationAgentAssignment({
    required this.agentId,
    this.revisionId
  });

  factory ConversationAgentAssignment.fromJson(Map<String, dynamic> json) {
    return ConversationAgentAssignment(
      agentId: (() {
        final value = json['agentId']?.toString();
        if (value == null) {
          throw FormatException('ConversationAgentAssignment.agentId is required');
        }
        return value;
      })(),
      revisionId: json['revisionId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'agentId': agentId,
      'revisionId': revisionId,
    };
  }
}

class ConversationAgentAssignments {
  final int generation;
  final String source;
  final List<ConversationAgentAssignment> agents;

  ConversationAgentAssignments({
    required this.generation,
    required this.source,
    required this.agents
  });

  factory ConversationAgentAssignments.fromJson(Map<String, dynamic> json) {
    return ConversationAgentAssignments(
      generation: (() {
        final value = json['generation'];
        if (value is! int) {
          throw FormatException('ConversationAgentAssignments.generation is required');
        }
        return value;
      })(),
      source: (() {
        final value = json['source']?.toString();
        if (value == null) {
          throw FormatException('ConversationAgentAssignments.source is required');
        }
        return value;
      })(),
      agents: (() {
        final list = _sdkworkAsList(json['agents']);
        if (list == null) {
          throw FormatException('ConversationAgentAssignments.agents is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ConversationAgentAssignment.fromJson(map);
      })())
            .whereType<ConversationAgentAssignment>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'generation': generation,
      'source': source,
      'agents': agents.map((item) => item.toJson()).toList(),
    };
  }
}

class UpdateConversationAgentsRequest {
  final int expectedGeneration;
  final List<ConversationAgentAssignment> agentAssignments;

  UpdateConversationAgentsRequest({
    required this.expectedGeneration,
    required this.agentAssignments
  });

  factory UpdateConversationAgentsRequest.fromJson(Map<String, dynamic> json) {
    return UpdateConversationAgentsRequest(
      expectedGeneration: (() {
        final value = json['expectedGeneration'];
        if (value is! int) {
          throw FormatException('UpdateConversationAgentsRequest.expectedGeneration is required');
        }
        return value;
      })(),
      agentAssignments: (() {
        final list = _sdkworkAsList(json['agentAssignments']);
        if (list == null) {
          throw FormatException('UpdateConversationAgentsRequest.agentAssignments is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ConversationAgentAssignment.fromJson(map);
      })())
            .whereType<ConversationAgentAssignment>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'expectedGeneration': expectedGeneration,
      'agentAssignments': agentAssignments.map((item) => item.toJson()).toList(),
    };
  }
}

class CreateAgentDialogRequest {
  final String agentId;
  final String? conversationId;

  CreateAgentDialogRequest({
    required this.agentId,
    this.conversationId
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
      conversationId: json['conversationId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'agentId': agentId,
      'conversationId': conversationId,
    };
  }
}

class CreateAgentHandoffRequest {
  final String conversationId;
  final String targetId;
  final String targetKind;
  final String handoffSessionId;
  final String? handoffReason;

  CreateAgentHandoffRequest({
    required this.conversationId,
    required this.targetId,
    required this.targetKind,
    required this.handoffSessionId,
    this.handoffReason
  });

  factory CreateAgentHandoffRequest.fromJson(Map<String, dynamic> json) {
    return CreateAgentHandoffRequest(
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('CreateAgentHandoffRequest.conversationId is required');
        }
        return value;
      })(),
      targetId: (() {
        final value = json['targetId']?.toString();
        if (value == null) {
          throw FormatException('CreateAgentHandoffRequest.targetId is required');
        }
        return value;
      })(),
      targetKind: (() {
        final value = json['targetKind']?.toString();
        if (value == null) {
          throw FormatException('CreateAgentHandoffRequest.targetKind is required');
        }
        return value;
      })(),
      handoffSessionId: (() {
        final value = json['handoffSessionId']?.toString();
        if (value == null) {
          throw FormatException('CreateAgentHandoffRequest.handoffSessionId is required');
        }
        return value;
      })(),
      handoffReason: json['handoffReason']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'targetId': targetId,
      'targetKind': targetKind,
      'handoffSessionId': handoffSessionId,
      'handoffReason': handoffReason,
    };
  }
}

class CreateSystemChannelRequest {
  final String conversationId;
  final String subscriberId;

  CreateSystemChannelRequest({
    required this.conversationId,
    required this.subscriberId
  });

  factory CreateSystemChannelRequest.fromJson(Map<String, dynamic> json) {
    return CreateSystemChannelRequest(
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('CreateSystemChannelRequest.conversationId is required');
        }
        return value;
      })(),
      subscriberId: (() {
        final value = json['subscriberId']?.toString();
        if (value == null) {
          throw FormatException('CreateSystemChannelRequest.subscriberId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'subscriberId': subscriberId,
    };
  }
}

class CreateThreadConversationRequest {
  final String conversationId;
  final String parentConversationId;
  final String rootMessageId;

  CreateThreadConversationRequest({
    required this.conversationId,
    required this.parentConversationId,
    required this.rootMessageId
  });

  factory CreateThreadConversationRequest.fromJson(Map<String, dynamic> json) {
    return CreateThreadConversationRequest(
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('CreateThreadConversationRequest.conversationId is required');
        }
        return value;
      })(),
      parentConversationId: (() {
        final value = json['parentConversationId']?.toString();
        if (value == null) {
          throw FormatException('CreateThreadConversationRequest.parentConversationId is required');
        }
        return value;
      })(),
      rootMessageId: (() {
        final value = json['rootMessageId']?.toString();
        if (value == null) {
          throw FormatException('CreateThreadConversationRequest.rootMessageId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'parentConversationId': parentConversationId,
      'rootMessageId': rootMessageId,
    };
  }
}

class BindDirectChatRequest {
  final String? conversationId;
  final String? directChatId;
  final String leftActorId;
  final String leftActorKind;
  final String rightActorId;
  final String rightActorKind;

  BindDirectChatRequest({
    this.conversationId,
    this.directChatId,
    required this.leftActorId,
    required this.leftActorKind,
    required this.rightActorId,
    required this.rightActorKind
  });

  factory BindDirectChatRequest.fromJson(Map<String, dynamic> json) {
    return BindDirectChatRequest(
      conversationId: json['conversationId']?.toString(),
      directChatId: json['directChatId']?.toString(),
      leftActorId: (() {
        final value = json['leftActorId']?.toString();
        if (value == null) {
          throw FormatException('BindDirectChatRequest.leftActorId is required');
        }
        return value;
      })(),
      leftActorKind: (() {
        final value = json['leftActorKind']?.toString();
        if (value == null) {
          throw FormatException('BindDirectChatRequest.leftActorKind is required');
        }
        return value;
      })(),
      rightActorId: (() {
        final value = json['rightActorId']?.toString();
        if (value == null) {
          throw FormatException('BindDirectChatRequest.rightActorId is required');
        }
        return value;
      })(),
      rightActorKind: (() {
        final value = json['rightActorKind']?.toString();
        if (value == null) {
          throw FormatException('BindDirectChatRequest.rightActorKind is required');
        }
        return value;
      })()
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
    };
  }
}

class CreateConversationResult {
  final String conversationId;
  final String eventId;
  final String? requestKey;
  final String? deliveryStatus;
  final String? proofVersion;
  final String? knowledgebaseInitialization;

  CreateConversationResult({
    required this.conversationId,
    required this.eventId,
    this.requestKey,
    this.deliveryStatus,
    this.proofVersion,
    this.knowledgebaseInitialization
  });

  factory CreateConversationResult.fromJson(Map<String, dynamic> json) {
    return CreateConversationResult(
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('CreateConversationResult.conversationId is required');
        }
        return value;
      })(),
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('CreateConversationResult.eventId is required');
        }
        return value;
      })(),
      requestKey: json['requestKey']?.toString(),
      deliveryStatus: json['deliveryStatus']?.toString(),
      proofVersion: json['proofVersion']?.toString(),
      knowledgebaseInitialization: json['knowledgebaseInitialization']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'eventId': eventId,
      'requestKey': requestKey,
      'deliveryStatus': deliveryStatus,
      'proofVersion': proofVersion,
      'knowledgebaseInitialization': knowledgebaseInitialization,
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

class MentionContentPart implements ContentPart {
  final String kind;
  final String targetKind;
  final String targetId;
  final String displayText;
  final int assignmentGeneration;

  MentionContentPart({
    required this.kind,
    required this.targetKind,
    required this.targetId,
    required this.displayText,
    required this.assignmentGeneration
  });

  factory MentionContentPart.fromJson(Map<String, dynamic> json) {
    return MentionContentPart(
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('MentionContentPart.kind is required');
        }
        return value;
      })(),
      targetKind: (() {
        final value = json['targetKind']?.toString();
        if (value == null) {
          throw FormatException('MentionContentPart.targetKind is required');
        }
        return value;
      })(),
      targetId: (() {
        final value = json['targetId']?.toString();
        if (value == null) {
          throw FormatException('MentionContentPart.targetId is required');
        }
        return value;
      })(),
      displayText: (() {
        final value = json['displayText']?.toString();
        if (value == null) {
          throw FormatException('MentionContentPart.displayText is required');
        }
        return value;
      })(),
      assignmentGeneration: (() {
        final value = json['assignmentGeneration'];
        if (value is! int) {
          throw FormatException('MentionContentPart.assignmentGeneration is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'kind': kind,
      'targetKind': targetKind,
      'targetId': targetId,
      'displayText': displayText,
      'assignmentGeneration': assignmentGeneration,
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

class PresenceHeartbeatResponse {
  final int code;
  final dynamic data;
  final String traceId;

  PresenceHeartbeatResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory PresenceHeartbeatResponse.fromJson(Map<String, dynamic> json) {
    return PresenceHeartbeatResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('PresenceHeartbeatResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('PresenceHeartbeatResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('PresenceHeartbeatResponse.traceId is required');
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

class PresenceMeRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  PresenceMeRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory PresenceMeRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return PresenceMeRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('PresenceMeRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('PresenceMeRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('PresenceMeRetrieveResponse.traceId is required');
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

class RealtimeSubscriptionsSyncResponse {
  final int code;
  final dynamic data;
  final String traceId;

  RealtimeSubscriptionsSyncResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory RealtimeSubscriptionsSyncResponse.fromJson(Map<String, dynamic> json) {
    return RealtimeSubscriptionsSyncResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('RealtimeSubscriptionsSyncResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('RealtimeSubscriptionsSyncResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('RealtimeSubscriptionsSyncResponse.traceId is required');
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

class RealtimeEventsAckResponse {
  final int code;
  final dynamic data;
  final String traceId;

  RealtimeEventsAckResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory RealtimeEventsAckResponse.fromJson(Map<String, dynamic> json) {
    return RealtimeEventsAckResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('RealtimeEventsAckResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('RealtimeEventsAckResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('RealtimeEventsAckResponse.traceId is required');
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

class RealtimeEventsListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  RealtimeEventsListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory RealtimeEventsListResponse.fromJson(Map<String, dynamic> json) {
    return RealtimeEventsListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('RealtimeEventsListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('RealtimeEventsListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('RealtimeEventsListResponse.traceId is required');
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

class CallsSessionsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  CallsSessionsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory CallsSessionsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return CallsSessionsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('CallsSessionsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('CallsSessionsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('CallsSessionsCreateResponse201.traceId is required');
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

class CallsSessionsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  CallsSessionsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory CallsSessionsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return CallsSessionsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('CallsSessionsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('CallsSessionsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('CallsSessionsRetrieveResponse.traceId is required');
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

class CallsSessionsInviteResponse {
  final int code;
  final dynamic data;
  final String traceId;

  CallsSessionsInviteResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory CallsSessionsInviteResponse.fromJson(Map<String, dynamic> json) {
    return CallsSessionsInviteResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('CallsSessionsInviteResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('CallsSessionsInviteResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('CallsSessionsInviteResponse.traceId is required');
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

class CallsSessionsAcceptResponse {
  final int code;
  final dynamic data;
  final String traceId;

  CallsSessionsAcceptResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory CallsSessionsAcceptResponse.fromJson(Map<String, dynamic> json) {
    return CallsSessionsAcceptResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('CallsSessionsAcceptResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('CallsSessionsAcceptResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('CallsSessionsAcceptResponse.traceId is required');
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

class CallsSessionsRejectResponse {
  final int code;
  final dynamic data;
  final String traceId;

  CallsSessionsRejectResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory CallsSessionsRejectResponse.fromJson(Map<String, dynamic> json) {
    return CallsSessionsRejectResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('CallsSessionsRejectResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('CallsSessionsRejectResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('CallsSessionsRejectResponse.traceId is required');
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

class CallsSessionsEndResponse {
  final int code;
  final dynamic data;
  final String traceId;

  CallsSessionsEndResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory CallsSessionsEndResponse.fromJson(Map<String, dynamic> json) {
    return CallsSessionsEndResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('CallsSessionsEndResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('CallsSessionsEndResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('CallsSessionsEndResponse.traceId is required');
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

class CallsSessionsSignalsListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  CallsSessionsSignalsListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory CallsSessionsSignalsListResponse.fromJson(Map<String, dynamic> json) {
    return CallsSessionsSignalsListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('CallsSessionsSignalsListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('CallsSessionsSignalsListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('CallsSessionsSignalsListResponse.traceId is required');
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

class CallsSessionsSignalsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  CallsSessionsSignalsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory CallsSessionsSignalsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return CallsSessionsSignalsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('CallsSessionsSignalsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('CallsSessionsSignalsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('CallsSessionsSignalsCreateResponse201.traceId is required');
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

class CallsSessionsCredentialsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  CallsSessionsCredentialsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory CallsSessionsCredentialsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return CallsSessionsCredentialsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('CallsSessionsCredentialsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('CallsSessionsCredentialsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('CallsSessionsCredentialsCreateResponse201.traceId is required');
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

class CallsSessionsCredentialsRefreshResponse {
  final int code;
  final dynamic data;
  final String traceId;

  CallsSessionsCredentialsRefreshResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory CallsSessionsCredentialsRefreshResponse.fromJson(Map<String, dynamic> json) {
    return CallsSessionsCredentialsRefreshResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('CallsSessionsCredentialsRefreshResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('CallsSessionsCredentialsRefreshResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('CallsSessionsCredentialsRefreshResponse.traceId is required');
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

class SocialUsersListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialUsersListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialUsersListResponse.fromJson(Map<String, dynamic> json) {
    return SocialUsersListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialUsersListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialUsersListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialUsersListResponse.traceId is required');
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

class SocialFriendRequestsPendingCountRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialFriendRequestsPendingCountRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialFriendRequestsPendingCountRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestsPendingCountRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialFriendRequestsPendingCountRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialFriendRequestsPendingCountRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialFriendRequestsPendingCountRetrieveResponse.traceId is required');
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

class SocialContactsTagsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialContactsTagsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialContactsTagsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialContactsTagsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialContactsTagsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialContactsTagsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialContactsTagsCreateResponse201.traceId is required');
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

class SocialContactsTagsUpdateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialContactsTagsUpdateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialContactsTagsUpdateResponse.fromJson(Map<String, dynamic> json) {
    return SocialContactsTagsUpdateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialContactsTagsUpdateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialContactsTagsUpdateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialContactsTagsUpdateResponse.traceId is required');
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

class SocialContactsRecommendationsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SocialContactsRecommendationsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialContactsRecommendationsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SocialContactsRecommendationsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialContactsRecommendationsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialContactsRecommendationsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialContactsRecommendationsCreateResponse201.traceId is required');
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

class SocialContactsPreferencesRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialContactsPreferencesRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialContactsPreferencesRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SocialContactsPreferencesRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialContactsPreferencesRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialContactsPreferencesRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialContactsPreferencesRetrieveResponse.traceId is required');
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

class SocialContactsPreferencesUpdateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SocialContactsPreferencesUpdateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SocialContactsPreferencesUpdateResponse.fromJson(Map<String, dynamic> json) {
    return SocialContactsPreferencesUpdateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SocialContactsPreferencesUpdateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SocialContactsPreferencesUpdateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SocialContactsPreferencesUpdateResponse.traceId is required');
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

class ContactsListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ContactsListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ContactsListResponse.fromJson(Map<String, dynamic> json) {
    return ContactsListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ContactsListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ContactsListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ContactsListResponse.traceId is required');
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

class InboxListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  InboxListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory InboxListResponse.fromJson(Map<String, dynamic> json) {
    return InboxListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('InboxListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('InboxListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('InboxListResponse.traceId is required');
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

class ConversationsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ConversationsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsCreateResponse201.traceId is required');
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

class ConversationsAgentDialogsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsAgentDialogsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsAgentDialogsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ConversationsAgentDialogsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsAgentDialogsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsAgentDialogsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsAgentDialogsCreateResponse201.traceId is required');
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

class ConversationsAgentHandoffsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsAgentHandoffsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsAgentHandoffsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ConversationsAgentHandoffsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsAgentHandoffsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsAgentHandoffsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsAgentHandoffsCreateResponse201.traceId is required');
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

class ConversationsSystemChannelsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsSystemChannelsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsSystemChannelsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ConversationsSystemChannelsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsSystemChannelsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsSystemChannelsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsSystemChannelsCreateResponse201.traceId is required');
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

class ConversationsThreadsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsThreadsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsThreadsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ConversationsThreadsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsThreadsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsThreadsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsThreadsCreateResponse201.traceId is required');
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

class ConversationsDirectChatsBindingsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsDirectChatsBindingsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsDirectChatsBindingsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ConversationsDirectChatsBindingsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsDirectChatsBindingsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsDirectChatsBindingsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsDirectChatsBindingsCreateResponse201.traceId is required');
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

class ConversationsAgentHandoffRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsAgentHandoffRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsAgentHandoffRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsAgentHandoffRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsAgentHandoffRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsAgentHandoffRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsAgentHandoffRetrieveResponse.traceId is required');
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

class ConversationsAgentHandoffAcceptResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsAgentHandoffAcceptResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsAgentHandoffAcceptResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsAgentHandoffAcceptResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsAgentHandoffAcceptResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsAgentHandoffAcceptResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsAgentHandoffAcceptResponse.traceId is required');
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

class ConversationsAgentHandoffResolveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsAgentHandoffResolveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsAgentHandoffResolveResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsAgentHandoffResolveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsAgentHandoffResolveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsAgentHandoffResolveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsAgentHandoffResolveResponse.traceId is required');
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

class ConversationsAgentHandoffCloseResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsAgentHandoffCloseResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsAgentHandoffCloseResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsAgentHandoffCloseResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsAgentHandoffCloseResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsAgentHandoffCloseResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsAgentHandoffCloseResponse.traceId is required');
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

class ConversationsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsRetrieveResponse.traceId is required');
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

class ConversationsMembersListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsMembersListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsMembersListResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsMembersListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsMembersListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsMembersListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsMembersListResponse.traceId is required');
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

class ConversationsMembersCurrentRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsMembersCurrentRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsMembersCurrentRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsMembersCurrentRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsMembersCurrentRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsMembersCurrentRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsMembersCurrentRetrieveResponse.traceId is required');
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

class ConversationsAgentsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsAgentsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsAgentsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsAgentsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsAgentsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsAgentsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsAgentsRetrieveResponse.traceId is required');
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

class ConversationsAgentsUpdateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsAgentsUpdateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsAgentsUpdateResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsAgentsUpdateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsAgentsUpdateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsAgentsUpdateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsAgentsUpdateResponse.traceId is required');
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

class ConversationsMembersAddResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsMembersAddResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsMembersAddResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsMembersAddResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsMembersAddResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsMembersAddResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsMembersAddResponse.traceId is required');
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

class ConversationsMembersRemoveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsMembersRemoveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsMembersRemoveResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsMembersRemoveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsMembersRemoveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsMembersRemoveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsMembersRemoveResponse.traceId is required');
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

class ConversationsMembersTransferOwnerResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsMembersTransferOwnerResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsMembersTransferOwnerResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsMembersTransferOwnerResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsMembersTransferOwnerResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsMembersTransferOwnerResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsMembersTransferOwnerResponse.traceId is required');
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

class ConversationsMembersChangeRoleResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsMembersChangeRoleResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsMembersChangeRoleResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsMembersChangeRoleResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsMembersChangeRoleResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsMembersChangeRoleResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsMembersChangeRoleResponse.traceId is required');
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

class ConversationsMembersLeaveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsMembersLeaveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsMembersLeaveResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsMembersLeaveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsMembersLeaveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsMembersLeaveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsMembersLeaveResponse.traceId is required');
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

class ConversationsMembersAcceptInvitationResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsMembersAcceptInvitationResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsMembersAcceptInvitationResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsMembersAcceptInvitationResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsMembersAcceptInvitationResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsMembersAcceptInvitationResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsMembersAcceptInvitationResponse.traceId is required');
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

class ConversationsPreferencesRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsPreferencesRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsPreferencesRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsPreferencesRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsPreferencesRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsPreferencesRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsPreferencesRetrieveResponse.traceId is required');
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

class ConversationsPreferencesUpdateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsPreferencesUpdateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsPreferencesUpdateResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsPreferencesUpdateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsPreferencesUpdateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsPreferencesUpdateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsPreferencesUpdateResponse.traceId is required');
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

class ConversationsProfileRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsProfileRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsProfileRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsProfileRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsProfileRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsProfileRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsProfileRetrieveResponse.traceId is required');
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

class ConversationsProfileUpdateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsProfileUpdateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsProfileUpdateResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsProfileUpdateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsProfileUpdateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsProfileUpdateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsProfileUpdateResponse.traceId is required');
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

class ConversationsReadCursorRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsReadCursorRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsReadCursorRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsReadCursorRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsReadCursorRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsReadCursorRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsReadCursorRetrieveResponse.traceId is required');
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

class ConversationsReadCursorUpdateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsReadCursorUpdateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsReadCursorUpdateResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsReadCursorUpdateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsReadCursorUpdateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsReadCursorUpdateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsReadCursorUpdateResponse.traceId is required');
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

class ConversationsMemberDirectoryListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsMemberDirectoryListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsMemberDirectoryListResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsMemberDirectoryListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsMemberDirectoryListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsMemberDirectoryListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsMemberDirectoryListResponse.traceId is required');
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

class ConversationsMessagesCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsMessagesCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsMessagesCreateResponse201.fromJson(Map<String, dynamic> json) {
    return ConversationsMessagesCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsMessagesCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsMessagesCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsMessagesCreateResponse201.traceId is required');
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

class ConversationsSystemChannelPublishResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsSystemChannelPublishResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsSystemChannelPublishResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsSystemChannelPublishResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsSystemChannelPublishResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsSystemChannelPublishResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsSystemChannelPublishResponse.traceId is required');
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

class ConversationsPinsListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsPinsListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsPinsListResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsPinsListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsPinsListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsPinsListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsPinsListResponse.traceId is required');
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

class ConversationsMessagesInteractionSummaryRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationsMessagesInteractionSummaryRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationsMessagesInteractionSummaryRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ConversationsMessagesInteractionSummaryRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationsMessagesInteractionSummaryRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationsMessagesInteractionSummaryRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationsMessagesInteractionSummaryRetrieveResponse.traceId is required');
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

class MessagesEditResponse {
  final int code;
  final dynamic data;
  final String traceId;

  MessagesEditResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory MessagesEditResponse.fromJson(Map<String, dynamic> json) {
    return MessagesEditResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('MessagesEditResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('MessagesEditResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('MessagesEditResponse.traceId is required');
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

class MessagesRecallResponse {
  final int code;
  final dynamic data;
  final String traceId;

  MessagesRecallResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory MessagesRecallResponse.fromJson(Map<String, dynamic> json) {
    return MessagesRecallResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('MessagesRecallResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('MessagesRecallResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('MessagesRecallResponse.traceId is required');
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

class MessagesFavoritesListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  MessagesFavoritesListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory MessagesFavoritesListResponse.fromJson(Map<String, dynamic> json) {
    return MessagesFavoritesListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('MessagesFavoritesListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('MessagesFavoritesListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('MessagesFavoritesListResponse.traceId is required');
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

class MessagesFavoritesCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  MessagesFavoritesCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory MessagesFavoritesCreateResponse201.fromJson(Map<String, dynamic> json) {
    return MessagesFavoritesCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('MessagesFavoritesCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('MessagesFavoritesCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('MessagesFavoritesCreateResponse201.traceId is required');
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

class MessagesReactionsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  MessagesReactionsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory MessagesReactionsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return MessagesReactionsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('MessagesReactionsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('MessagesReactionsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('MessagesReactionsCreateResponse201.traceId is required');
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

class MessagesReactionsRemoveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  MessagesReactionsRemoveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory MessagesReactionsRemoveResponse.fromJson(Map<String, dynamic> json) {
    return MessagesReactionsRemoveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('MessagesReactionsRemoveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('MessagesReactionsRemoveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('MessagesReactionsRemoveResponse.traceId is required');
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

class MessagesPinResponse {
  final int code;
  final dynamic data;
  final String traceId;

  MessagesPinResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory MessagesPinResponse.fromJson(Map<String, dynamic> json) {
    return MessagesPinResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('MessagesPinResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('MessagesPinResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('MessagesPinResponse.traceId is required');
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

class MessagesUnpinResponse {
  final int code;
  final dynamic data;
  final String traceId;

  MessagesUnpinResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory MessagesUnpinResponse.fromJson(Map<String, dynamic> json) {
    return MessagesUnpinResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('MessagesUnpinResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('MessagesUnpinResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('MessagesUnpinResponse.traceId is required');
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

class RoomsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  RoomsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory RoomsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return RoomsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('RoomsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('RoomsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('RoomsCreateResponse201.traceId is required');
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

class RoomsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  RoomsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory RoomsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return RoomsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('RoomsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('RoomsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('RoomsRetrieveResponse.traceId is required');
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

class RoomsEnterResponse {
  final int code;
  final dynamic data;
  final String traceId;

  RoomsEnterResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory RoomsEnterResponse.fromJson(Map<String, dynamic> json) {
    return RoomsEnterResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('RoomsEnterResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('RoomsEnterResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('RoomsEnterResponse.traceId is required');
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

class RoomsLeaveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  RoomsLeaveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory RoomsLeaveResponse.fromJson(Map<String, dynamic> json) {
    return RoomsLeaveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('RoomsLeaveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('RoomsLeaveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('RoomsLeaveResponse.traceId is required');
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

class StreamsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  StreamsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory StreamsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return StreamsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('StreamsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('StreamsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('StreamsCreateResponse201.traceId is required');
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

class StreamsFramesListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  StreamsFramesListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory StreamsFramesListResponse.fromJson(Map<String, dynamic> json) {
    return StreamsFramesListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('StreamsFramesListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('StreamsFramesListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('StreamsFramesListResponse.traceId is required');
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

class StreamsFramesCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  StreamsFramesCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory StreamsFramesCreateResponse201.fromJson(Map<String, dynamic> json) {
    return StreamsFramesCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('StreamsFramesCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('StreamsFramesCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('StreamsFramesCreateResponse201.traceId is required');
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

class StreamsCheckpointResponse {
  final int code;
  final dynamic data;
  final String traceId;

  StreamsCheckpointResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory StreamsCheckpointResponse.fromJson(Map<String, dynamic> json) {
    return StreamsCheckpointResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('StreamsCheckpointResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('StreamsCheckpointResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('StreamsCheckpointResponse.traceId is required');
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

class StreamsCompleteResponse {
  final int code;
  final dynamic data;
  final String traceId;

  StreamsCompleteResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory StreamsCompleteResponse.fromJson(Map<String, dynamic> json) {
    return StreamsCompleteResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('StreamsCompleteResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('StreamsCompleteResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('StreamsCompleteResponse.traceId is required');
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

class StreamsAbortResponse {
  final int code;
  final dynamic data;
  final String traceId;

  StreamsAbortResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory StreamsAbortResponse.fromJson(Map<String, dynamic> json) {
    return StreamsAbortResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('StreamsAbortResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('StreamsAbortResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('StreamsAbortResponse.traceId is required');
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

class SpacesCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SpacesCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesCreateResponse201.traceId is required');
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

class SpacesListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesListResponse.fromJson(Map<String, dynamic> json) {
    return SpacesListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesListResponse.traceId is required');
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

class SpacesRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SpacesRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesRetrieveResponse.traceId is required');
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

class SpacesUpdateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesUpdateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesUpdateResponse.fromJson(Map<String, dynamic> json) {
    return SpacesUpdateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesUpdateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesUpdateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesUpdateResponse.traceId is required');
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

class SpacesMembersListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesMembersListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesMembersListResponse.fromJson(Map<String, dynamic> json) {
    return SpacesMembersListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesMembersListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesMembersListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesMembersListResponse.traceId is required');
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

class SpacesMembersCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesMembersCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesMembersCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SpacesMembersCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesMembersCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesMembersCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesMembersCreateResponse201.traceId is required');
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

class SpacesMembersRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesMembersRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesMembersRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SpacesMembersRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesMembersRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesMembersRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesMembersRetrieveResponse.traceId is required');
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

class SpacesMembersUpdateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesMembersUpdateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesMembersUpdateResponse.fromJson(Map<String, dynamic> json) {
    return SpacesMembersUpdateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesMembersUpdateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesMembersUpdateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesMembersUpdateResponse.traceId is required');
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

class SpacesGroupsListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesGroupsListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesGroupsListResponse.fromJson(Map<String, dynamic> json) {
    return SpacesGroupsListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesGroupsListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesGroupsListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesGroupsListResponse.traceId is required');
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

class SpacesGroupsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesGroupsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesGroupsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SpacesGroupsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesGroupsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesGroupsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesGroupsCreateResponse201.traceId is required');
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

class SpacesGroupsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesGroupsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesGroupsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SpacesGroupsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesGroupsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesGroupsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesGroupsRetrieveResponse.traceId is required');
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

class SpacesGroupsUpdateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesGroupsUpdateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesGroupsUpdateResponse.fromJson(Map<String, dynamic> json) {
    return SpacesGroupsUpdateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesGroupsUpdateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesGroupsUpdateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesGroupsUpdateResponse.traceId is required');
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

class SpacesGroupsMembersListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesGroupsMembersListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesGroupsMembersListResponse.fromJson(Map<String, dynamic> json) {
    return SpacesGroupsMembersListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesGroupsMembersListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesGroupsMembersListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesGroupsMembersListResponse.traceId is required');
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

class SpacesGroupsMembersCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesGroupsMembersCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesGroupsMembersCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SpacesGroupsMembersCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesGroupsMembersCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesGroupsMembersCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesGroupsMembersCreateResponse201.traceId is required');
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

class SpacesGroupsMembersRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesGroupsMembersRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesGroupsMembersRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SpacesGroupsMembersRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesGroupsMembersRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesGroupsMembersRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesGroupsMembersRetrieveResponse.traceId is required');
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

class SpacesGroupsMembersUpdateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesGroupsMembersUpdateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesGroupsMembersUpdateResponse.fromJson(Map<String, dynamic> json) {
    return SpacesGroupsMembersUpdateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesGroupsMembersUpdateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesGroupsMembersUpdateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesGroupsMembersUpdateResponse.traceId is required');
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

class SpacesChannelsListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesChannelsListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesChannelsListResponse.fromJson(Map<String, dynamic> json) {
    return SpacesChannelsListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesChannelsListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesChannelsListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesChannelsListResponse.traceId is required');
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

class SpacesChannelsCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesChannelsCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesChannelsCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SpacesChannelsCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesChannelsCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesChannelsCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesChannelsCreateResponse201.traceId is required');
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

class SpacesChannelsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesChannelsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesChannelsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SpacesChannelsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesChannelsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesChannelsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesChannelsRetrieveResponse.traceId is required');
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

class SpacesChannelsUpdateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesChannelsUpdateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesChannelsUpdateResponse.fromJson(Map<String, dynamic> json) {
    return SpacesChannelsUpdateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesChannelsUpdateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesChannelsUpdateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesChannelsUpdateResponse.traceId is required');
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

class SpacesChannelsAccessRulesListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesChannelsAccessRulesListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesChannelsAccessRulesListResponse.fromJson(Map<String, dynamic> json) {
    return SpacesChannelsAccessRulesListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesChannelsAccessRulesListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesChannelsAccessRulesListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesChannelsAccessRulesListResponse.traceId is required');
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

class SpacesChannelsAccessRulesCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesChannelsAccessRulesCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesChannelsAccessRulesCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SpacesChannelsAccessRulesCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesChannelsAccessRulesCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesChannelsAccessRulesCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesChannelsAccessRulesCreateResponse201.traceId is required');
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

class SpacesInvitesListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesInvitesListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesInvitesListResponse.fromJson(Map<String, dynamic> json) {
    return SpacesInvitesListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesInvitesListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesInvitesListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesInvitesListResponse.traceId is required');
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

class SpacesInvitesCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesInvitesCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesInvitesCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SpacesInvitesCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesInvitesCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesInvitesCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesInvitesCreateResponse201.traceId is required');
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

class SpacesInvitesRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesInvitesRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesInvitesRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SpacesInvitesRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesInvitesRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesInvitesRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesInvitesRetrieveResponse.traceId is required');
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

class SpacesBansListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesBansListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesBansListResponse.fromJson(Map<String, dynamic> json) {
    return SpacesBansListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesBansListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesBansListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesBansListResponse.traceId is required');
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

class SpacesBansCreateResponse201 {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesBansCreateResponse201({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesBansCreateResponse201.fromJson(Map<String, dynamic> json) {
    return SpacesBansCreateResponse201(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesBansCreateResponse201.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesBansCreateResponse201.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesBansCreateResponse201.traceId is required');
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

class SpacesBansRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SpacesBansRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SpacesBansRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return SpacesBansRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SpacesBansRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('SpacesBansRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SpacesBansRetrieveResponse.traceId is required');
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
