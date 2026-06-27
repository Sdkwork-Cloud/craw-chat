from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class CreateAgentDialogRequest:
    agent_id: str
    conversation_id: Optional[str] = None
    title: Optional[str] = None
