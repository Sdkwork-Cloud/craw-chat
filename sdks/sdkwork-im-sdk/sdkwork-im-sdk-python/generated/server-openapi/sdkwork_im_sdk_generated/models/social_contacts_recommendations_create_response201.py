from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .contact_recommendation_view import ContactRecommendationView


@dataclass
class SocialContactsRecommendationsCreateResponse201:
    code: int
    data: Any
    trace_id: str
