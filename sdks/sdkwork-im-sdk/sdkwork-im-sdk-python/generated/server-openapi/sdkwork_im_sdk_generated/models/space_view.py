from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SpaceView:
    space_id: str
    space_name: str
    space_type: str
    owner_user_id: str
    created_at: str
