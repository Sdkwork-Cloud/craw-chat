from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class UpdateConversationProfileRequest:
    display_name: Optional[str] = None
    avatar_url: Optional[str] = None
    notice: Optional[str] = None
