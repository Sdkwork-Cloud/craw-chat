from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SpaceGroupMemberCreateRequest:
    user_id: str
    role: Optional[str] = None
    nickname: Optional[str] = None
