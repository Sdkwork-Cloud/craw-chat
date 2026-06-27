from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class BindDirectChatRequest:
    bound_at: str
    conversation_id: str
    direct_chat_id: str
    event_id: str
    left_actor_id: str
    right_actor_id: str
