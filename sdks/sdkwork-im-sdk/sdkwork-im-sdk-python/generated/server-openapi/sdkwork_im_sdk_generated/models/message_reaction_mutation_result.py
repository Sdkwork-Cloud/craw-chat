from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class MessageReactionMutationResult:
    tenant_id: str
    conversation_id: str
    message_id: str
    reaction_key: str
    count: int
    updated_at: str
