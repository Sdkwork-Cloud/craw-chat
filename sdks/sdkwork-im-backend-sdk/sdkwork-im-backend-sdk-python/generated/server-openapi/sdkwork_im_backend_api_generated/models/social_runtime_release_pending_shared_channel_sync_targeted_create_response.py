from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_shared_channel_sync_pending_release_response import SocialSharedChannelSyncPendingReleaseResponse


@dataclass
class SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse:
    code: int
    data: Any
    trace_id: str
