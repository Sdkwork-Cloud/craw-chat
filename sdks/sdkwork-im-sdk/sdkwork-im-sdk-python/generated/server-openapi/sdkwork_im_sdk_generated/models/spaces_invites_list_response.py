from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .page_info import PageInfo
    from .space_invite_view import SpaceInviteView


@dataclass
class SpacesInvitesListResponse:
    code: int
    data: Any
    trace_id: str
