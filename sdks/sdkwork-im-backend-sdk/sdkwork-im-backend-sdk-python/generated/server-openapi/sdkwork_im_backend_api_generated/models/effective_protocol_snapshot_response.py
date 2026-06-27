from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class EffectiveProtocolSnapshotResponse:
    allowed_bindings: List[str]
    allowed_codecs: List[str]
    enabled_capabilities: List[str]
    kill_switch_active: bool
    precedence: List[str]
    protocol_version: str
    quota_profile_id: str
    release_channel: str
