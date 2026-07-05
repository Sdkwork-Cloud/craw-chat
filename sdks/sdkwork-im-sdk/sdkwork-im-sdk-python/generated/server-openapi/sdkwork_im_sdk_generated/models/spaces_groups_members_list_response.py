from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .page_info import PageInfo
    from .space_group_member_view import SpaceGroupMemberView


@dataclass
class SpacesGroupsMembersListResponse:
    code: int
    data: Any
    trace_id: str
