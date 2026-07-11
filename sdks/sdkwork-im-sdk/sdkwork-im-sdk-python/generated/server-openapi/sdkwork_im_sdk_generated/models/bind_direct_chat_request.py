from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class BindDirectChatRequest:
    left_actor_id: str
    left_actor_kind: str
    right_actor_id: str
    right_actor_kind: str
    conversation_id: Optional[str] = None
    direct_chat_id: Optional[str] = None
