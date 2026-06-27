from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest, PostRtcSignalRequest, RtcParticipantCredential, RtcSession, RtcSessionMutationResponse, RtcSignalEvent, UpdateRtcSessionRequest

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





class CallsApi:
    """calls calls API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.sessions = CallsSessionsApi(client)


class CallsSessionsApi:
    """calls calls.sessions API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.signals = CallsSessionsSignalsApi(client)
        self.credentials = CallsSessionsCredentialsApi(client)


    def create(self, body: CreateRtcSessionRequest) -> RtcSessionMutationResponse:
        """Create an IM call signaling session"""
        return self._client.post(f"/im/v3/api/calls/sessions", json=body)

    def retrieve(self, rtc_session_id: str) -> RtcSession:
        """Retrieve IM call signaling session state"""
        return self._client.get(f"/im/v3/api/calls/sessions/{serialize_path_parameter(rtc_session_id, {'name': 'rtcSessionId', 'style': 'simple', 'explode': False})}")

    def create_invite(self, rtc_session_id: str, body: InviteRtcSessionRequest) -> RtcSessionMutationResponse:
        """Invite participants into an IM call signaling session"""
        return self._client.post(f"/im/v3/api/calls/sessions/{serialize_path_parameter(rtc_session_id, {'name': 'rtcSessionId', 'style': 'simple', 'explode': False})}/invite", json=body)

    def create_accept(self, rtc_session_id: str, body: UpdateRtcSessionRequest) -> RtcSessionMutationResponse:
        """Accept an IM call signaling session"""
        return self._client.post(f"/im/v3/api/calls/sessions/{serialize_path_parameter(rtc_session_id, {'name': 'rtcSessionId', 'style': 'simple', 'explode': False})}/accept", json=body)

    def create_reject(self, rtc_session_id: str, body: UpdateRtcSessionRequest) -> RtcSessionMutationResponse:
        """Reject an IM call signaling session"""
        return self._client.post(f"/im/v3/api/calls/sessions/{serialize_path_parameter(rtc_session_id, {'name': 'rtcSessionId', 'style': 'simple', 'explode': False})}/reject", json=body)

    def create_end(self, rtc_session_id: str, body: UpdateRtcSessionRequest) -> RtcSessionMutationResponse:
        """End an IM call signaling session"""
        return self._client.post(f"/im/v3/api/calls/sessions/{serialize_path_parameter(rtc_session_id, {'name': 'rtcSessionId', 'style': 'simple', 'explode': False})}/end", json=body)

class CallsSessionsSignalsApi:
    """calls calls.sessions.signals API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, rtc_session_id: str, body: PostRtcSignalRequest) -> RtcSignalEvent:
        """Post an IM call signaling event"""
        return self._client.post(f"/im/v3/api/calls/sessions/{serialize_path_parameter(rtc_session_id, {'name': 'rtcSessionId', 'style': 'simple', 'explode': False})}/signals", json=body)

class CallsSessionsCredentialsApi:
    """calls calls.sessions.credentials API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, rtc_session_id: str, body: IssueRtcParticipantCredentialRequest) -> RtcParticipantCredential:
        """Issue an RTC media participant credential for an IM call"""
        return self._client.post(f"/im/v3/api/calls/sessions/{serialize_path_parameter(rtc_session_id, {'name': 'rtcSessionId', 'style': 'simple', 'explode': False})}/credentials", json=body)
