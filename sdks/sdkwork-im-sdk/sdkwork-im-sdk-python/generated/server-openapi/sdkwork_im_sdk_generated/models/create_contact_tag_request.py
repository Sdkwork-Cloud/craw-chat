from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class CreateContactTagRequest:
    name: str
    color: str
    count: Optional[int] = None
    bg: Optional[str] = None
    border: Optional[str] = None
