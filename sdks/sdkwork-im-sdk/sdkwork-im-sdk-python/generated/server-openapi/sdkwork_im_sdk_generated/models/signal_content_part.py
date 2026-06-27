from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SignalContentPart:
    kind: str
    signal_type: Optional[str]
    payload: Optional[str]
    schema_ref: Optional[str] = None
