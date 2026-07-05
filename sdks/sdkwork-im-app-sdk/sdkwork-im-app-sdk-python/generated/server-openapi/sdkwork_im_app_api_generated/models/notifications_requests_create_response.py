from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .notification_request_response import NotificationRequestResponse


@dataclass
class NotificationsRequestsCreateResponse:
    code: int
    data: Any
    trace_id: str
