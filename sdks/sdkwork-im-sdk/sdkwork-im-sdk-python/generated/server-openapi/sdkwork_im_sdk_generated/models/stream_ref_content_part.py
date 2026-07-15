from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class StreamRefContentPart:
    kind: str
    stream_id: str
    stream_type: str
    state: str
