from __future__ import annotations
from typing import Any, Dict, Union

from .text_content_part import TextContentPart
from .data_content_part import DataContentPart
from .media_content_part import MediaContentPart
from .signal_content_part import SignalContentPart
from .stream_ref_content_part import StreamRefContentPart


ContentPart = Union[TextContentPart, DataContentPart, MediaContentPart, SignalContentPart, StreamRefContentPart]


def content_part_from_dict(value: Dict[str, Any]) -> ContentPart:
    kind = value.get('kind')
    if kind == 'text':
        return TextContentPart(**value)
    if kind == 'data':
        return DataContentPart(**value)
    if kind == 'media':
        return MediaContentPart(**value)
    if kind == 'signal':
        return SignalContentPart(**value)
    if kind == 'stream_ref':
        return StreamRefContentPart(**value)
    raise ValueError(f"Unknown kind discriminator: {kind}")
