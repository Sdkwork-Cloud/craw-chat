from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import AgentToolCall, AppendAgentResponseDeltaRequest, AutomationExecution, AutomationExecutionRequestResponse, CompleteAgentResponseRequest, CompleteAgentToolCallRequest, RequestAgentToolCallRequest, RequestAutomationExecution, StartAgentResponseRequest, StreamFrame, StreamSession

def _append_query_string(path: str, raw_query_string: str) -> str:
    query = raw_query_string.lstrip('?')
    if not query:
        return path
    separator = '&' if '?' in path else '?'
    return f"{path}{separator}{query}"

def serialize_path_parameter(value: Any, spec: Dict[str, Any]) -> str:
    if value is None:
        return ''

    style = str(spec.get('style') or 'simple')
    name = str(spec.get('name') or '')
    explode = bool(spec.get('explode'))
    if isinstance(value, (list, tuple)):
        return serialize_path_array(name, value, style, explode)
    if isinstance(value, dict):
        return serialize_path_object(name, value, style, explode)
    return path_prefix(name, style) + encode_path_value(serialize_path_primitive(value))


def serialize_path_array(name: str, values: Any, style: str, explode: bool) -> str:
    serialized = [encode_path_value(serialize_path_primitive(item)) for item in values if item is not None]
    if not serialized:
        return path_prefix(name, style)
    if style == 'matrix':
        return ''.join(f";{name}={item}" for item in serialized) if explode else f";{name}={','.join(serialized)}"
    return path_prefix(name, style) + ('.' if explode else ',').join(serialized)


def serialize_path_object(name: str, value: Dict[str, Any], style: str, explode: bool) -> str:
    entries = [(key, entry_value) for key, entry_value in value.items() if entry_value is not None]
    if not entries:
        return path_prefix(name, style)
    if style == 'matrix':
        if explode:
            return ''.join(f";{encode_path_value(str(key))}={encode_path_value(serialize_path_primitive(entry_value))}" for key, entry_value in entries)
        serialized = ','.join(item for key, entry_value in entries for item in (encode_path_value(str(key)), encode_path_value(serialize_path_primitive(entry_value))))
        return f";{name}={serialized}"
    if explode:
        separator = '.' if style == 'label' else ','
        serialized = separator.join(f"{encode_path_value(str(key))}={encode_path_value(serialize_path_primitive(entry_value))}" for key, entry_value in entries)
    else:
        serialized = ','.join(item for key, entry_value in entries for item in (encode_path_value(str(key)), encode_path_value(serialize_path_primitive(entry_value))))
    return path_prefix(name, style) + serialized


def path_prefix(name: str, style: str) -> str:
    if style == 'label':
        return '.'
    if style == 'matrix':
        return f";{name}"
    return ''


def encode_path_value(value: str) -> str:
    from urllib.parse import quote

    return quote(value, safe='')


def serialize_path_primitive(value: Any) -> str:
    if isinstance(value, dict):
        import json

        return json.dumps(value, separators=(',', ':'))
    return str(value)





class AutomationApi:
    """automation automation API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.agent_responses = AutomationAgentResponsesApi(client)
        self.agent_tool_calls = AutomationAgentToolCallsApi(client)
        self.executions = AutomationExecutionsApi(client)


class AutomationAgentResponsesApi:
    """automation automation.agent_responses API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.frames = AutomationAgentResponsesFramesApi(client)


    def create(self, body: StartAgentResponseRequest) -> StreamSession:
        """Start an agent response stream"""
        return self._client.post(f"/app/v3/api/automation/agent_responses", json=body)

    def complete(self, stream_id: str, body: CompleteAgentResponseRequest) -> StreamSession:
        """Complete an agent response stream"""
        return self._client.post(f"/app/v3/api/automation/agent_responses/{serialize_path_parameter(stream_id, {'name': 'streamId', 'style': 'simple', 'explode': False})}/complete", json=body)

class AutomationAgentResponsesFramesApi:
    """automation automation.agent_responses.frames API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, stream_id: str, body: AppendAgentResponseDeltaRequest) -> StreamFrame:
        """Append a frame to an agent response stream"""
        return self._client.post(f"/app/v3/api/automation/agent_responses/{serialize_path_parameter(stream_id, {'name': 'streamId', 'style': 'simple', 'explode': False})}/frames", json=body)

class AutomationAgentToolCallsApi:
    """automation automation.agent_tool_calls API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: RequestAgentToolCallRequest) -> AgentToolCall:
        """Request an agent tool call"""
        return self._client.post(f"/app/v3/api/automation/agent_tool_calls", json=body)

    def complete(self, execution_id: str, tool_call_id: str, body: CompleteAgentToolCallRequest) -> AgentToolCall:
        """Complete an agent tool call"""
        return self._client.post(f"/app/v3/api/automation/executions/{serialize_path_parameter(execution_id, {'name': 'executionId', 'style': 'simple', 'explode': False})}/agent_tool_calls/{serialize_path_parameter(tool_call_id, {'name': 'toolCallId', 'style': 'simple', 'explode': False})}/complete", json=body)

class AutomationExecutionsApi:
    """automation automation.executions API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: RequestAutomationExecution) -> AutomationExecutionRequestResponse:
        """Request an automation execution"""
        return self._client.post(f"/app/v3/api/automation/executions", json=body)

    def retrieve(self, execution_id: str) -> AutomationExecution:
        """Get an automation execution"""
        return self._client.get(f"/app/v3/api/automation/executions/{serialize_path_parameter(execution_id, {'name': 'executionId', 'style': 'simple', 'explode': False})}")
