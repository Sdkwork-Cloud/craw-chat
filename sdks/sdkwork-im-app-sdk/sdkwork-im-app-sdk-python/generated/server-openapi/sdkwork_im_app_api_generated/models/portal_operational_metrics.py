from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class PortalOperationalMetrics:
    client_route_window_count: str
    pending_realtime_event_count: str
