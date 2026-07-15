from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ConversationAgentAssignment:
    agent_id: str
    revision_id: Optional[str] = None
