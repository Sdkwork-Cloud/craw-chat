from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class MentionContentPart:
    kind: str
    target_kind: str
    target_id: str
    display_text: str
    assignment_generation: int
