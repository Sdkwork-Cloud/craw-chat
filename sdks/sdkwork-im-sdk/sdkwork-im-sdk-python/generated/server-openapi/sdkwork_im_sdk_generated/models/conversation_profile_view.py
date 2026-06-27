from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ConversationProfileView:
    tenant_id: str
    conversation_id: str
    display_name: str
    avatar_url: str
    notice: str
    updated_at: str
    updated_by_principal_kind: Optional[str] = None
    updated_by_principal_id: Optional[str] = None
