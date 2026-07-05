from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .open_api_block_user_response import OpenApiBlockUserResponse


@dataclass
class SocialUserBlocksCreateResponse:
    code: int
    data: Any
    trace_id: str
