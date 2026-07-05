from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .realtime_subscription_sync_response import RealtimeSubscriptionSyncResponse


@dataclass
class RealtimeSubscriptionsSyncResponse:
    code: int
    data: Any
    trace_id: str
