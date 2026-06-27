from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ChangeConversationMemberRoleRequest:
    member_id: str
    role: str
