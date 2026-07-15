from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ArchiveGroupConversationRequest:
    """Explicit command input. The group target and archive actor are derived from the authenticated request context and path and cannot be supplied by the caller."""
    pass
