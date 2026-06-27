from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class DeclineFriendRequestRequest:
    declined_at: str
    declined_by_user_id: str
    event_id: str
