from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_friend_request_commit_response import SocialFriendRequestCommitResponse


@dataclass
class SocialFriendRequestsAcceptResponse:
    code: int
    data: Any
    trace_id: str
