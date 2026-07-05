from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class OpenApiBlockUserRequest:
    blocked_user_id: str
    scope: Optional[str] = None
