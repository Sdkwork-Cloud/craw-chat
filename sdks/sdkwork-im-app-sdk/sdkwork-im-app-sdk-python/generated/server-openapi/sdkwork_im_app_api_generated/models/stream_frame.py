from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .sender import Sender


@dataclass
class StreamFrame:
    tenant_id: str
    stream_id: str
    stream_type: str
    scope_kind: str
    scope_id: str
    frame_seq: int
    frame_type: str
    encoding: str
    payload: str
    sender: Sender
    attributes: Dict[str, str]
    occurred_at: str
    schema_ref: Optional[str] = None
