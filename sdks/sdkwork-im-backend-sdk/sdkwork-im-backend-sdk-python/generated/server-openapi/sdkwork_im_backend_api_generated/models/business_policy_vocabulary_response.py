from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class BusinessPolicyVocabularyResponse:
    capability_flags_field: str
    history_visibility_field: str
    history_visibility_modes: List[str]
    policy_version_field: str
    retention_policy_ref_field: str
    retention_policy_scopes: List[str]
