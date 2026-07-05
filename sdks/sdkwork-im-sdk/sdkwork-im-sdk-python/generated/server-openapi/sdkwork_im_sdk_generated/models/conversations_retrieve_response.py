from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .conversation_summary_view import ConversationSummaryView


@dataclass
class ConversationsRetrieveResponse:
    code: int
    data: Any
    trace_id: str
