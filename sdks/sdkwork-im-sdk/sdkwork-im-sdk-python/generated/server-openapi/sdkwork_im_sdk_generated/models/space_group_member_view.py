from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SpaceGroupMemberView:
    user_id: str
    role: str
    joined_at: str
    nickname: Optional[str] = None
    mute_until: Optional[str] = None
