from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .conversation_agent_assignments import ConversationAgentAssignments


@dataclass
class ConversationsAgentsRetrieveResponse:
    code: int
    data: Any
    trace_id: str
