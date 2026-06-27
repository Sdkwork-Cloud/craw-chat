from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ContactView:
    tenant_id: str
    owner_user_id: str
    target_user_id: str
    contact_type: str
    relationship_state: str
    friendship_id: str
    established_at: str
    last_interaction_at: str
    direct_chat_id: Optional[str] = None
    conversation_id: Optional[str] = None
