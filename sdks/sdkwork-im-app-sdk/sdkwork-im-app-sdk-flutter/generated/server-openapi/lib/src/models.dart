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
  final String? detail;
  final String? instance;
  final int code;
  final String traceId;
  final List<FieldError>? errors;

  ProblemDetail({
    required this.type,
    required this.title,
    required this.status,
    this.detail,
    this.instance,
    required this.code,
    required this.traceId,
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
      'errors': errors?.map((item) => item.toJson()).toList(),
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

  FieldError({
    required this.field,
    required this.message,
    this.code
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
      code: json['code'] is int ? json['code'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'field': field,
      'message': message,
      'code': code,
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

class AutomationAgentResponsesCreateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  AutomationAgentResponsesCreateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory AutomationAgentResponsesCreateResponse.fromJson(Map<String, dynamic> json) {
    return AutomationAgentResponsesCreateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('AutomationAgentResponsesCreateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('AutomationAgentResponsesCreateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('AutomationAgentResponsesCreateResponse.traceId is required');
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

class AutomationAgentResponsesCompleteResponse {
  final int code;
  final dynamic data;
  final String traceId;

  AutomationAgentResponsesCompleteResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory AutomationAgentResponsesCompleteResponse.fromJson(Map<String, dynamic> json) {
    return AutomationAgentResponsesCompleteResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('AutomationAgentResponsesCompleteResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('AutomationAgentResponsesCompleteResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('AutomationAgentResponsesCompleteResponse.traceId is required');
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

class AutomationAgentResponsesFramesCreateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  AutomationAgentResponsesFramesCreateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory AutomationAgentResponsesFramesCreateResponse.fromJson(Map<String, dynamic> json) {
    return AutomationAgentResponsesFramesCreateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('AutomationAgentResponsesFramesCreateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('AutomationAgentResponsesFramesCreateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('AutomationAgentResponsesFramesCreateResponse.traceId is required');
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

class AutomationAgentToolCallsCreateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  AutomationAgentToolCallsCreateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory AutomationAgentToolCallsCreateResponse.fromJson(Map<String, dynamic> json) {
    return AutomationAgentToolCallsCreateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('AutomationAgentToolCallsCreateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('AutomationAgentToolCallsCreateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('AutomationAgentToolCallsCreateResponse.traceId is required');
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

class AutomationExecutionsCreateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  AutomationExecutionsCreateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory AutomationExecutionsCreateResponse.fromJson(Map<String, dynamic> json) {
    return AutomationExecutionsCreateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('AutomationExecutionsCreateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('AutomationExecutionsCreateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionsCreateResponse.traceId is required');
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

class AutomationExecutionsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  AutomationExecutionsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory AutomationExecutionsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return AutomationExecutionsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('AutomationExecutionsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('AutomationExecutionsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('AutomationExecutionsRetrieveResponse.traceId is required');
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

class AutomationAgentToolCallsCompleteResponse {
  final int code;
  final dynamic data;
  final String traceId;

  AutomationAgentToolCallsCompleteResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory AutomationAgentToolCallsCompleteResponse.fromJson(Map<String, dynamic> json) {
    return AutomationAgentToolCallsCompleteResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('AutomationAgentToolCallsCompleteResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('AutomationAgentToolCallsCompleteResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('AutomationAgentToolCallsCompleteResponse.traceId is required');
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

class NotificationsListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  NotificationsListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory NotificationsListResponse.fromJson(Map<String, dynamic> json) {
    return NotificationsListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('NotificationsListResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('NotificationsListResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('NotificationsListResponse.traceId is required');
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

class NotificationsRequestsCreateResponse {
  final int code;
  final dynamic data;
  final String traceId;

  NotificationsRequestsCreateResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory NotificationsRequestsCreateResponse.fromJson(Map<String, dynamic> json) {
    return NotificationsRequestsCreateResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('NotificationsRequestsCreateResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('NotificationsRequestsCreateResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('NotificationsRequestsCreateResponse.traceId is required');
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

class NotificationsRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  NotificationsRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory NotificationsRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return NotificationsRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('NotificationsRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('NotificationsRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('NotificationsRetrieveResponse.traceId is required');
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

class AccessRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  AccessRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory AccessRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return AccessRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('AccessRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('AccessRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('AccessRetrieveResponse.traceId is required');
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

class AutomationRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  AutomationRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory AutomationRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return AutomationRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('AutomationRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('AutomationRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('AutomationRetrieveResponse.traceId is required');
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

class ConversationSnapshotRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  ConversationSnapshotRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory ConversationSnapshotRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return ConversationSnapshotRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ConversationSnapshotRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('ConversationSnapshotRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ConversationSnapshotRetrieveResponse.traceId is required');
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

class DashboardRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  DashboardRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory DashboardRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return DashboardRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('DashboardRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('DashboardRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('DashboardRetrieveResponse.traceId is required');
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

class HomeRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  HomeRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory HomeRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return HomeRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('HomeRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('HomeRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('HomeRetrieveResponse.traceId is required');
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

class MediaRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  MediaRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory MediaRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return MediaRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('MediaRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('MediaRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('MediaRetrieveResponse.traceId is required');
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

class RealtimeRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  RealtimeRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory RealtimeRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return RealtimeRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('RealtimeRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('RealtimeRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('RealtimeRetrieveResponse.traceId is required');
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

class WorkspaceRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  WorkspaceRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory WorkspaceRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return WorkspaceRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('WorkspaceRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('WorkspaceRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('WorkspaceRetrieveResponse.traceId is required');
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

class MediaHealthRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  MediaHealthRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory MediaHealthRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return MediaHealthRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('MediaHealthRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('MediaHealthRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('MediaHealthRetrieveResponse.traceId is required');
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

class PrincipalProfileHealthRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  PrincipalProfileHealthRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory PrincipalProfileHealthRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return PrincipalProfileHealthRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('PrincipalProfileHealthRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('PrincipalProfileHealthRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('PrincipalProfileHealthRetrieveResponse.traceId is required');
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
