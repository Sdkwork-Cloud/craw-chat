from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class SocialWritePersistence:
    journal_authority: bool
    snapshot_status: str
