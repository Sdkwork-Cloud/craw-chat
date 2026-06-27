from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ConversationMember:
    tenant_id: str
    conversation_id: str
    member_id: str
    principal_id: str
    principal_kind: str
    role: str
    state: str
    joined_at: str
