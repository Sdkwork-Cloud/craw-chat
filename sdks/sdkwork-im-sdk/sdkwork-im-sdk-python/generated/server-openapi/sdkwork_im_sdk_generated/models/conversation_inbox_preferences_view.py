from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ConversationInboxPreferencesView:
    is_pinned: bool
    is_muted: bool
    is_marked_unread: bool
    is_hidden: bool
