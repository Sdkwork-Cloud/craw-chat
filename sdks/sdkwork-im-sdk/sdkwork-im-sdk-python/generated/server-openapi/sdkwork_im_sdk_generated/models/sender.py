from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class Sender:
    id: str
    kind: str
    principal_id: Optional[str] = None
    principal_kind: Optional[str] = None
    display_name: Optional[str] = None
    avatar_url: Optional[str] = None
