from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .portal_workspace_view import PortalWorkspaceView


@dataclass
class WorkspaceRetrieveResponse:
    code: int
    data: Any
    trace_id: str
