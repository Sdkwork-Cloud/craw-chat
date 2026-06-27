from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .space_ban_view import SpaceBanView


@dataclass
class SpaceBanListResponse:
    items: List[SpaceBanView]
