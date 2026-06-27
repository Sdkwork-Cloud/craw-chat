from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class MessageVisibilityMutationResult:
    tenant_id: str
    conversation_id: str
    message_id: str
    message_seq: int
    principal_kind: str
    principal_id: str
    is_deleted: bool
    updated_at: str
