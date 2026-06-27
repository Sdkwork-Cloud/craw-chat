from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RtcSessionMutationResponse:
    tenant_id: str
    rtc_session_id: str
    initiator_id: str
    initiator_kind: str
    rtc_mode: str
    state: str
    started_at: str
    request_key: str
    delivery_status: str
    proof_version: str
    conversation_id: Optional[str] = None
    provider_plugin_id: Optional[str] = None
    provider_session_id: Optional[str] = None
    access_endpoint: Optional[str] = None
    provider_region: Optional[str] = None
    signaling_stream_id: Optional[str] = None
    artifact_message_id: Optional[str] = None
    ended_at: Optional[str] = None
