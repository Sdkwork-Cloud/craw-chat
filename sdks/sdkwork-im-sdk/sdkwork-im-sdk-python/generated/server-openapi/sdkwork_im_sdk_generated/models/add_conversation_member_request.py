from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class AddConversationMemberRequest:
    principal_id: str
    principal_kind: str
    role: str
    attributes: Optional[Dict[str, Any]] = None
