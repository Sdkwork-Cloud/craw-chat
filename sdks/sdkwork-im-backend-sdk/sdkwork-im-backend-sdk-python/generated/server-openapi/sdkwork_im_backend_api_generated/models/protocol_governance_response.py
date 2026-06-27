from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .business_policy_vocabulary_response import BusinessPolicyVocabularyResponse
    from .capability_profile_response import CapabilityProfileResponse
    from .effective_protocol_snapshot_response import EffectiveProtocolSnapshotResponse
    from .kill_switch_response import KillSwitchResponse
    from .quota_profile_response import QuotaProfileResponse
    from .rollout_policy_response import RolloutPolicyResponse
    from .sdk_compatibility_baseline_response import SdkCompatibilityBaselineResponse


@dataclass
class ProtocolGovernanceResponse:
    business_policy_vocabulary: BusinessPolicyVocabularyResponse
    capability_profile: CapabilityProfileResponse
    effective_snapshot: EffectiveProtocolSnapshotResponse
    kill_switch: KillSwitchResponse
    quota_profile: QuotaProfileResponse
    rollout_policy: RolloutPolicyResponse
    sdk_compatibility_baseline: SdkCompatibilityBaselineResponse
