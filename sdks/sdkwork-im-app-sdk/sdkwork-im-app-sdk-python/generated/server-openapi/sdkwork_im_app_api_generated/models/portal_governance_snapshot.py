from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .portal_data_availability import PortalDataAvailability
    from .portal_governance_risk_sample import PortalGovernanceRiskSample
    from .portal_snapshot_meta import PortalSnapshotMeta


@dataclass
class PortalGovernanceSnapshot:
    meta: PortalSnapshotMeta
    availability: PortalDataAvailability
    sampled_event_count: str
    risk_sample: PortalGovernanceRiskSample
