from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .content_part import ContentPart
    from .message_reply_reference import MessageReplyReference


@dataclass
class EditMessageRequest:
    text: Optional[str] = None
    parts: Optional[List[ContentPart]] = None
    reply_to: Optional[MessageReplyReference] = None
