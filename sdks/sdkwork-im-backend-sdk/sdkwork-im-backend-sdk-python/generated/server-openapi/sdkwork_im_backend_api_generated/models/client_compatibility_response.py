from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ClientCompatibilityResponse:
    blocked_experimental_capabilities: List[str]
    client_type: str
    minimum_protocol_version: str
    supported_bindings: List[str]
    supported_capabilities: List[str]
    supported_codecs: List[str]
