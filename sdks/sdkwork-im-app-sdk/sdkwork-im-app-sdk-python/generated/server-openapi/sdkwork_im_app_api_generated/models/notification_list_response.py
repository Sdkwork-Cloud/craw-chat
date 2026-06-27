from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .notification_task import NotificationTask


@dataclass
class NotificationListResponse:
    items: List[NotificationTask]
