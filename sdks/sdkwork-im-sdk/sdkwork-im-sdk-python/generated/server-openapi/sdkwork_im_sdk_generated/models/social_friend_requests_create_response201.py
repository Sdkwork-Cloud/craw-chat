from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_friend_request_mutation_response import SocialFriendRequestMutationResponse


@dataclass
class SocialFriendRequestsCreateResponse201:
    code: int
    data: Any
    trace_id: str
