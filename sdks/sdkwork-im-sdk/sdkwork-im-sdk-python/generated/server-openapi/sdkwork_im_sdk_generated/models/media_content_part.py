from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .drive_reference import DriveReference
    from .media_resource import MediaResource


@dataclass
class MediaContentPart:
    kind: str
    drive: DriveReference
    resource: MediaResource
    media_role: Optional[str] = None
