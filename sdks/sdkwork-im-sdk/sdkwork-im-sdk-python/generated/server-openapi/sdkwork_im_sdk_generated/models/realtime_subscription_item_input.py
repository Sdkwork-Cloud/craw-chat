from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RealtimeSubscriptionItemInput:
    scope_type: str
    scope_id: str
    event_types: Optional[List[str]] = None
