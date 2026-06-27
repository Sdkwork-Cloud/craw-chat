from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .client_compatibility_response import ClientCompatibilityResponse
    from .protocol_schema_response import ProtocolSchemaResponse


@dataclass
class ProtocolRegistryResponse:
    bindings: List[str]
    codecs: List[str]
    compatibility_matrix: List[ClientCompatibilityResponse]
    protocol_version: str
    schemas: List[ProtocolSchemaResponse]
