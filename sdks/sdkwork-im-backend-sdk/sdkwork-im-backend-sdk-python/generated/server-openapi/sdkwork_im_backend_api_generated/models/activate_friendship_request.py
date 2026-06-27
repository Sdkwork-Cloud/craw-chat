from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ActivateFriendshipRequest:
    established_at: str
    event_id: str
    friendship_id: str
    initiator_user_id: str
    peer_user_id: str
    direct_chat_id: Optional[str] = None
