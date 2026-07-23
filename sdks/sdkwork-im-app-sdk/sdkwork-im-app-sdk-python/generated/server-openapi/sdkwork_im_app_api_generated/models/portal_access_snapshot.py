from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .portal_audit_record_view import PortalAuditRecordView
    from .portal_data_availability import PortalDataAvailability
    from .portal_snapshot_meta import PortalSnapshotMeta


@dataclass
class PortalAccessSnapshot:
    meta: PortalSnapshotMeta
    availability: PortalDataAvailability
    recent_items: List[PortalAuditRecordView]
    has_more: bool
    tenant_id: Optional[str] = None
    principal_id: Optional[str] = None
