from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class PostRtcSignalRequest:
    signal_type: str
    payload: str
    schema_ref: Optional[str] = None
    signaling_stream_id: Optional[str] = None
