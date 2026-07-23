from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .portal_governance_snapshot import PortalGovernanceSnapshot


@dataclass
class GovernanceRetrieveResponse:
    code: int
    data: Any
    trace_id: str
