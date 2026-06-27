from typing import Any, Dict, List, Optional
from ..http_client import HttpClient

def _append_query_string(path: str, raw_query_string: str) -> str:
    query = raw_query_string.lstrip('?')
    if not query:
        return path
    separator = '&' if '?' in path else '?'
    return f"{path}{separator}{query}"





class OpsApi:
    """ops ops API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.health = OpsHealthApi(client)
        self.cluster = OpsClusterApi(client)
        self.lag = OpsLagApi(client)
        self.replay_status = OpsReplayStatusApi(client)
        self.commercial_readiness = OpsCommercialReadinessApi(client)
        self.runtime_dir = OpsRuntimeDirApi(client)
        self.provider_bindings = OpsProviderBindingsApi(client)
        self.diagnostics = OpsDiagnosticsApi(client)


class OpsHealthApi:
    """ops ops.health API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Retrieve ops health"""
        return self._client.get(f"/backend/v3/api/ops/health")

class OpsClusterApi:
    """ops ops.cluster API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Retrieve cluster state"""
        return self._client.get(f"/backend/v3/api/ops/cluster")

class OpsLagApi:
    """ops ops.lag API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Retrieve projection lag"""
        return self._client.get(f"/backend/v3/api/ops/lag")

class OpsReplayStatusApi:
    """ops ops.replay_status API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Retrieve replay status"""
        return self._client.get(f"/backend/v3/api/ops/replay_status")

class OpsCommercialReadinessApi:
    """ops ops.commercial_readiness API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Retrieve commercial readiness"""
        return self._client.get(f"/backend/v3/api/ops/commercial_readiness")

class OpsRuntimeDirApi:
    """ops ops.runtime_dir API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Inspect runtime directory"""
        return self._client.get(f"/backend/v3/api/ops/runtime_dir")

class OpsProviderBindingsApi:
    """ops ops.provider_bindings API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.drift = OpsProviderBindingsDriftApi(client)


    def list(self) -> Dict[str, Any]:
        """List provider bindings"""
        return self._client.get(f"/backend/v3/api/ops/provider_bindings")

class OpsProviderBindingsDriftApi:
    """ops ops.provider_bindings.drift API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Dict[str, Any]:
        """Retrieve provider binding drift"""
        return self._client.get(f"/backend/v3/api/ops/provider_bindings/drift")

class OpsDiagnosticsApi:
    """ops ops.diagnostics API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Retrieve diagnostics"""
        return self._client.get(f"/backend/v3/api/ops/diagnostics")
