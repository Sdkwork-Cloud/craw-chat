from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class NotificationRequestResponse:
    tenant_id: str
    notification_id: str
    source_event_id: str
    source_event_type: str
    category: str
    channel: str
    recipient_id: str
    recipient_kind: str
    status: str
    requested_at: str
    request_key: str
    delivery_status: str
    proof_version: str
    title: Optional[str] = None
    body: Optional[str] = None
    payload: Optional[str] = None
    dispatched_at: Optional[str] = None
    failure_reason: Optional[str] = None
