from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class PostMessageResult:
    message_id: str
    message_seq: int
    event_id: str
    delivery_status: str
    request_key: Optional[str] = None
    proof_version: Optional[str] = None
