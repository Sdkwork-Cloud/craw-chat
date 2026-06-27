from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SubmitFriendRequestRequest:
    event_id: str
    request_id: str
    requested_at: str
    requester_user_id: str
    target_user_id: str
    request_message: Optional[str] = None
