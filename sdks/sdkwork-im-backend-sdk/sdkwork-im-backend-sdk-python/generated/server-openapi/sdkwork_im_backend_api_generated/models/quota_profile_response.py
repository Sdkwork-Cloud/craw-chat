from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class QuotaProfileResponse:
    max_concurrent_sessions_per_tenant: str
    max_inflight_messages: str
    max_payload_bytes: str
    max_subscriptions_per_session: str
    profile_id: str
