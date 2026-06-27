from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class EstablishExternalConnectionRequest:
    connection_id: str
    connection_kind: str
    established_at: str
    event_id: str
    external_tenant_id: str
    external_org_name: Optional[str] = None
