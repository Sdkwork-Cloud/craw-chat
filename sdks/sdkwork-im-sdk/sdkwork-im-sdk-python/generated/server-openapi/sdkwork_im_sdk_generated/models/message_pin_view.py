from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .interaction_actor_view import InteractionActorView


@dataclass
class MessagePinView:
    pinned_by: InteractionActorView
    pinned_at: str
