from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SocialSharedChannelSyncPendingTargetedTakeoverRequest:
    request_keys: List[str]
    allow_legacy_untracked: Optional[bool] = None
