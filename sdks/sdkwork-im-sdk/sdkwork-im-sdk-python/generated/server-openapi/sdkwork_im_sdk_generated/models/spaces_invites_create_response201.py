from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .space_invite_view import SpaceInviteView


@dataclass
class SpacesInvitesCreateResponse201:
    code: int
    data: Any
    trace_id: str
