from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .open_api_user_block_response import OpenApiUserBlockResponse


@dataclass
class SocialUserBlocksCreateResponse201:
    code: int
    data: Any
    trace_id: str
