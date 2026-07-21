from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .lag_item import LagItem
    from .page_info import PageInfo


@dataclass
class LagPageData:
    items: List[LagItem]
    page_info: PageInfo
