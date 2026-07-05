from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class MessageForwardReference:
    """Source tracing metadata for a forwarded message. Carries attribution to the original message across conversations so the UI can render a "Forwarded from <sender>" label and preserve audit provenance. The forwarder remains the Sender of the new message; this object only records where the content originated. Cross-conversation recall visibility is intentionally NOT cascaded — recipients of a forward see the original snapshot at forward-time."""
    original_message_id: str
    original_conversation_id: str
    original_sender_id: str
    original_sender_kind: str
    original_sender_display_name: str
    original_occurred_at: str
    forwarded_at: str
    content_preview: str
    forward_count: Optional[int] = None
