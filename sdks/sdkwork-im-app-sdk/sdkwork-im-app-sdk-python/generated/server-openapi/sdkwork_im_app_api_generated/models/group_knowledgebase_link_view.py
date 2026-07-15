from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class GroupKnowledgebaseLinkView:
    conversation_id: str
    lifecycle_state: str
    membership_epoch: str
    upstream_link_generation: str
    space_id: Optional[str] = None
    space_uuid: Optional[str] = None
    provisioning_operation_id: Optional[str] = None
    last_error_code: Optional[str] = None
