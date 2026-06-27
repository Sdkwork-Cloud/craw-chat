from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class FriendRequest:
    tenant_id: str
    request_id: str
    requester_user_id: str
    target_user_id: str
    status: str
    created_at: str
    updated_at: str
    request_message: Optional[str] = None
