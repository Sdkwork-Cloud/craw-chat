from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RolloutPolicyResponse:
    cell_selector: str
    operator_override: bool
    policy_id: str
    region_selector: str
    release_channel: str
    tenant_allowlist: List[str]
    traffic_percent: int
