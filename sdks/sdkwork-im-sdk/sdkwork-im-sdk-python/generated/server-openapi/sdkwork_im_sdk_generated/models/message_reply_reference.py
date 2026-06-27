from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class MessageReplyReference:
    message_id: str
    sender_display_name: str
    content_preview: str
