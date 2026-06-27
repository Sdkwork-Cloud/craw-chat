from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class StreamSession:
    tenant_id: str
    stream_id: str
    stream_type: str
    scope_kind: str
    scope_id: str
    durability_class: str
    ordering_scope: str
    state: str
    last_frame_seq: int
    opened_at: str
    schema_ref: Optional[str] = None
    last_checkpoint_seq: Optional[int] = None
    result_message_id: Optional[str] = None
    closed_at: Optional[str] = None
    expires_at: Optional[str] = None
