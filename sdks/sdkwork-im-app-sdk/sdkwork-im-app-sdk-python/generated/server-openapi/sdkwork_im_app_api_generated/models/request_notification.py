from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RequestNotification:
    notification_id: str
    source_event_id: str
    source_event_type: str
    category: str
    channel: str
    recipient_id: str
    recipient_kind: str
    title: Optional[str] = None
    body: Optional[str] = None
    payload: Optional[str] = None
