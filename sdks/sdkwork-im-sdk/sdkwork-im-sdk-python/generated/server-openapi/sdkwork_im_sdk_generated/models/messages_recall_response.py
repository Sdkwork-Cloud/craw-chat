from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .posted_message_response import PostedMessageResponse


@dataclass
class MessagesRecallResponse:
    code: int
    data: Any
    trace_id: str
