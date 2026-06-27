from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class QuotaProfileResponse:
    max_concurrent_sessions_per_tenant: int
    max_inflight_messages: int
    max_payload_bytes: int
    max_subscriptions_per_session: int
    profile_id: str
