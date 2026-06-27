from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class BindDirectChatRequest:
    conversation_id: Optional[str] = None
    direct_chat_id: Optional[str] = None
    left_actor_id: Optional[str] = None
    left_actor_kind: Optional[str] = None
    right_actor_id: Optional[str] = None
    right_actor_kind: Optional[str] = None
    target_user_id: Optional[str] = None
