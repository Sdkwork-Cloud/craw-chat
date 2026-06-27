from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .conversation_member import ConversationMember


@dataclass
class ListMembersResponse:
    items: List[ConversationMember]
    has_more: bool
    next_cursor: Optional[str] = None
