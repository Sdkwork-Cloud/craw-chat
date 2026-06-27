from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SpaceChannelAccessRuleView:
    rule_id: str
    channel_id: str
    rule_type: str
    permission: str
    created_at: str
    principal_kind: Optional[str] = None
    principal_id: Optional[str] = None
