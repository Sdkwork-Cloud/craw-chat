from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .content_part import ContentPart
    from .message_reply_reference import MessageReplyReference


@dataclass
class PostMessageRequest:
    text: Optional[str] = None
    parts: Optional[List[ContentPart]] = None
    reply_to: Optional[MessageReplyReference] = None
    client_msg_id: Optional[str] = None
    summary: Optional[str] = None
    render_hints: Optional[Dict[str, Any]] = None
