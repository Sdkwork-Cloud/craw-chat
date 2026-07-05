from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class MessageMentions:
    """Parsed @mention metadata extracted from message text parts. Allows notification fanout to determine who was mentioned and client rendering to highlight mentions without re-parsing."""
    user_ids: Optional[List[str]] = None
    scopes: Optional[List[str]] = None
