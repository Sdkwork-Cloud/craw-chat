from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SpaceCreateRequest:
    space_name: str
    space_type: str
    description: Optional[str] = None
