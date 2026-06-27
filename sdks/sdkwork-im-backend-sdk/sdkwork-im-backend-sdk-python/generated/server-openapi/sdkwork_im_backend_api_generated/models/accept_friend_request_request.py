from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class AcceptFriendRequestRequest:
    accepted_at: str
    accepted_by_user_id: str
    event_id: str
