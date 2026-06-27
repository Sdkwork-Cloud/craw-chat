from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import PresenceHeartbeatRequest, PresenceView

def _append_query_string(path: str, raw_query_string: str) -> str:
    query = raw_query_string.lstrip('?')
    if not query:
        return path
    separator = '&' if '?' in path else '?'
    return f"{path}{separator}{query}"





class PresenceApi:
    """presence presence API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.heartbeat = PresenceHeartbeatApi(client)
        self.me = PresenceMeApi(client)


class PresenceHeartbeatApi:
    """presence presence.heartbeat API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: PresenceHeartbeatRequest) -> PresenceView:
        """Publish current client route presence heartbeat"""
        return self._client.post(f"/im/v3/api/presence/heartbeat", json=body)

class PresenceMeApi:
    """presence presence.me API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> PresenceView:
        """Retrieve current principal presence"""
        return self._client.get(f"/im/v3/api/presence/me")
