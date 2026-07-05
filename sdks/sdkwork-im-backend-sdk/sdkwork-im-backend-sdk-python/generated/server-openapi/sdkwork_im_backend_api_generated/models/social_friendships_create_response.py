from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_friendship_commit_response import SocialFriendshipCommitResponse


@dataclass
class SocialFriendshipsCreateResponse:
    code: int
    data: Any
    trace_id: str
