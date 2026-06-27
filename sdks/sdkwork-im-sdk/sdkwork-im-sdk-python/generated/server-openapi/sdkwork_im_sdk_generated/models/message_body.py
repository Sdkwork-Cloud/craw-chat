from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .content_part import ContentPart
    from .message_reply_reference import MessageReplyReference


@dataclass
class MessageBody:
    parts: List[ContentPart]
    text: Optional[str] = None
    reply_to: Optional[MessageReplyReference] = None
    render_hints: Optional[Dict[str, Any]] = None
    summary: Optional[str] = None
    metadata: Optional[Dict[str, Any]] = None
