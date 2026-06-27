from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .message_favorite_view import MessageFavoriteView


@dataclass
class FavoriteMessagesResponse:
    items: List[MessageFavoriteView]
    has_more: bool
    next_cursor: Optional[str] = None
