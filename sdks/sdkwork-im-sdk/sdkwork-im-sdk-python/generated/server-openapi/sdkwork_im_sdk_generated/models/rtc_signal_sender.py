from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RtcSignalSender:
    id: str
    kind: str
    metadata: Dict[str, Any]
    member_id: Optional[str] = None
    device_id: Optional[str] = None
    session_id: Optional[str] = None
