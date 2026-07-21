from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ProviderBindingItem:
    domain: str
    default_plugin_id: Optional[str]
    selected_plugin_id: Optional[str]
    selection_source: str
    tenant_override_allowed: bool
