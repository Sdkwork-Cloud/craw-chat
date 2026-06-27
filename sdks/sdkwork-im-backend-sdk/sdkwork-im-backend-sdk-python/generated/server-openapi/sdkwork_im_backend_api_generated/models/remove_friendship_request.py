from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RemoveFriendshipRequest:
    event_id: str
    removed_at: str
    removed_by_user_id: str
