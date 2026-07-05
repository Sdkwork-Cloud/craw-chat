from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .rtc_session import RtcSession


@dataclass
class CallsSessionsRetrieveResponse:
    code: int
    data: Any
    trace_id: str
