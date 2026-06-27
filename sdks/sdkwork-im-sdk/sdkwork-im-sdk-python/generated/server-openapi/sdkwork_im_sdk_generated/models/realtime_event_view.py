from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RealtimeEventView:
    event_id: str
    scope: str
    scope_id: str
    event_type: str
    occurred_at: str
    payload: Optional[str] = None
