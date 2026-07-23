from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .portal_realtime_snapshot import PortalRealtimeSnapshot


@dataclass
class RealtimeRetrieveResponse:
    code: int
    data: Any
    trace_id: str
