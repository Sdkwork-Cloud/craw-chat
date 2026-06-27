from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .realtime_subscription_item_input import RealtimeSubscriptionItemInput


@dataclass
class RealtimeSubscriptionSyncRequest:
    device_id: Optional[str] = None
    conversations: Optional[List[str]] = None
    items: Optional[List[RealtimeSubscriptionItemInput]] = None
