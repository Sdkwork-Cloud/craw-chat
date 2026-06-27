from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class BindExternalMemberLinkRequest:
    connection_id: str
    event_id: str
    external_member_id: str
    link_id: str
    linked_at: str
    local_actor_id: str
    local_actor_kind: str
    external_display_name: Optional[str] = None
