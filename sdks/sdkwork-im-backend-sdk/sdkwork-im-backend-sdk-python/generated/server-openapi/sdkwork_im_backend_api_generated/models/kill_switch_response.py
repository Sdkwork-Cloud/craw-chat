from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class KillSwitchResponse:
    active: bool
    disabled_bindings: List[str]
    disabled_capabilities: List[str]
    disabled_codecs: List[str]
    reason: str
    rule_id: str
