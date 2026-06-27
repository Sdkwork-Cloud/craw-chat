from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class Friendship:
    tenant_id: str
    friendship_id: str
    initiator_user_id: str
    left_user_id: str
    right_user_id: str
    user_high_id: str
    user_low_id: str
    status: str
    created_at: str
