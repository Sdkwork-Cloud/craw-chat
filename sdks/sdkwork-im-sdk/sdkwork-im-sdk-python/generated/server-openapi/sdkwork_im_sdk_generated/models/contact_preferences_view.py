from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ContactPreferencesView:
    tenant_id: str
    owner_user_id: str
    target_user_id: str
    is_starred: bool
    remark: str
    is_blocked: bool
    updated_at: str
