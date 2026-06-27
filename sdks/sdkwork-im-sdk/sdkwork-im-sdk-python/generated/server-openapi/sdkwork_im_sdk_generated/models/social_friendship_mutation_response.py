from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .friendship import Friendship


@dataclass
class SocialFriendshipMutationResponse:
    friendship: Friendship
