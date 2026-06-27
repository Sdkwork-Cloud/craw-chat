from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class UpdateRtcSessionRequest:
    artifact_message_id: Optional[str] = None
