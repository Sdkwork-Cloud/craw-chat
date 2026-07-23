from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class PortalConversationOperationalMetrics:
    lagging_scope_count: str
    max_operational_lag: str
    pending_outbox_event_count: str
    failed_outbox_attempt_count: str
