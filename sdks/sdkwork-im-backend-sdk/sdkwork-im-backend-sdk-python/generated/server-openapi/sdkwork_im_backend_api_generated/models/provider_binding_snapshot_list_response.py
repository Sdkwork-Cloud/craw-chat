from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .provider_binding_snapshot_page_data import ProviderBindingSnapshotPageData


@dataclass
class ProviderBindingSnapshotListResponse:
    code: int
    data: Any
    trace_id: str
