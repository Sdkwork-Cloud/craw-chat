from typing import Any, Dict, List, Optional
from ..http_client import HttpClient

def _append_query_string(path: str, raw_query_string: str) -> str:
    query = raw_query_string.lstrip('?')
    if not query:
        return path
    separator = '&' if '?' in path else '?'
    return f"{path}{separator}{query}"





class ProviderApi:
    """provider provider API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.media_health = ProviderMediaHealthApi(client)
        self.principal_profile_health = ProviderPrincipalProfileHealthApi(client)


class ProviderMediaHealthApi:
    """provider provider.media_health API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Retrieve media provider health"""
        return self._client.get(f"/app/v3/api/media/provider_health")

class ProviderPrincipalProfileHealthApi:
    """provider provider.principal_profile_health API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Retrieve principal-profile provider health"""
        return self._client.get(f"/app/v3/api/principal/profiles/provider_health")
