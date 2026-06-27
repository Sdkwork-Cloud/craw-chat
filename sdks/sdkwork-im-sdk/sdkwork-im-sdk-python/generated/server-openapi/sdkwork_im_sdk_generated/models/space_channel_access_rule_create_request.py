from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SpaceChannelAccessRuleCreateRequest:
    rule_type: str
    permission: str
    principal_kind: Optional[str] = None
    principal_id: Optional[str] = None
