from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_friendship_snapshot_response import SocialFriendshipSnapshotResponse


@dataclass
class SocialFriendshipsRetrieveResponse:
    code: int
    data: Any
    trace_id: str
