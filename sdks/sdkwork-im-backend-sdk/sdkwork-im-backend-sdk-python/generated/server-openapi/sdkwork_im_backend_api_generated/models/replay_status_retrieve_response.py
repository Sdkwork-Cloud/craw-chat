from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ReplayStatusRetrieveResponse:
    code: int
    data: Any
    trace_id: str
