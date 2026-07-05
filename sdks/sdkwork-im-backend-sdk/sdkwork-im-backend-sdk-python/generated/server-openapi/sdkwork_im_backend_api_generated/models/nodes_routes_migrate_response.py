from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .route_migration_result import RouteMigrationResult


@dataclass
class NodesRoutesMigrateResponse:
    code: int
    data: Any
    trace_id: str
