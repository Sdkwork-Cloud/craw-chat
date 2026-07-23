from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class PortalGovernanceRiskSample:
    critical_count: str
    high_count: str
    warning_count: str
    informational_count: str
