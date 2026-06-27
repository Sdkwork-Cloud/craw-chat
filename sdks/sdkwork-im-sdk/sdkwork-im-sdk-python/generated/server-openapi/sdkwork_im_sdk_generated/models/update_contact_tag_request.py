from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class UpdateContactTagRequest:
    name: Optional[str] = None
    color: Optional[str] = None
    count: Optional[int] = None
    bg: Optional[str] = None
    border: Optional[str] = None
