from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .conversation_member import ConversationMember
    from .page_info import PageInfo


@dataclass
class ConversationsMemberDirectoryListResponse:
    code: int
    data: Any
    trace_id: str
