from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class BlockUserRequest:
    blocked_user_id: str
    scope: str
    direct_chat_id: Optional[str] = None
    expires_at: Optional[str] = None
