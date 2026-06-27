from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SdkCompatibilityBaselineResponse:
    app_sdk_family: str
    backend_sdk_family: str
    im_sdk_family: str
    rtc_sdk_family: str
    matrix_client_types: List[str]
    protocol_governance_path: str
    protocol_registry_path: str
