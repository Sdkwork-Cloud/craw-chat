from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .rtc_signal_event import RtcSignalEvent


@dataclass
class CallsSessionsSignalsCreateResponse201:
    code: int
    data: Any
    trace_id: str
