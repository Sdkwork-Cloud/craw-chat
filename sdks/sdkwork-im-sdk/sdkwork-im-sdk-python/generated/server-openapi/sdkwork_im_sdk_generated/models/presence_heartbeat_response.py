from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .presence_view import PresenceView


@dataclass
class PresenceHeartbeatResponse:
    code: int
    data: Any
    trace_id: str
