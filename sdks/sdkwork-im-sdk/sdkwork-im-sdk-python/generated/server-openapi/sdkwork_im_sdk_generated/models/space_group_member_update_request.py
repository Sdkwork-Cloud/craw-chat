from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SpaceGroupMemberUpdateRequest:
    role: Optional[str] = None
    nickname: Optional[str] = None
    mute_until: Optional[str] = None
