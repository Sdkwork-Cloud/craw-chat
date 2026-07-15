from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .archive_group_conversation_response import ArchiveGroupConversationResponse


@dataclass
class ConversationsArchiveResponse:
    code: int
    data: Any
    trace_id: str
