from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class CreateSystemChannelRequest:
    conversation_id: str
    subscriber_id: str
