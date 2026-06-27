from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class MediaResource:
    source: str
    uri: str
    id: Optional[str] = None
    kind: Optional[str] = None
    media_kind: Optional[str] = None
    public_url: Optional[str] = None
    url: Optional[str] = None
    name: Optional[str] = None
    title: Optional[str] = None
    file_name: Optional[str] = None
    mime_type: Optional[str] = None
    size: Optional[int] = None
    size_bytes: Optional[str] = None
    file_size: Optional[str] = None
    duration_seconds: Optional[int] = None
    poster: Optional[MediaResource] = None
    thumbnails: Optional[List[MediaResource]] = None
