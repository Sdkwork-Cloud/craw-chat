from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .friend_request import FriendRequest


@dataclass
class SocialFriendRequestMutationResponse:
    friend_request: FriendRequest
