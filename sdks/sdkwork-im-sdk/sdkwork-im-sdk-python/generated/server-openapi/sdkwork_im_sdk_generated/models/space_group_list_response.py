from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .space_group_view import SpaceGroupView


@dataclass
class SpaceGroupListResponse:
    items: List[SpaceGroupView]
