from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .read_cursor_view import ReadCursorView


@dataclass
class ConversationsReadCursorUpdateResponse:
    code: int
    data: Any
    trace_id: str
