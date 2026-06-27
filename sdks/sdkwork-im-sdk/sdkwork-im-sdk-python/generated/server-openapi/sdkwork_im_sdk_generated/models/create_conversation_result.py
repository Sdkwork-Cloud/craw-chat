from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class CreateConversationResult:
    tenant_id: str
    conversation_id: str
    kind: str
    created_at: str
