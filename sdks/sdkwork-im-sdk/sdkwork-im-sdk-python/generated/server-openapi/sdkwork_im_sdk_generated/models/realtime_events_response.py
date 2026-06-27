from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .realtime_event_view import RealtimeEventView


@dataclass
class RealtimeEventsResponse:
    items: List[RealtimeEventView]
    has_more: bool
    next_cursor: Optional[str] = None
