from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .create_conversation_result import CreateConversationResult
    from .direct_chat import DirectChat
    from .friend_request import FriendRequest
    from .friendship import Friendship


@dataclass
class SocialFriendRequestAcceptanceResponse:
    friend_request: FriendRequest
    friendship: Friendship
    direct_chat: DirectChat
    conversation: CreateConversationResult
