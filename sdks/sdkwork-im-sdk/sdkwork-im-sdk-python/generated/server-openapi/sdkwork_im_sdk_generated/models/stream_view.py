from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class StreamView:
    tenant_id: str
    stream_id: str
    state: str
    opened_at: str
