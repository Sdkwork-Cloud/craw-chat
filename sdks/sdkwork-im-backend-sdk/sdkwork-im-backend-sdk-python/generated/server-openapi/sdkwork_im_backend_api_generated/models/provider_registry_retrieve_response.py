from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .provider_registry_snapshot_response import ProviderRegistrySnapshotResponse


@dataclass
class ProviderRegistryRetrieveResponse:
    code: int
    data: Any
    trace_id: str
