from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .commit_envelope_response import CommitEnvelopeResponse
    from .social_write_persistence import SocialWritePersistence
    from .user_block import UserBlock


@dataclass
class OpenApiUserBlockResponse:
    user_block: UserBlock
    latest_commit: CommitEnvelopeResponse
    persistence: SocialWritePersistence
