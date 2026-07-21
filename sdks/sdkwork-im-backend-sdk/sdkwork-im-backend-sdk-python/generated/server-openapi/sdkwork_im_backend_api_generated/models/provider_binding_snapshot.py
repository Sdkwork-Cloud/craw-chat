from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .provider_binding_item import ProviderBindingItem


@dataclass
class ProviderBindingSnapshot:
    interface_version: str
    tenant_id: Optional[str]
    effective_bindings: List[ProviderBindingItem]
    precedence: List[str]
