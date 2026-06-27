from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class CompleteAgentResponseRequest:
    frame_seq: int
    result_message_id: Optional[str] = None
