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

class PortalWorkspaceView {
  final String name;
  final String slug;
  final String tier;
  final String region;
  final String supportPlan;
  final int seats;
  final int activeBrands;
  final String uptime;

  PortalWorkspaceView({
    required this.name,
    required this.slug,
    required this.tier,
    required this.region,
    required this.supportPlan,
    required this.seats,
    required this.activeBrands,
    required this.uptime
  });

  factory PortalWorkspaceView.fromJson(Map<String, dynamic> json) {
    return PortalWorkspaceView(
      name: (() {
        final value = json['name']?.toString();
        if (value == null) {
          throw FormatException('PortalWorkspaceView.name is required');
        }
        return value;
      })(),
      slug: (() {
        final value = json['slug']?.toString();
        if (value == null) {
          throw FormatException('PortalWorkspaceView.slug is required');
        }
        return value;
      })(),
      tier: (() {
        final value = json['tier']?.toString();
        if (value == null) {
          throw FormatException('PortalWorkspaceView.tier is required');
        }
        return value;
      })(),
      region: (() {
        final value = json['region']?.toString();
        if (value == null) {
          throw FormatException('PortalWorkspaceView.region is required');
        }
        return value;
      })(),
      supportPlan: (() {
        final value = json['supportPlan']?.toString();
        if (value == null) {
          throw FormatException('PortalWorkspaceView.supportPlan is required');
        }
        return value;
      })(),
      seats: (() {
        final value = json['seats'];
        if (value is! int) {
          throw FormatException('PortalWorkspaceView.seats is required');
        }
        return value;
      })(),
      activeBrands: (() {
        final value = json['activeBrands'];
        if (value is! int) {
          throw FormatException('PortalWorkspaceView.activeBrands is required');
        }
        return value;
      })(),
      uptime: (() {
        final value = json['uptime']?.toString();
        if (value == null) {
          throw FormatException('PortalWorkspaceView.uptime is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'name': name,
      'slug': slug,
      'tier': tier,
      'region': region,
      'supportPlan': supportPlan,
      'seats': seats,
      'activeBrands': activeBrands,
      'uptime': uptime,
    };
  }
}

class Sender {
  final String id;
  final String kind;
  final String? memberId;
  final String? deviceId;
  final String? sessionId;
  final Map<String, String> metadata;

  Sender({
    required this.id,
    required this.kind,
    this.memberId,
    this.deviceId,
    this.sessionId,
    required this.metadata
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
      memberId: json['memberId']?.toString(),
      deviceId: json['deviceId']?.toString(),
      sessionId: json['sessionId']?.toString(),
      metadata: (() {
        final map = _sdkworkAsMap(json['metadata']);
        if (map == null) {
          throw FormatException('Sender.metadata is required');
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
      'id': id,
      'kind': kind,
      'memberId': memberId,
      'deviceId': deviceId,
      'sessionId': sessionId,
      'metadata': metadata.map((key, item) => MapEntry(key, item)),
    };
  }
}

class StreamSession {
  final String tenantId;
  final String streamId;
  final String streamType;
  final String scopeKind;
  final String scopeId;
  final String durabilityClass;
  final String orderingScope;
  final String? schemaRef;
  final String state;
  final String lastFrameSeq;
  final String? lastCheckpointSeq;
  final String? resultMessageId;
  final String openedAt;
  final String? closedAt;
  final String? expiresAt;

  StreamSession({
    required this.tenantId,
    required this.streamId,
    required this.streamType,
    required this.scopeKind,
    required this.scopeId,
    required this.durabilityClass,
    required this.orderingScope,
    this.schemaRef,
    required this.state,
    required this.lastFrameSeq,
    this.lastCheckpointSeq,
    this.resultMessageId,
    required this.openedAt,
    this.closedAt,
    this.expiresAt
  });

  factory StreamSession.fromJson(Map<String, dynamic> json) {
    return StreamSession(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('StreamSession.tenantId is required');
        }
        return value;
      })(),
      streamId: (() {
        final value = json['streamId']?.toString();
        if (value == null) {
          throw FormatException('StreamSession.streamId is required');
        }
        return value;
      })(),
      streamType: (() {
        final value = json['streamType']?.toString();
        if (value == null) {
          throw FormatException('StreamSession.streamType is required');
        }
        return value;
      })(),
      scopeKind: (() {
        final value = json['scopeKind']?.toString();
        if (value == null) {
          throw FormatException('StreamSession.scopeKind is required');
        }
        return value;
      })(),
      scopeId: (() {
        final value = json['scopeId']?.toString();
        if (value == null) {
          throw FormatException('StreamSession.scopeId is required');
        }
        return value;
      })(),
      durabilityClass: (() {
        final value = json['durabilityClass']?.toString();
        if (value == null) {
          throw FormatException('StreamSession.durabilityClass is required');
        }
        return value;
      })(),
      orderingScope: (() {
        final value = json['orderingScope']?.toString();
        if (value == null) {
          throw FormatException('StreamSession.orderingScope is required');
        }
        return value;
      })(),
      schemaRef: json['schemaRef']?.toString(),
      state: (() {
        final value = json['state']?.toString();
        if (value == null) {
          throw FormatException('StreamSession.state is required');
        }
        return value;
      })(),
      lastFrameSeq: (() {
        final value = json['lastFrameSeq']?.toString();
        if (value == null) {
          throw FormatException('StreamSession.lastFrameSeq is required');
        }
        return value;
      })(),
      lastCheckpointSeq: json['lastCheckpointSeq']?.toString(),
      resultMessageId: json['resultMessageId']?.toString(),
      openedAt: (() {
        final value = json['openedAt']?.toString();
        if (value == null) {
          throw FormatException('StreamSession.openedAt is required');
        }
        return value;
      })(),
      closedAt: json['closedAt']?.toString(),
      expiresAt: json['expiresAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'streamId': streamId,
      'streamType': streamType,
      'scopeKind': scopeKind,
      'scopeId': scopeId,
      'durabilityClass': durabilityClass,
      'orderingScope': orderingScope,
      'schemaRef': schemaRef,
      'state': state,
      'lastFrameSeq': lastFrameSeq,
      'lastCheckpointSeq': lastCheckpointSeq,
      'resultMessageId': resultMessageId,
      'openedAt': openedAt,
      'closedAt': closedAt,
      'expiresAt': expiresAt,
    };
  }
}

class StreamFrame {
  final String tenantId;
  final String streamId;
  final String streamType;
  final String scopeKind;
  final String scopeId;
  final String frameSeq;
  final String frameType;
  final String? schemaRef;
  final String encoding;
  final String payload;
  final Sender sender;
  final Map<String, String> attributes;
  final String occurredAt;

  StreamFrame({
    required this.tenantId,
    required this.streamId,
    required this.streamType,
    required this.scopeKind,
    required this.scopeId,
    required this.frameSeq,
    required this.frameType,
    this.schemaRef,
    required this.encoding,
    required this.payload,
    required this.sender,
    required this.attributes,
    required this.occurredAt
  });

  factory StreamFrame.fromJson(Map<String, dynamic> json) {
    return StreamFrame(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('StreamFrame.tenantId is required');
        }
        return value;
      })(),
      streamId: (() {
        final value = json['streamId']?.toString();
        if (value == null) {
          throw FormatException('StreamFrame.streamId is required');
        }
        return value;
      })(),
      streamType: (() {
        final value = json['streamType']?.toString();
        if (value == null) {
          throw FormatException('StreamFrame.streamType is required');
        }
        return value;
      })(),
      scopeKind: (() {
        final value = json['scopeKind']?.toString();
        if (value == null) {
          throw FormatException('StreamFrame.scopeKind is required');
        }
        return value;
      })(),
      scopeId: (() {
        final value = json['scopeId']?.toString();
        if (value == null) {
          throw FormatException('StreamFrame.scopeId is required');
        }
        return value;
      })(),
      frameSeq: (() {
        final value = json['frameSeq']?.toString();
        if (value == null) {
          throw FormatException('StreamFrame.frameSeq is required');
        }
        return value;
      })(),
      frameType: (() {
        final value = json['frameType']?.toString();
        if (value == null) {
          throw FormatException('StreamFrame.frameType is required');
        }
        return value;
      })(),
      schemaRef: json['schemaRef']?.toString(),
      encoding: (() {
        final value = json['encoding']?.toString();
        if (value == null) {
          throw FormatException('StreamFrame.encoding is required');
        }
        return value;
      })(),
      payload: (() {
        final value = json['payload']?.toString();
        if (value == null) {
          throw FormatException('StreamFrame.payload is required');
        }
        return value;
      })(),
      sender: (() {
        final map = _sdkworkAsMap(json['sender']);
        if (map == null) {
          throw FormatException('StreamFrame.sender is required');
        }
        return Sender.fromJson(map);
      })(),
      attributes: (() {
        final map = _sdkworkAsMap(json['attributes']);
        if (map == null) {
          throw FormatException('StreamFrame.attributes is required');
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })(),
      occurredAt: (() {
        final value = json['occurredAt']?.toString();
        if (value == null) {
          throw FormatException('StreamFrame.occurredAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'streamId': streamId,
      'streamType': streamType,
      'scopeKind': scopeKind,
      'scopeId': scopeId,
      'frameSeq': frameSeq,
      'frameType': frameType,
      'schemaRef': schemaRef,
      'encoding': encoding,
      'payload': payload,
      'sender': sender.toJson(),
      'attributes': attributes.map((key, item) => MapEntry(key, item)),
      'occurredAt': occurredAt,
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

class AgentSubject {
  final String agentId;
  final String? sessionId;
  final Map<String, String> metadata;

  AgentSubject({
    required this.agentId,
    this.sessionId,
    required this.metadata
  });

  factory AgentSubject.fromJson(Map<String, dynamic> json) {
    return AgentSubject(
      agentId: (() {
        final value = json['agent_id']?.toString();
        if (value == null) {
          throw FormatException('AgentSubject.agent_id is required');
        }
        return value;
      })(),
      sessionId: json['session_id']?.toString(),
      metadata: (() {
        final map = _sdkworkAsMap(json['metadata']);
        if (map == null) {
          throw FormatException('AgentSubject.metadata is required');
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
      'agent_id': agentId,
      'session_id': sessionId,
      'metadata': metadata.map((key, item) => MapEntry(key, item)),
    };
  }
}

class AgentToolCall {
  final String tenantId;
  final String executionId;
  final String agentId;
  final String toolCallId;
  final String toolName;
  final String argumentsPayload;
  final String? resultPayload;
  final String state;
  final String requestedAt;
  final String? completedAt;

  AgentToolCall({
    required this.tenantId,
    required this.executionId,
    required this.agentId,
    required this.toolCallId,
    required this.toolName,
    required this.argumentsPayload,
    this.resultPayload,
    required this.state,
    required this.requestedAt,
    this.completedAt
  });

  factory AgentToolCall.fromJson(Map<String, dynamic> json) {
    return AgentToolCall(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('AgentToolCall.tenantId is required');
        }
        return value;
      })(),
      executionId: (() {
        final value = json['executionId']?.toString();
        if (value == null) {
          throw FormatException('AgentToolCall.executionId is required');
        }
        return value;
      })(),
      agentId: (() {
        final value = json['agentId']?.toString();
        if (value == null) {
          throw FormatException('AgentToolCall.agentId is required');
        }
        return value;
      })(),
      toolCallId: (() {
        final value = json['toolCallId']?.toString();
        if (value == null) {
          throw FormatException('AgentToolCall.toolCallId is required');
        }
        return value;
      })(),
      toolName: (() {
        final value = json['toolName']?.toString();
        if (value == null) {
          throw FormatException('AgentToolCall.toolName is required');
        }
        return value;
      })(),
      argumentsPayload: (() {
        final value = json['argumentsPayload']?.toString();
        if (value == null) {
          throw FormatException('AgentToolCall.argumentsPayload is required');
        }
        return value;
      })(),
      resultPayload: json['resultPayload']?.toString(),
      state: (() {
        final value = json['state']?.toString();
        if (value == null) {
          throw FormatException('AgentToolCall.state is required');
        }
        return value;
      })(),
      requestedAt: (() {
        final value = json['requestedAt']?.toString();
        if (value == null) {
          throw FormatException('AgentToolCall.requestedAt is required');
        }
        return value;
      })(),
      completedAt: json['completedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'executionId': executionId,
      'agentId': agentId,
      'toolCallId': toolCallId,
      'toolName': toolName,
      'argumentsPayload': argumentsPayload,
      'resultPayload': resultPayload,
      'state': state,
      'requestedAt': requestedAt,
      'completedAt': completedAt,
    };
  }
}

class AppendAgentResponseDeltaRequest {
  final String frameSeq;
  final String frameType;
  final String? schemaRef;
  final String encoding;
  final String payload;
  final Map<String, String>? attributes;

  AppendAgentResponseDeltaRequest({
    required this.frameSeq,
    required this.frameType,
    this.schemaRef,
    required this.encoding,
    required this.payload,
    this.attributes
  });

  factory AppendAgentResponseDeltaRequest.fromJson(Map<String, dynamic> json) {
    return AppendAgentResponseDeltaRequest(
      frameSeq: (() {
        final value = json['frameSeq']?.toString();
        if (value == null) {
          throw FormatException('AppendAgentResponseDeltaRequest.frameSeq is required');
        }
        return value;
      })(),
      frameType: (() {
        final value = json['frameType']?.toString();
        if (value == null) {
          throw FormatException('AppendAgentResponseDeltaRequest.frameType is required');
        }
        return value;
      })(),
      schemaRef: json['schemaRef']?.toString(),
      encoding: (() {
        final value = json['encoding']?.toString();
        if (value == null) {
          throw FormatException('AppendAgentResponseDeltaRequest.encoding is required');
        }
        return value;
      })(),
      payload: (() {
        final value = json['payload']?.toString();
        if (value == null) {
          throw FormatException('AppendAgentResponseDeltaRequest.payload is required');
        }
        return value;
      })(),
      attributes: (() {
        final map = _sdkworkAsMap(json['attributes']);
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
      'frameSeq': frameSeq,
      'frameType': frameType,
      'schemaRef': schemaRef,
      'encoding': encoding,
      'payload': payload,
      'attributes': attributes?.map((key, item) => MapEntry(key, item)),
    };
  }
}

class AutomationExecution {
  final String tenantId;
  final String principalId;
  final String principalKind;
  final String executionId;
  final String triggerType;
  final String targetKind;
  final String targetRef;
  final String? inputPayload;
  final String? outputPayload;
  final String state;
  final int retryCount;
  final String requestedAt;
  final String? completedAt;
  final String? failureReason;

  AutomationExecution({
    required this.tenantId,
    required this.principalId,
    required this.principalKind,
    required this.executionId,
    required this.triggerType,
    required this.targetKind,
    required this.targetRef,
    this.inputPayload,
    this.outputPayload,
    required this.state,
    required this.retryCount,
    required this.requestedAt,
    this.completedAt,
    this.failureReason
  });

  factory AutomationExecution.fromJson(Map<String, dynamic> json) {
    return AutomationExecution(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecution.tenantId is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecution.principalId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecution.principalKind is required');
        }
        return value;
      })(),
      executionId: (() {
        final value = json['executionId']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecution.executionId is required');
        }
        return value;
      })(),
      triggerType: (() {
        final value = json['triggerType']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecution.triggerType is required');
        }
        return value;
      })(),
      targetKind: (() {
        final value = json['targetKind']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecution.targetKind is required');
        }
        return value;
      })(),
      targetRef: (() {
        final value = json['targetRef']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecution.targetRef is required');
        }
        return value;
      })(),
      inputPayload: json['inputPayload']?.toString(),
      outputPayload: json['outputPayload']?.toString(),
      state: (() {
        final value = json['state']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecution.state is required');
        }
        return value;
      })(),
      retryCount: (() {
        final value = json['retryCount'];
        if (value is! int) {
          throw FormatException('AutomationExecution.retryCount is required');
        }
        return value;
      })(),
      requestedAt: (() {
        final value = json['requestedAt']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecution.requestedAt is required');
        }
        return value;
      })(),
      completedAt: json['completedAt']?.toString(),
      failureReason: json['failureReason']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'principalKind': principalKind,
      'executionId': executionId,
      'triggerType': triggerType,
      'targetKind': targetKind,
      'targetRef': targetRef,
      'inputPayload': inputPayload,
      'outputPayload': outputPayload,
      'state': state,
      'retryCount': retryCount,
      'requestedAt': requestedAt,
      'completedAt': completedAt,
      'failureReason': failureReason,
    };
  }
}

class AutomationExecutionRequestResponse {
  final String tenantId;
  final String principalId;
  final String principalKind;
  final String executionId;
  final String triggerType;
  final String targetKind;
  final String targetRef;
  final String? inputPayload;
  final String? outputPayload;
  final String state;
  final int retryCount;
  final String requestedAt;
  final String? completedAt;
  final String? failureReason;
  final String requestKey;
  final String deliveryStatus;
  final String proofVersion;

  AutomationExecutionRequestResponse({
    required this.tenantId,
    required this.principalId,
    required this.principalKind,
    required this.executionId,
    required this.triggerType,
    required this.targetKind,
    required this.targetRef,
    this.inputPayload,
    this.outputPayload,
    required this.state,
    required this.retryCount,
    required this.requestedAt,
    this.completedAt,
    this.failureReason,
    required this.requestKey,
    required this.deliveryStatus,
    required this.proofVersion
  });

  factory AutomationExecutionRequestResponse.fromJson(Map<String, dynamic> json) {
    return AutomationExecutionRequestResponse(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionRequestResponse.tenantId is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionRequestResponse.principalId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionRequestResponse.principalKind is required');
        }
        return value;
      })(),
      executionId: (() {
        final value = json['executionId']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionRequestResponse.executionId is required');
        }
        return value;
      })(),
      triggerType: (() {
        final value = json['triggerType']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionRequestResponse.triggerType is required');
        }
        return value;
      })(),
      targetKind: (() {
        final value = json['targetKind']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionRequestResponse.targetKind is required');
        }
        return value;
      })(),
      targetRef: (() {
        final value = json['targetRef']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionRequestResponse.targetRef is required');
        }
        return value;
      })(),
      inputPayload: json['inputPayload']?.toString(),
      outputPayload: json['outputPayload']?.toString(),
      state: (() {
        final value = json['state']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionRequestResponse.state is required');
        }
        return value;
      })(),
      retryCount: (() {
        final value = json['retryCount'];
        if (value is! int) {
          throw FormatException('AutomationExecutionRequestResponse.retryCount is required');
        }
        return value;
      })(),
      requestedAt: (() {
        final value = json['requestedAt']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionRequestResponse.requestedAt is required');
        }
        return value;
      })(),
      completedAt: json['completedAt']?.toString(),
      failureReason: json['failureReason']?.toString(),
      requestKey: (() {
        final value = json['requestKey']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionRequestResponse.requestKey is required');
        }
        return value;
      })(),
      deliveryStatus: (() {
        final value = json['deliveryStatus']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionRequestResponse.deliveryStatus is required');
        }
        return value;
      })(),
      proofVersion: (() {
        final value = json['proofVersion']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionRequestResponse.proofVersion is required');
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
      'executionId': executionId,
      'triggerType': triggerType,
      'targetKind': targetKind,
      'targetRef': targetRef,
      'inputPayload': inputPayload,
      'outputPayload': outputPayload,
      'state': state,
      'retryCount': retryCount,
      'requestedAt': requestedAt,
      'completedAt': completedAt,
      'failureReason': failureReason,
      'requestKey': requestKey,
      'deliveryStatus': deliveryStatus,
      'proofVersion': proofVersion,
    };
  }
}

class CompleteAgentResponseRequest {
  final String frameSeq;
  final String? resultMessageId;

  CompleteAgentResponseRequest({
    required this.frameSeq,
    this.resultMessageId
  });

  factory CompleteAgentResponseRequest.fromJson(Map<String, dynamic> json) {
    return CompleteAgentResponseRequest(
      frameSeq: (() {
        final value = json['frameSeq']?.toString();
        if (value == null) {
          throw FormatException('CompleteAgentResponseRequest.frameSeq is required');
        }
        return value;
      })(),
      resultMessageId: json['resultMessageId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'frameSeq': frameSeq,
      'resultMessageId': resultMessageId,
    };
  }
}

class CompleteAgentToolCallRequest {
  final String resultPayload;

  CompleteAgentToolCallRequest({
    required this.resultPayload
  });

  factory CompleteAgentToolCallRequest.fromJson(Map<String, dynamic> json) {
    return CompleteAgentToolCallRequest(
      resultPayload: (() {
        final value = json['resultPayload']?.toString();
        if (value == null) {
          throw FormatException('CompleteAgentToolCallRequest.resultPayload is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'resultPayload': resultPayload,
    };
  }
}

class NotificationTask {
  final String tenantId;
  final String notificationId;
  final String sourceEventId;
  final String sourceEventType;
  final String category;
  final String channel;
  final String recipientId;
  final String recipientKind;
  final String status;
  final String? title;
  final String? body;
  final String? payload;
  final String requestedAt;
  final String? dispatchedAt;
  final String? failureReason;

  NotificationTask({
    required this.tenantId,
    required this.notificationId,
    required this.sourceEventId,
    required this.sourceEventType,
    required this.category,
    required this.channel,
    required this.recipientId,
    required this.recipientKind,
    required this.status,
    this.title,
    this.body,
    this.payload,
    required this.requestedAt,
    this.dispatchedAt,
    this.failureReason
  });

  factory NotificationTask.fromJson(Map<String, dynamic> json) {
    return NotificationTask(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('NotificationTask.tenantId is required');
        }
        return value;
      })(),
      notificationId: (() {
        final value = json['notificationId']?.toString();
        if (value == null) {
          throw FormatException('NotificationTask.notificationId is required');
        }
        return value;
      })(),
      sourceEventId: (() {
        final value = json['sourceEventId']?.toString();
        if (value == null) {
          throw FormatException('NotificationTask.sourceEventId is required');
        }
        return value;
      })(),
      sourceEventType: (() {
        final value = json['sourceEventType']?.toString();
        if (value == null) {
          throw FormatException('NotificationTask.sourceEventType is required');
        }
        return value;
      })(),
      category: (() {
        final value = json['category']?.toString();
        if (value == null) {
          throw FormatException('NotificationTask.category is required');
        }
        return value;
      })(),
      channel: (() {
        final value = json['channel']?.toString();
        if (value == null) {
          throw FormatException('NotificationTask.channel is required');
        }
        return value;
      })(),
      recipientId: (() {
        final value = json['recipientId']?.toString();
        if (value == null) {
          throw FormatException('NotificationTask.recipientId is required');
        }
        return value;
      })(),
      recipientKind: (() {
        final value = json['recipientKind']?.toString();
        if (value == null) {
          throw FormatException('NotificationTask.recipientKind is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status']?.toString();
        if (value == null) {
          throw FormatException('NotificationTask.status is required');
        }
        return value;
      })(),
      title: json['title']?.toString(),
      body: json['body']?.toString(),
      payload: json['payload']?.toString(),
      requestedAt: (() {
        final value = json['requestedAt']?.toString();
        if (value == null) {
          throw FormatException('NotificationTask.requestedAt is required');
        }
        return value;
      })(),
      dispatchedAt: json['dispatchedAt']?.toString(),
      failureReason: json['failureReason']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'notificationId': notificationId,
      'sourceEventId': sourceEventId,
      'sourceEventType': sourceEventType,
      'category': category,
      'channel': channel,
      'recipientId': recipientId,
      'recipientKind': recipientKind,
      'status': status,
      'title': title,
      'body': body,
      'payload': payload,
      'requestedAt': requestedAt,
      'dispatchedAt': dispatchedAt,
      'failureReason': failureReason,
    };
  }
}

class NotificationListResponse {
  final List<NotificationTask> items;

  NotificationListResponse({
    required this.items
  });

  factory NotificationListResponse.fromJson(Map<String, dynamic> json) {
    return NotificationListResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('NotificationListResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : NotificationTask.fromJson(map);
      })())
            .whereType<NotificationTask>()
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

class NotificationRequestResponse {
  final String tenantId;
  final String notificationId;
  final String sourceEventId;
  final String sourceEventType;
  final String category;
  final String channel;
  final String recipientId;
  final String recipientKind;
  final String status;
  final String? title;
  final String? body;
  final String? payload;
  final String requestedAt;
  final String? dispatchedAt;
  final String? failureReason;
  final String requestKey;
  final String deliveryStatus;
  final String proofVersion;

  NotificationRequestResponse({
    required this.tenantId,
    required this.notificationId,
    required this.sourceEventId,
    required this.sourceEventType,
    required this.category,
    required this.channel,
    required this.recipientId,
    required this.recipientKind,
    required this.status,
    this.title,
    this.body,
    this.payload,
    required this.requestedAt,
    this.dispatchedAt,
    this.failureReason,
    required this.requestKey,
    required this.deliveryStatus,
    required this.proofVersion
  });

  factory NotificationRequestResponse.fromJson(Map<String, dynamic> json) {
    return NotificationRequestResponse(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('NotificationRequestResponse.tenantId is required');
        }
        return value;
      })(),
      notificationId: (() {
        final value = json['notificationId']?.toString();
        if (value == null) {
          throw FormatException('NotificationRequestResponse.notificationId is required');
        }
        return value;
      })(),
      sourceEventId: (() {
        final value = json['sourceEventId']?.toString();
        if (value == null) {
          throw FormatException('NotificationRequestResponse.sourceEventId is required');
        }
        return value;
      })(),
      sourceEventType: (() {
        final value = json['sourceEventType']?.toString();
        if (value == null) {
          throw FormatException('NotificationRequestResponse.sourceEventType is required');
        }
        return value;
      })(),
      category: (() {
        final value = json['category']?.toString();
        if (value == null) {
          throw FormatException('NotificationRequestResponse.category is required');
        }
        return value;
      })(),
      channel: (() {
        final value = json['channel']?.toString();
        if (value == null) {
          throw FormatException('NotificationRequestResponse.channel is required');
        }
        return value;
      })(),
      recipientId: (() {
        final value = json['recipientId']?.toString();
        if (value == null) {
          throw FormatException('NotificationRequestResponse.recipientId is required');
        }
        return value;
      })(),
      recipientKind: (() {
        final value = json['recipientKind']?.toString();
        if (value == null) {
          throw FormatException('NotificationRequestResponse.recipientKind is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status']?.toString();
        if (value == null) {
          throw FormatException('NotificationRequestResponse.status is required');
        }
        return value;
      })(),
      title: json['title']?.toString(),
      body: json['body']?.toString(),
      payload: json['payload']?.toString(),
      requestedAt: (() {
        final value = json['requestedAt']?.toString();
        if (value == null) {
          throw FormatException('NotificationRequestResponse.requestedAt is required');
        }
        return value;
      })(),
      dispatchedAt: json['dispatchedAt']?.toString(),
      failureReason: json['failureReason']?.toString(),
      requestKey: (() {
        final value = json['requestKey']?.toString();
        if (value == null) {
          throw FormatException('NotificationRequestResponse.requestKey is required');
        }
        return value;
      })(),
      deliveryStatus: (() {
        final value = json['deliveryStatus']?.toString();
        if (value == null) {
          throw FormatException('NotificationRequestResponse.deliveryStatus is required');
        }
        return value;
      })(),
      proofVersion: (() {
        final value = json['proofVersion']?.toString();
        if (value == null) {
          throw FormatException('NotificationRequestResponse.proofVersion is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'notificationId': notificationId,
      'sourceEventId': sourceEventId,
      'sourceEventType': sourceEventType,
      'category': category,
      'channel': channel,
      'recipientId': recipientId,
      'recipientKind': recipientKind,
      'status': status,
      'title': title,
      'body': body,
      'payload': payload,
      'requestedAt': requestedAt,
      'dispatchedAt': dispatchedAt,
      'failureReason': failureReason,
      'requestKey': requestKey,
      'deliveryStatus': deliveryStatus,
      'proofVersion': proofVersion,
    };
  }
}

class RequestAgentToolCallRequest {
  final String executionId;
  final String toolCallId;
  final String toolName;
  final String argumentsPayload;

  RequestAgentToolCallRequest({
    required this.executionId,
    required this.toolCallId,
    required this.toolName,
    required this.argumentsPayload
  });

  factory RequestAgentToolCallRequest.fromJson(Map<String, dynamic> json) {
    return RequestAgentToolCallRequest(
      executionId: (() {
        final value = json['executionId']?.toString();
        if (value == null) {
          throw FormatException('RequestAgentToolCallRequest.executionId is required');
        }
        return value;
      })(),
      toolCallId: (() {
        final value = json['toolCallId']?.toString();
        if (value == null) {
          throw FormatException('RequestAgentToolCallRequest.toolCallId is required');
        }
        return value;
      })(),
      toolName: (() {
        final value = json['toolName']?.toString();
        if (value == null) {
          throw FormatException('RequestAgentToolCallRequest.toolName is required');
        }
        return value;
      })(),
      argumentsPayload: (() {
        final value = json['argumentsPayload']?.toString();
        if (value == null) {
          throw FormatException('RequestAgentToolCallRequest.argumentsPayload is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'executionId': executionId,
      'toolCallId': toolCallId,
      'toolName': toolName,
      'argumentsPayload': argumentsPayload,
    };
  }
}

class RequestAutomationExecution {
  final String executionId;
  final String triggerType;
  final String targetKind;
  final String targetRef;
  final String? inputPayload;

  RequestAutomationExecution({
    required this.executionId,
    required this.triggerType,
    required this.targetKind,
    required this.targetRef,
    this.inputPayload
  });

  factory RequestAutomationExecution.fromJson(Map<String, dynamic> json) {
    return RequestAutomationExecution(
      executionId: (() {
        final value = json['executionId']?.toString();
        if (value == null) {
          throw FormatException('RequestAutomationExecution.executionId is required');
        }
        return value;
      })(),
      triggerType: (() {
        final value = json['triggerType']?.toString();
        if (value == null) {
          throw FormatException('RequestAutomationExecution.triggerType is required');
        }
        return value;
      })(),
      targetKind: (() {
        final value = json['targetKind']?.toString();
        if (value == null) {
          throw FormatException('RequestAutomationExecution.targetKind is required');
        }
        return value;
      })(),
      targetRef: (() {
        final value = json['targetRef']?.toString();
        if (value == null) {
          throw FormatException('RequestAutomationExecution.targetRef is required');
        }
        return value;
      })(),
      inputPayload: json['inputPayload']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'executionId': executionId,
      'triggerType': triggerType,
      'targetKind': targetKind,
      'targetRef': targetRef,
      'inputPayload': inputPayload,
    };
  }
}

class RequestNotification {
  final String notificationId;
  final String sourceEventId;
  final String sourceEventType;
  final String category;
  final String channel;
  final String recipientId;
  final String recipientKind;
  final String? title;
  final String? body;
  final String? payload;

  RequestNotification({
    required this.notificationId,
    required this.sourceEventId,
    required this.sourceEventType,
    required this.category,
    required this.channel,
    required this.recipientId,
    required this.recipientKind,
    this.title,
    this.body,
    this.payload
  });

  factory RequestNotification.fromJson(Map<String, dynamic> json) {
    return RequestNotification(
      notificationId: (() {
        final value = json['notificationId']?.toString();
        if (value == null) {
          throw FormatException('RequestNotification.notificationId is required');
        }
        return value;
      })(),
      sourceEventId: (() {
        final value = json['sourceEventId']?.toString();
        if (value == null) {
          throw FormatException('RequestNotification.sourceEventId is required');
        }
        return value;
      })(),
      sourceEventType: (() {
        final value = json['sourceEventType']?.toString();
        if (value == null) {
          throw FormatException('RequestNotification.sourceEventType is required');
        }
        return value;
      })(),
      category: (() {
        final value = json['category']?.toString();
        if (value == null) {
          throw FormatException('RequestNotification.category is required');
        }
        return value;
      })(),
      channel: (() {
        final value = json['channel']?.toString();
        if (value == null) {
          throw FormatException('RequestNotification.channel is required');
        }
        return value;
      })(),
      recipientId: (() {
        final value = json['recipientId']?.toString();
        if (value == null) {
          throw FormatException('RequestNotification.recipientId is required');
        }
        return value;
      })(),
      recipientKind: (() {
        final value = json['recipientKind']?.toString();
        if (value == null) {
          throw FormatException('RequestNotification.recipientKind is required');
        }
        return value;
      })(),
      title: json['title']?.toString(),
      body: json['body']?.toString(),
      payload: json['payload']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'notificationId': notificationId,
      'sourceEventId': sourceEventId,
      'sourceEventType': sourceEventType,
      'category': category,
      'channel': channel,
      'recipientId': recipientId,
      'recipientKind': recipientKind,
      'title': title,
      'body': body,
      'payload': payload,
    };
  }
}

class StartAgentResponseRequest {
  final String executionId;
  final String streamId;
  final String streamType;
  final String conversationId;
  final String? schemaRef;
  final String? memberId;
  final AgentSubject agent;

  StartAgentResponseRequest({
    required this.executionId,
    required this.streamId,
    required this.streamType,
    required this.conversationId,
    this.schemaRef,
    this.memberId,
    required this.agent
  });

  factory StartAgentResponseRequest.fromJson(Map<String, dynamic> json) {
    return StartAgentResponseRequest(
      executionId: (() {
        final value = json['executionId']?.toString();
        if (value == null) {
          throw FormatException('StartAgentResponseRequest.executionId is required');
        }
        return value;
      })(),
      streamId: (() {
        final value = json['streamId']?.toString();
        if (value == null) {
          throw FormatException('StartAgentResponseRequest.streamId is required');
        }
        return value;
      })(),
      streamType: (() {
        final value = json['streamType']?.toString();
        if (value == null) {
          throw FormatException('StartAgentResponseRequest.streamType is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('StartAgentResponseRequest.conversationId is required');
        }
        return value;
      })(),
      schemaRef: json['schemaRef']?.toString(),
      memberId: json['memberId']?.toString(),
      agent: (() {
        final map = _sdkworkAsMap(json['agent']);
        if (map == null) {
          throw FormatException('StartAgentResponseRequest.agent is required');
        }
        return AgentSubject.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'executionId': executionId,
      'streamId': streamId,
      'streamType': streamType,
      'conversationId': conversationId,
      'schemaRef': schemaRef,
      'memberId': memberId,
      'agent': agent.toJson(),
    };
  }
}
