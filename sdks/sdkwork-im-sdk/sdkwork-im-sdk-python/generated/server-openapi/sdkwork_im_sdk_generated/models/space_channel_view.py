from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SpaceChannelView:
    channel_id: str
    channel_name: str
    channel_type: str
