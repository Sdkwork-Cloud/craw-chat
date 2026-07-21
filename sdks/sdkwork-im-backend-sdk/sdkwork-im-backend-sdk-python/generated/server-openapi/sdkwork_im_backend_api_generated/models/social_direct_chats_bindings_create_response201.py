from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_direct_chat_commit_response import SocialDirectChatCommitResponse


@dataclass
class SocialDirectChatsBindingsCreateResponse201:
    code: int
    data: Any
    trace_id: str
