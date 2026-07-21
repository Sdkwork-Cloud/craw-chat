from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_shared_channel_sync_pending_stale_reclaim_response import SocialSharedChannelSyncPendingStaleReclaimResponse


@dataclass
class SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201:
    code: int
    data: Any
    trace_id: str
