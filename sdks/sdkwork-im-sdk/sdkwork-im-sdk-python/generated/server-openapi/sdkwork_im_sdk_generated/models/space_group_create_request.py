from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SpaceGroupCreateRequest:
    group_name: str
    description: Optional[str] = None
