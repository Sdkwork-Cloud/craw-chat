from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class BlockUserRequest:
    block_id: str
    blocked_user_id: str
    blocker_user_id: str
    effective_at: str
    event_id: str
    scope: str
    direct_chat_id: Optional[str] = None
    expires_at: Optional[str] = None
