from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ProviderBindingDriftItem:
    tenant_id: str
    domain: str
    baseline_selected_plugin_id: Optional[str]
    selected_plugin_id: Optional[str]
    baseline_selection_source: str
    selection_source: str
    drift_kind: str
