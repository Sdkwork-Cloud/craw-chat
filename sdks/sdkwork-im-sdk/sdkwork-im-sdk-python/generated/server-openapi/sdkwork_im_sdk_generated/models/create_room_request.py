from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class CreateRoomRequest:
    conversation_id: str
    room_id: str
    room_kind: str
