from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .conversation_profile_view import ConversationProfileView


@dataclass
class ConversationsProfileRetrieveResponse:
    code: int
    data: Any
    trace_id: str
