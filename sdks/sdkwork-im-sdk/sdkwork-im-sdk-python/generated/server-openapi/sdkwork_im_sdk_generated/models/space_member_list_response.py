from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .space_member_view import SpaceMemberView


@dataclass
class SpaceMemberListResponse:
    items: List[SpaceMemberView]
