from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class PortalDataAvailability:
    state: str
    source: str
    complete: bool
    reason: Optional[str] = None
