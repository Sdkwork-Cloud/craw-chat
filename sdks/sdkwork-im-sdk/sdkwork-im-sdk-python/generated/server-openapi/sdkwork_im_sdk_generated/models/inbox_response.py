from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .conversation_inbox_entry import ConversationInboxEntry


@dataclass
class InboxResponse:
    items: List[ConversationInboxEntry]
    has_more: bool
    next_cursor: Optional[str] = None
