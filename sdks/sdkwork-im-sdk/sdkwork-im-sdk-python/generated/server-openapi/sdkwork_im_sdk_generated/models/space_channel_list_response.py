from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .space_channel_view import SpaceChannelView


@dataclass
class SpaceChannelListResponse:
    items: List[SpaceChannelView]
