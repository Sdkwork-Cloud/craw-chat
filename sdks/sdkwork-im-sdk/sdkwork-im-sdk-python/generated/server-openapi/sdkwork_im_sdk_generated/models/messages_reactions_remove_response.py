from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .message_reaction_mutation_result import MessageReactionMutationResult


@dataclass
class MessagesReactionsRemoveResponse:
    code: int
    data: Any
    trace_id: str
