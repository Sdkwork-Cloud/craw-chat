from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class AgentSubject:
    agent_id: str
    metadata: Dict[str, str]
    session_id: Optional[str] = None
