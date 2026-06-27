from typing import Any, Dict, List, Optional
from ..http_client import HttpClient

def _append_query_string(path: str, raw_query_string: str) -> str:
    query = raw_query_string.lstrip('?')
    if not query:
        return path
    separator = '&' if '?' in path else '?'
    return f"{path}{separator}{query}"





class AutomationApi:
    """automation automation API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.governance = AutomationGovernanceApi(client)


class AutomationGovernanceApi:
    """automation automation.governance API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Retrieve automation governance"""
        return self._client.get(f"/backend/v3/api/automation/governance")
