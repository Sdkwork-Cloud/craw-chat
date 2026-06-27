from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class StreamFrameView:
    stream_id: str
    frame_seq: int
    payload: str
    created_at: str
