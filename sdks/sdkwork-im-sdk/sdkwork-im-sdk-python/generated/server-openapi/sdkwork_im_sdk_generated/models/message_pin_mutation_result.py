from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class MessagePinMutationResult:
    tenant_id: str
    conversation_id: str
    message_id: str
    is_pinned: bool
    updated_at: str
