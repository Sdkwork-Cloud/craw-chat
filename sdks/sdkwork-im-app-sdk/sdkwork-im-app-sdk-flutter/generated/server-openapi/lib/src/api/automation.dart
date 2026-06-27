import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class AutomationApi {
  final HttpClient _client;

  AutomationApi(this._client);

  /// Start an agent response stream
  Future<StreamSession?> agentResponsesCreate(StartAgentResponseRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/automation/agent_responses'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StreamSession.fromJson(map);
    })();
  }

  /// Complete an agent response stream
  Future<StreamSession?> agentResponsesComplete(String streamId, CompleteAgentResponseRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/automation/agent_responses/${serializePathParameter(streamId, const PathParameterSpec('streamId', 'simple', false))}/complete'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StreamSession.fromJson(map);
    })();
  }

  /// Append a frame to an agent response stream
  Future<StreamFrame?> agentResponsesFramesCreate(String streamId, AppendAgentResponseDeltaRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/automation/agent_responses/${serializePathParameter(streamId, const PathParameterSpec('streamId', 'simple', false))}/frames'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StreamFrame.fromJson(map);
    })();
  }

  /// Request an agent tool call
  Future<AgentToolCall?> agentToolCallsCreate(RequestAgentToolCallRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/automation/agent_tool_calls'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : AgentToolCall.fromJson(map);
    })();
  }

  /// Request an automation execution
  Future<AutomationExecutionRequestResponse?> executionsCreate(RequestAutomationExecution body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/automation/executions'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : AutomationExecutionRequestResponse.fromJson(map);
    })();
  }

  /// Get an automation execution
  Future<AutomationExecution?> executionsRetrieve(String executionId) async {
    final response = await _client.get(ApiPaths.appPath('/automation/executions/${serializePathParameter(executionId, const PathParameterSpec('executionId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : AutomationExecution.fromJson(map);
    })();
  }

  /// Complete an agent tool call
  Future<AgentToolCall?> agentToolCallsComplete(String executionId, String toolCallId, CompleteAgentToolCallRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/automation/executions/${serializePathParameter(executionId, const PathParameterSpec('executionId', 'simple', false))}/agent_tool_calls/${serializePathParameter(toolCallId, const PathParameterSpec('toolCallId', 'simple', false))}/complete'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : AgentToolCall.fromJson(map);
    })();
  }
}

class PathParameterSpec {
  final String name;
  final String style;
  final bool explode;

  const PathParameterSpec(this.name, this.style, this.explode);
}

String serializePathParameter(dynamic value, PathParameterSpec spec) {
  if (value == null) return '';
  final style = spec.style.trim().isEmpty ? 'simple' : spec.style;
  if (value is Iterable) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (value is Map) {
    return serializePathObject(spec.name, value, style, spec.explode);
  }
  return pathPrimitivePrefix(spec.name, style) + Uri.encodeComponent(value.toString());
}

String serializePathArray(String name, Iterable values, String style, bool explode) {
  final serialized = values.where((item) => item != null).map((item) => Uri.encodeComponent(item.toString())).toList();
  if (serialized.isEmpty) return pathPrefix(name, style);
  if (style == 'matrix') {
    if (explode) {
      return serialized.map((item) => ';$name=$item').join();
    }
    return ';$name=${serialized.join(',')}';
  }
  final separator = explode ? '.' : ',';
  return pathPrefix(name, style) + serialized.join(separator);
}

String serializePathObject(String name, Map values, String style, bool explode) {
  final entries = <String>[];
  final exploded = <String>[];
  values.forEach((key, value) {
    if (value == null) return;
    final escapedKey = Uri.encodeComponent(key.toString());
    final escapedValue = Uri.encodeComponent(value.toString());
    if (explode) {
      if (style == 'matrix') {
        exploded.add(';$escapedKey=$escapedValue');
      } else {
        exploded.add('$escapedKey=$escapedValue');
      }
    } else {
      entries.add(escapedKey);
      entries.add(escapedValue);
    }
  });
  if (style == 'matrix') {
    if (explode) return exploded.join();
    return ';$name=${entries.join(',')}';
  }
  if (explode) {
    final separator = style == 'label' ? '.' : ',';
    return pathPrefix(name, style) + exploded.join(separator);
  }
  return pathPrefix(name, style) + entries.join(',');
}

String pathPrefix(String name, String style) {
  if (style == 'label') return '.';
  if (style == 'matrix') return ';$name';
  return '';
}

String pathPrimitivePrefix(String name, String style) {
  return style == 'matrix' ? ';$name=' : pathPrefix(name, style);
}
