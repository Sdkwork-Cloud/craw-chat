from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .ack_response import AckResponse


@dataclass
class ConversationsAgentHandoffResolveResponse:
    code: int
    data: Any
    trace_id: str
