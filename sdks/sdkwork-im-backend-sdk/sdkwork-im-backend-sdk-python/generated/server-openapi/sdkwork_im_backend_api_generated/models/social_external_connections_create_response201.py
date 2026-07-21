from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .social_external_connection_commit_response import SocialExternalConnectionCommitResponse


@dataclass
class SocialExternalConnectionsCreateResponse201:
    code: int
    data: Any
    trace_id: str
