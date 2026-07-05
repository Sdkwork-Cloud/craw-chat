from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .provider_binding_commit_response import ProviderBindingCommitResponse


@dataclass
class ProviderPoliciesPreviewResponse:
    code: int
    data: Any
    trace_id: str
