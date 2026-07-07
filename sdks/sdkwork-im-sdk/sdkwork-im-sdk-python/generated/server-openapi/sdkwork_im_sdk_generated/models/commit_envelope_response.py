from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .event_actor import EventActor


@dataclass
class CommitEnvelopeResponse:
    event_id: str
    tenant_id: str
    event_type: str
    event_version: int
    aggregate_type: str
    aggregate_id: str
    scope_type: str
    scope_id: str
    ordering_key: str
    ordering_seq: int
    actor: EventActor
    occurred_at: str
    committed_at: str
    payload: str
    retention_class: str
    audit_class: str
    causation_id: Optional[str] = None
    correlation_id: Optional[str] = None
    idempotency_key: Optional[str] = None
    payload_schema: Optional[str] = None
