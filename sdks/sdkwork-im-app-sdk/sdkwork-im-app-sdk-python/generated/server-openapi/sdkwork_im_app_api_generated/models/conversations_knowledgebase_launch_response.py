from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .group_knowledgebase_launch_response import GroupKnowledgebaseLaunchResponse


@dataclass
class ConversationsKnowledgebaseLaunchResponse:
    code: int
    data: Any
    trace_id: str
