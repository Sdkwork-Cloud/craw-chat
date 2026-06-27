from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class DriveReference:
    drive_uri: str
    space_id: str
    node_id: str
    node_version: Optional[str] = None
