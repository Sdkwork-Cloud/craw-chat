from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .portal_conversation_operational_metrics import PortalConversationOperationalMetrics
    from .portal_data_availability import PortalDataAvailability
    from .portal_snapshot_meta import PortalSnapshotMeta


@dataclass
class PortalConversationSnapshot:
    meta: PortalSnapshotMeta
    availability: PortalDataAvailability
    metrics: Optional[PortalConversationOperationalMetrics] = None
