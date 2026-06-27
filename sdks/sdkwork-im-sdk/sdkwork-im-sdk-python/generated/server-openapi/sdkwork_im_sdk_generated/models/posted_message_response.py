from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .message_body import MessageBody


@dataclass
class PostedMessageResponse:
    conversation_id: str
    message_id: str
    message_seq: int
    body: MessageBody
    occurred_at: str
