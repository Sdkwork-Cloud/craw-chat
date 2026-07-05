from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_external_member_link_commit_response import SocialExternalMemberLinkCommitResponse


@dataclass
class SocialExternalMemberLinksCreateResponse:
    code: int
    data: Any
    trace_id: str
