from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class PortalRealtimeMetrics:
    client_route_window_count: str
    pending_event_count: str
    max_client_route_window_event_count: str
    client_route_window_capacity: str
    max_client_route_window_usage_permille: int
    capacity_trimmed_event_count: str
    oldest_pending_occurred_at: Optional[str] = None
