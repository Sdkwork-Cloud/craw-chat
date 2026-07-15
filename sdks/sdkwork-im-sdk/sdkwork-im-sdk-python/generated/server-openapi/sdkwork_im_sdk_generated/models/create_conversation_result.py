from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class CreateConversationResult:
    conversation_id: str
    event_id: str
    request_key: Optional[str] = None
    delivery_status: Optional[str] = None
    proof_version: Optional[str] = None
    knowledgebase_initialization: Optional[str] = None
