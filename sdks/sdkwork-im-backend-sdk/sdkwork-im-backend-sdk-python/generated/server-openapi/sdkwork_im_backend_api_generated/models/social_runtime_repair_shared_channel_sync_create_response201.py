from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_shared_channel_sync_repair_response import SocialSharedChannelSyncRepairResponse


@dataclass
class SocialRuntimeRepairSharedChannelSyncCreateResponse201:
    code: int
    data: Any
    trace_id: str
