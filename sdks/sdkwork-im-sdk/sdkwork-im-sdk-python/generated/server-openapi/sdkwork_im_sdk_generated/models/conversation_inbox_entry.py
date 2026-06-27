from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .conversation_inbox_peer_view import ConversationInboxPeerView
    from .conversation_inbox_preferences_view import ConversationInboxPreferencesView


@dataclass
class ConversationInboxEntry:
    tenant_id: str
    conversation_id: str
    conversation_type: str
    last_activity_at: str
    message_count: int
    last_message_seq: int
    unread_count: int
    agent_handoff: Optional[bool] = None
    display_name: Optional[str] = None
    avatar_url: Optional[str] = None
    display_source: Optional[str] = None
    peer: Optional[ConversationInboxPeerView] = None
    preferences: Optional[ConversationInboxPreferencesView] = None
    last_message_id: Optional[str] = None
    last_sender_id: Optional[str] = None
    last_summary: Optional[str] = None
    last_message_at: Optional[str] = None
