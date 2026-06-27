from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class PortalWorkspaceView:
    name: str
    slug: str
    tier: str
    region: str
    support_plan: str
    seats: int
    active_brands: int
    uptime: str
