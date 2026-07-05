from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class UserBlock:
    tenant_id: str
    block_id: str
    blocker_user_id: str
    blocked_user_id: str
    scope: str
    status: str
    created_at: str
    updated_at: str
    direct_chat_id: Optional[str] = None
    expires_at: Optional[str] = None
