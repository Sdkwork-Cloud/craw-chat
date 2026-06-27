from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class AgentToolCall:
    tenant_id: str
    execution_id: str
    agent_id: str
    tool_call_id: str
    tool_name: str
    arguments_payload: str
    state: str
    requested_at: str
    result_payload: Optional[str] = None
    completed_at: Optional[str] = None
