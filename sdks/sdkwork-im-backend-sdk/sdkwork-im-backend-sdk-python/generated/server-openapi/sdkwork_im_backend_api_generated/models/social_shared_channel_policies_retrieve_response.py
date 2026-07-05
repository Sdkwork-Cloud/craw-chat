from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_shared_channel_policy_snapshot_response import SocialSharedChannelPolicySnapshotResponse


@dataclass
class SocialSharedChannelPoliciesRetrieveResponse:
    code: int
    data: Any
    trace_id: str
