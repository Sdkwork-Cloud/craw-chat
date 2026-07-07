from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class EventActor:
    actor_id: str
    actor_kind: str
    actor_session_id: Optional[str] = None
