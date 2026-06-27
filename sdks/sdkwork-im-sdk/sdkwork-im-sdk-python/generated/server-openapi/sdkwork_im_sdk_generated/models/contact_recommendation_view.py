from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ContactRecommendationView:
    tenant_id: str
    owner_user_id: str
    target_user_id: str
    recommendation_id: str
    created_at: str
    target_conversation_id: Optional[str] = None
