from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .conversation_agent_assignment import ConversationAgentAssignment


@dataclass
class CreateConversationRequest:
    conversation_type: str
    conversation_id: Optional[str] = None
    group_name: Optional[str] = None
    client_request_key: Optional[str] = None
    initialize_knowledgebase: Optional[bool] = None
    member_user_ids: Optional[List[str]] = None
    agent_assignments: Optional[List[ConversationAgentAssignment]] = None
    policy_version: Optional[str] = None
    capability_flags: Optional[List[str]] = None
    history_visibility: Optional[str] = None
    retention_policy_ref: Optional[str] = None
