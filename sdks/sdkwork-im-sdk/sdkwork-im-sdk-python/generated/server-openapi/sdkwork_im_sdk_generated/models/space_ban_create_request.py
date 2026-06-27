from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SpaceBanCreateRequest:
    user_id: str
    reason: Optional[str] = None
