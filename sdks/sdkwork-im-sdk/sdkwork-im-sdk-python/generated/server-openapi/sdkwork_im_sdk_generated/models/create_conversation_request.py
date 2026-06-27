from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class CreateConversationRequest:
    conversation_id: Optional[str] = None
    conversation_type: Optional[str] = None
    kind: Optional[str] = None
    title: Optional[str] = None
    member_ids: Optional[List[str]] = None
