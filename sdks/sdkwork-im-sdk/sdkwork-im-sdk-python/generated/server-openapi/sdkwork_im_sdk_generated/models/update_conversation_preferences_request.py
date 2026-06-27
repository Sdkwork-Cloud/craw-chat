from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class UpdateConversationPreferencesRequest:
    is_pinned: Optional[bool] = None
    is_muted: Optional[bool] = None
    is_marked_unread: Optional[bool] = None
    is_hidden: Optional[bool] = None
