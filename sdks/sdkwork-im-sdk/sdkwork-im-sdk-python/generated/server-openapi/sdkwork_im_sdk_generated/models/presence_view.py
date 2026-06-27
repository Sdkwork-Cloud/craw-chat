from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class PresenceView:
    tenant_id: str
    principal_id: str
    principal_kind: str
    device_id: str
    status: str
    updated_at: str
