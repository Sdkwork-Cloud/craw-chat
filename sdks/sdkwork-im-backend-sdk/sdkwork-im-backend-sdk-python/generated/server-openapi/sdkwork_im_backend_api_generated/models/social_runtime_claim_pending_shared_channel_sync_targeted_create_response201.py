from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_shared_channel_sync_pending_claim_response import SocialSharedChannelSyncPendingClaimResponse


@dataclass
class SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201:
    code: int
    data: Any
    trace_id: str
