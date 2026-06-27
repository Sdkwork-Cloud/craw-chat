from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SocialUserSearchResult:
    tenant_id: str
    user_id: str
    chat_id: str
    display_name: str
    relationship_state: str
    avatar_url: Optional[str] = None
    email: Optional[str] = None
    phone: Optional[str] = None
    metadata: Optional[Dict[str, Any]] = None
