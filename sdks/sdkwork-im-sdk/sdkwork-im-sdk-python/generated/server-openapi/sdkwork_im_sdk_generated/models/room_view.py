from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RoomView:
    room_id: str
    room_kind: str
    conversation_id: str
    active_member_count: int
    max_members: int
