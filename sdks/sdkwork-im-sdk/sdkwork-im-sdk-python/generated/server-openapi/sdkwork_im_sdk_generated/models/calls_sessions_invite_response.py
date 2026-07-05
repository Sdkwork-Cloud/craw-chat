from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .rtc_session_mutation_response import RtcSessionMutationResponse


@dataclass
class CallsSessionsInviteResponse:
    code: int
    data: Any
    trace_id: str
