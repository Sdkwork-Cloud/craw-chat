from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_user_search_result import SocialUserSearchResult


@dataclass
class SocialUserSearchResponse:
    items: List[SocialUserSearchResult]
    has_more: bool
    next_cursor: Optional[str] = None
