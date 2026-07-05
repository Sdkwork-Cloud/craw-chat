from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .contact_tag_view import ContactTagView


@dataclass
class SocialContactsTagsListResponse:
    code: int
    data: Any
    trace_id: str
