from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .message_pin_view import MessagePinView
    from .message_reaction_count_view import MessageReactionCountView


@dataclass
class MessageInteractionSummaryView:
    tenant_id: str
    conversation_id: str
    message_id: str
    message_seq: int
    total_reaction_count: int
    reaction_counts: List[MessageReactionCountView]
    pin: Optional[MessagePinView] = None
