from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RouteNodeLifecycle:
    drain_status: str
    node_id: str
    owned_route_count: int
    rebalance_state: str
