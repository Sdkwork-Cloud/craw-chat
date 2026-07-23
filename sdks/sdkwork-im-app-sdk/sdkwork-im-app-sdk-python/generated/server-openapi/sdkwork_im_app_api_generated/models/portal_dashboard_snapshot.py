from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .portal_data_availability import PortalDataAvailability
    from .portal_operational_metrics import PortalOperationalMetrics
    from .portal_snapshot_meta import PortalSnapshotMeta


@dataclass
class PortalDashboardSnapshot:
    meta: PortalSnapshotMeta
    availability: PortalDataAvailability
    metrics: Optional[PortalOperationalMetrics] = None
