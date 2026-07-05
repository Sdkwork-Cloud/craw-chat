from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_user_search_result import SocialUserSearchResult


@dataclass
class SocialUsersListResponse:
    code: int
    data: Any
    trace_id: str
