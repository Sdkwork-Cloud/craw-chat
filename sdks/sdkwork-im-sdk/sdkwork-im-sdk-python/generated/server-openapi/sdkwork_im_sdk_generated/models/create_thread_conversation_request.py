from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class CreateThreadConversationRequest:
    conversation_id: str
    parent_conversation_id: str
    root_message_id: str
