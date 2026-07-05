from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .user_block import UserBlock


@dataclass
class OpenApiBlockUserResponse:
    user_block: UserBlock
