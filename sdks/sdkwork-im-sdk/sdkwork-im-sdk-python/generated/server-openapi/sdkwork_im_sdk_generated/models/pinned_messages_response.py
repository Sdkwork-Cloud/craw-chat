from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .message_interaction_summary_view import MessageInteractionSummaryView


@dataclass
class PinnedMessagesResponse:
    items: List[MessageInteractionSummaryView]
