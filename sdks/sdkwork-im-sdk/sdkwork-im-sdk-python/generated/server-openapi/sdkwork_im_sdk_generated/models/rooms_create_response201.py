from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .create_conversation_result import CreateConversationResult


@dataclass
class RoomsCreateResponse201:
    code: int
    data: Any
    trace_id: str
