from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SubmitFriendRequestRequest:
    target_user_id: str
    request_message: Optional[str] = None
