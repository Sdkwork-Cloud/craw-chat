from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .message_body import MessageBody
    from .sender import Sender


@dataclass
class TimelineViewEntry:
    tenant_id: str
    conversation_id: str
    message_id: str
    message_seq: int
    sender: Sender
    body: MessageBody
    message_type: str
    delivery_mode: str
    occurred_at: str
    summary: Optional[str] = None
    client_msg_id: Optional[str] = None
    stream_session_id: Optional[str] = None
    rtc_session_id: Optional[str] = None
    committed_at: Optional[str] = None
