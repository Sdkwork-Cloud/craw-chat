from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .automation_execution import AutomationExecution


@dataclass
class AutomationExecutionsRetrieveResponse:
    code: int
    data: Any
    trace_id: str
