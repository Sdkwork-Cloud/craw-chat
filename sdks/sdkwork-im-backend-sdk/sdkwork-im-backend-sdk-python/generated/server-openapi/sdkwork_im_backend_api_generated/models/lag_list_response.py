from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .lag_page_data import LagPageData


@dataclass
class LagListResponse:
    code: int
    data: Any
    trace_id: str
