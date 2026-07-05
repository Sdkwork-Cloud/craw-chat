from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .stream_frame_view import StreamFrameView


@dataclass
class StreamsFramesListResponse:
    code: int
    data: Any
    trace_id: str
