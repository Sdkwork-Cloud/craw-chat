from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class CreateAgentHandoffRequest:
    conversation_id: str
    target_id: str
    target_kind: str
    handoff_session_id: str
    handoff_reason: Optional[str] = None
