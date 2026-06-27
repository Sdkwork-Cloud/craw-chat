from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ConversationSummaryView:
    tenant_id: str
    conversation_id: str
    message_count: int
    last_message_seq: int
    last_summary: Optional[str] = None
    last_message_at: Optional[str] = None
