from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ProblemDetail:
    type: str
    title: str
    status: int
    detail: str
    code: Optional[str] = None
    message: Optional[str] = None
    trace_id: Optional[str] = None
    retryable: Optional[bool] = None
