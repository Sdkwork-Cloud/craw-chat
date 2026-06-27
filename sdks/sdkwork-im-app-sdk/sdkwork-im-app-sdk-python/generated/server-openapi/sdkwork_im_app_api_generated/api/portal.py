from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import PortalWorkspaceView

def _append_query_string(path: str, raw_query_string: str) -> str:
    query = raw_query_string.lstrip('?')
    if not query:
        return path
    separator = '&' if '?' in path else '?'
    return f"{path}{separator}{query}"





class PortalApi:
    """portal portal API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.access = PortalAccessApi(client)
        self.automation = PortalAutomationApi(client)
        self.conversation_snapshot = PortalConversationSnapshotApi(client)
        self.dashboard = PortalDashboardApi(client)
        self.governance = PortalGovernanceApi(client)
        self.home = PortalHomeApi(client)
        self.media = PortalMediaApi(client)
        self.realtime = PortalRealtimeApi(client)
        self.workspace = PortalWorkspaceApi(client)


class PortalAccessApi:
    """portal portal.access API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Read the tenant portal sign-in snapshot"""
        return self._client.get(f"/app/v3/api/portal/access")

class PortalAutomationApi:
    """portal portal.automation API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Read the tenant automation snapshot"""
        return self._client.get(f"/app/v3/api/portal/automation")

class PortalConversationSnapshotApi:
    """portal portal.conversation_snapshot API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Read the tenant conversations snapshot"""
        return self._client.get(f"/app/v3/api/portal/conversations")

class PortalDashboardApi:
    """portal portal.dashboard API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Read the tenant dashboard snapshot"""
        return self._client.get(f"/app/v3/api/portal/dashboard")

class PortalGovernanceApi:
    """portal portal.governance API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Read the tenant governance snapshot"""
        return self._client.get(f"/app/v3/api/portal/governance")

class PortalHomeApi:
    """portal portal.home API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Read the tenant portal home snapshot"""
        return self._client.get(f"/app/v3/api/portal/home")

class PortalMediaApi:
    """portal portal.media API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Read the tenant media snapshot"""
        return self._client.get(f"/app/v3/api/portal/media")

class PortalRealtimeApi:
    """portal portal.realtime API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Read the tenant realtime snapshot"""
        return self._client.get(f"/app/v3/api/portal/realtime")

class PortalWorkspaceApi:
    """portal portal.workspace API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> PortalWorkspaceView:
        """Read the current tenant workspace snapshot"""
        return self._client.get(f"/app/v3/api/portal/workspace")
