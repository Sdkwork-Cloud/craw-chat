from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ConversationInboxPeerView:
    principal_kind: str
    principal_id: str
    user_id: Optional[str] = None
    chat_id: Optional[str] = None
    display_name: Optional[str] = None
    avatar_url: Optional[str] = None
    relationship_state: Optional[str] = None
