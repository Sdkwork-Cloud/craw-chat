from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .provider_binding_drift_page_data import ProviderBindingDriftPageData


@dataclass
class ProviderBindingDriftListResponse:
    code: int
    data: Any
    trace_id: str
