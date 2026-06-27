from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class CancelFriendRequestRequest:
    canceled_at: str
    canceled_by_user_id: str
    event_id: str
