from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_runtime_repair_response import SocialRuntimeRepairResponse


@dataclass
class SocialRuntimeRepairDerivedSnapshotCreateResponse201:
    code: int
    data: Any
    trace_id: str
