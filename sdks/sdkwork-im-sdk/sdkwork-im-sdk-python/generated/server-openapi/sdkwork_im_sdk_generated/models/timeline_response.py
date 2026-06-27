from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .timeline_view_entry import TimelineViewEntry


@dataclass
class TimelineResponse:
    items: List[TimelineViewEntry]
    has_more: bool
    next_after_seq: Optional[int] = None
