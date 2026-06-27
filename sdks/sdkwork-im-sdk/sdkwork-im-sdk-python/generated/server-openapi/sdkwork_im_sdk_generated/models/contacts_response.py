from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .contact_view import ContactView


@dataclass
class ContactsResponse:
    items: List[ContactView]
    has_more: bool
    next_cursor: Optional[str] = None
