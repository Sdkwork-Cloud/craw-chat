from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RtcParticipantCredential:
    tenant_id: str
    rtc_session_id: str
    participant_id: str
    credential: str
    expires_at: str
