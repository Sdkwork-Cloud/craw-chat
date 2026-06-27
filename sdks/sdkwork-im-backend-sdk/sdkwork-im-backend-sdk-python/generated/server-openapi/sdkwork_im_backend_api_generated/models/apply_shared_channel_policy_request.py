from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ApplySharedChannelPolicyRequest:
    applied_at: str
    channel_id: str
    connection_id: str
    event_id: str
    history_visibility: str
    policy_id: str
    policy_version: int
    conversation_id: Optional[str] = None
