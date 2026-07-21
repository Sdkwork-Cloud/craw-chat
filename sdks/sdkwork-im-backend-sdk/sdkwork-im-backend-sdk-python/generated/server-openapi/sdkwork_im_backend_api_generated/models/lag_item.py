from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class LagItem:
    component: str
    scope_id: str
    current_offset: str
    committed_offset: str
    lag: str
