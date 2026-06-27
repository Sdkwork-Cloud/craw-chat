from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class AppendAgentResponseDeltaRequest:
    frame_seq: int
    frame_type: str
    encoding: str
    payload: str
    schema_ref: Optional[str] = None
    attributes: Optional[Dict[str, str]] = None
