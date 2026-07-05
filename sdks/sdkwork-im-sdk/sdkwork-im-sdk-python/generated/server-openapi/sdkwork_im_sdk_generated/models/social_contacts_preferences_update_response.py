from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .contact_preferences_view import ContactPreferencesView


@dataclass
class SocialContactsPreferencesUpdateResponse:
    code: int
    data: Any
    trace_id: str
