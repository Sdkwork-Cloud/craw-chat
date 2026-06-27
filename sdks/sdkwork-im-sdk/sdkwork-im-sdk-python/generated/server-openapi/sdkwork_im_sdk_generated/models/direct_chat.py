from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class DirectChat:
    tenant_id: str
    direct_chat_id: str
    conversation_id: str
    status: str
