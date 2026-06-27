from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class MessageFavoriteView:
    tenant_id: str
    principal_kind: str
    principal_id: str
    favorite_id: str
    favorite_type: str
    conversation_id: str
    message_id: str
    message_seq: int
    title: str
    content_preview: str
    source_display_name: str
    favorited_at: str
