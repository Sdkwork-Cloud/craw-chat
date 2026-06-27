from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class FavoriteMessageRequest:
    conversation_id: str
    favorite_type: str
    title: str
    content_preview: str
    source_display_name: str
