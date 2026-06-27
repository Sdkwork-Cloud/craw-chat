from typing import Any, Dict, List, Optional
from ..http_client import HttpClient

def _append_query_string(path: str, raw_query_string: str) -> str:
    query = raw_query_string.lstrip('?')
    if not query:
        return path
    separator = '&' if '?' in path else '?'
    return f"{path}{separator}{query}"





class AuditApi:
    """audit audit API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.records = AuditRecordsApi(client)
        self.export = AuditExportApi(client)


class AuditRecordsApi:
    """audit audit.records API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Dict[str, Any]:
        """List audit records"""
        return self._client.get(f"/backend/v3/api/audit/records")

    def create(self) -> Dict[str, Any]:
        """Record audit anchor"""
        return self._client.post(f"/backend/v3/api/audit/records")

class AuditExportApi:
    """audit audit.export API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Export audit bundle"""
        return self._client.get(f"/backend/v3/api/audit/export")
