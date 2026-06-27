from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class AutomationExecution:
    tenant_id: str
    principal_id: str
    principal_kind: str
    execution_id: str
    trigger_type: str
    target_kind: str
    target_ref: str
    state: str
    retry_count: int
    requested_at: str
    input_payload: Optional[str] = None
    output_payload: Optional[str] = None
    completed_at: Optional[str] = None
    failure_reason: Optional[str] = None
