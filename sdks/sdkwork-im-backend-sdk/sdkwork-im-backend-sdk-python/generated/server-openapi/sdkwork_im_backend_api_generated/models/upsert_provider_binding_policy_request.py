from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class UpsertProviderBindingPolicyRequest:
    domain: str
    plugin_id: str
    expected_base_version: Optional[int] = None
    tenant_id: Optional[str] = None
