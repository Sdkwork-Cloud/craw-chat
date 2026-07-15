from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .direct_chat import DirectChat
    from .friend_request import FriendRequest
    from .friendship import Friendship
    from .social_friend_request_accepted_conversation import SocialFriendRequestAcceptedConversation


@dataclass
class SocialFriendRequestAcceptanceResponse:
    friend_request: FriendRequest
    friendship: Friendship
    direct_chat: DirectChat
    conversation: SocialFriendRequestAcceptedConversation
