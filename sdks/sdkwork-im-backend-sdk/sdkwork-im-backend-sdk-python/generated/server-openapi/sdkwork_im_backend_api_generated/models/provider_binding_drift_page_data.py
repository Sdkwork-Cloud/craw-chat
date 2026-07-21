from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .page_info import PageInfo
    from .provider_binding_drift_item import ProviderBindingDriftItem


@dataclass
class ProviderBindingDriftPageData:
    items: List[ProviderBindingDriftItem]
    page_info: PageInfo
