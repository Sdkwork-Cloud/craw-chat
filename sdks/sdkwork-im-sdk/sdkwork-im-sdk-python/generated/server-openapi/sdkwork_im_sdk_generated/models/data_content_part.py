from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class DataContentPart:
    kind: str
    schema_ref: Optional[str]
    encoding: Optional[str]
    payload: Optional[str]
