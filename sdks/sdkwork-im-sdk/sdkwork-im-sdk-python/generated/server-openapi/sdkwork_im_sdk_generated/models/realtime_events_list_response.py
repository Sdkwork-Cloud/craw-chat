from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .realtime_event_view import RealtimeEventView


@dataclass
class RealtimeEventsListResponse:
    code: int
    data: Any
    trace_id: str
