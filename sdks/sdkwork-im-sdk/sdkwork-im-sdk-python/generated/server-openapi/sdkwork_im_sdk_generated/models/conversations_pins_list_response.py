from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .message_interaction_summary_view import MessageInteractionSummaryView
    from .page_info import PageInfo


@dataclass
class ConversationsPinsListResponse:
    code: int
    data: Any
    trace_id: str
