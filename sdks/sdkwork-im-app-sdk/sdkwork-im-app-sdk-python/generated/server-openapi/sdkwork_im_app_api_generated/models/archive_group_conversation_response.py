from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ArchiveGroupConversationResponse:
    accepted: bool
    resource_id: str
    status: str
    archive_event_id: str
    archived_at: str
    knowledgebase_archive_scheduled: bool
