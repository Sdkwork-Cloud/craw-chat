from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RequestAgentToolCallRequest:
    execution_id: str
    tool_call_id: str
    tool_name: str
    arguments_payload: str
