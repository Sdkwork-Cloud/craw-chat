from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .agent_subject import AgentSubject


@dataclass
class StartAgentResponseRequest:
    execution_id: str
    stream_id: str
    stream_type: str
    conversation_id: str
    agent: AgentSubject
    schema_ref: Optional[str] = None
    member_id: Optional[str] = None
