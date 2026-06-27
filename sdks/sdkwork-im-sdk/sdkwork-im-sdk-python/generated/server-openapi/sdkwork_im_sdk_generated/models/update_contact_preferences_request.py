from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class UpdateContactPreferencesRequest:
    is_starred: Optional[bool] = None
    remark: Optional[str] = None
    is_blocked: Optional[bool] = None
