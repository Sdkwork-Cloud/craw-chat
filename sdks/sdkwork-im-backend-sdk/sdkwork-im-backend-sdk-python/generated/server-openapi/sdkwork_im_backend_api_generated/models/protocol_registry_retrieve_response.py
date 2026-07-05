from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .protocol_registry_response import ProtocolRegistryResponse


@dataclass
class ProtocolRegistryRetrieveResponse:
    code: int
    data: Any
    trace_id: str
