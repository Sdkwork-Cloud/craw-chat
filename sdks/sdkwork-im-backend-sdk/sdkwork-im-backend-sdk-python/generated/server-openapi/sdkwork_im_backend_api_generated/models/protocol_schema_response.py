from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ProtocolSchemaResponse:
    binding_protocols: List[str]
    kind: str
    required_capabilities: List[str]
    schema: str
    stage: str
    supported_consumers: List[str]
