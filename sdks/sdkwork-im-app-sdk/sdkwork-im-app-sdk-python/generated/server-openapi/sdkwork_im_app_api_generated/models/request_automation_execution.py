from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RequestAutomationExecution:
    execution_id: str
    trigger_type: str
    target_kind: str
    target_ref: str
    input_payload: Optional[str] = None
