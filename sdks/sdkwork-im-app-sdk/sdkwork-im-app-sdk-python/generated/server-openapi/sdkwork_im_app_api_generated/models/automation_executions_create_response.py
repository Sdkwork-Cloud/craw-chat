from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .automation_execution_request_response import AutomationExecutionRequestResponse


@dataclass
class AutomationExecutionsCreateResponse:
    code: int
    data: Any
    trace_id: str
