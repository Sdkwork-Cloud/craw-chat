from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class CapabilityProfileResponse:
    enabled_capabilities: List[str]
    experimental_capabilities: List[str]
    profile_id: str
    release_channel: str
