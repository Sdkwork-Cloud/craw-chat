from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class PortalAuditRecordView:
    record_id: str
    action: str
    actor_id: str
    recorded_at: str
    severity: str
