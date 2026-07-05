from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_external_member_link_snapshot_response import SocialExternalMemberLinkSnapshotResponse


@dataclass
class SocialExternalMemberLinksRetrieveResponse:
    code: int
    data: Any
    trace_id: str
