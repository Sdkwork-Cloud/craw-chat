from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RuntimeDirRetrieveResponse:
    code: int
    data: Any
    trace_id: str
