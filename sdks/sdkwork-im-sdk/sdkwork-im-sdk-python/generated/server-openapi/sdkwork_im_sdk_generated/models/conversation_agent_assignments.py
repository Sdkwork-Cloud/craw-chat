from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .conversation_agent_assignment import ConversationAgentAssignment


@dataclass
class ConversationAgentAssignments:
    generation: int
    source: str
    agents: List[ConversationAgentAssignment]
