from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class PortalSnapshotMeta:
    section: str
    generated_at: str
    ops_status: str
