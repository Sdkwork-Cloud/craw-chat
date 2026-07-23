from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class PortalWorkspaceView:
    name: str
    slug: str
    environment: str
    tier: Optional[str] = None
    region: Optional[str] = None
    support_plan: Optional[str] = None
    seats: Optional[str] = None
    active_brands: Optional[str] = None
