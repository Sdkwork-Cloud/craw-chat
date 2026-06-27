from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ReadCursorView:
    tenant_id: str
    conversation_id: str
    principal_id: str
    read_seq: int
    updated_at: str
