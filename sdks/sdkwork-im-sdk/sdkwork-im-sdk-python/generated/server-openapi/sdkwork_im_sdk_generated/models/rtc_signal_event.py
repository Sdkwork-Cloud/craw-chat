from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .rtc_signal_sender import RtcSignalSender


@dataclass
class RtcSignalEvent:
    tenant_id: str
    rtc_session_id: str
    signal_seq: int
    rtc_mode: str
    signal_type: str
    payload: str
    sender: RtcSignalSender
    occurred_at: str
    conversation_id: Optional[str] = None
    schema_ref: Optional[str] = None
    signaling_stream_id: Optional[str] = None
