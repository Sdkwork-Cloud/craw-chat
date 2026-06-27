from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RouteMigrationResult:
    migrated_route_count: int
    source_drain_status: str
    source_node_id: str
    source_rebalance_state: str
    target_drain_status: str
    target_node_id: str
    target_rebalance_state: str
