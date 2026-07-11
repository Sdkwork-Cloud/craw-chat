from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class CreateConversationRequest:
    conversation_type: str
    conversation_id: Optional[str] = None
    group_name: Optional[str] = None
    client_request_key: Optional[str] = None
    policy_version: Optional[str] = None
    capability_flags: Optional[List[str]] = None
    history_visibility: Optional[str] = None
    retention_policy_ref: Optional[str] = None
