from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ContactTagView:
    tenant_id: str
    owner_user_id: str
    tag_id: str
    name: str
    color: str
    count: int
    bg: str
    border: str
    created_at: str
    updated_at: str
