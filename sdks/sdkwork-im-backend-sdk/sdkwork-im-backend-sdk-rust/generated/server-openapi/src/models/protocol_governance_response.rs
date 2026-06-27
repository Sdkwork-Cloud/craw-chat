use serde::{Deserialize, Serialize};

use crate::models::{BusinessPolicyVocabularyResponse, CapabilityProfileResponse, EffectiveProtocolSnapshotResponse, KillSwitchResponse, QuotaProfileResponse, RolloutPolicyResponse, SdkCompatibilityBaselineResponse};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProtocolGovernanceResponse {
    #[serde(rename = "businessPolicyVocabulary")]
    pub business_policy_vocabulary: BusinessPolicyVocabularyResponse,

    #[serde(rename = "capabilityProfile")]
    pub capability_profile: CapabilityProfileResponse,

    #[serde(rename = "effectiveSnapshot")]
    pub effective_snapshot: EffectiveProtocolSnapshotResponse,

    #[serde(rename = "killSwitch")]
    pub kill_switch: KillSwitchResponse,

    #[serde(rename = "quotaProfile")]
    pub quota_profile: QuotaProfileResponse,

    #[serde(rename = "rolloutPolicy")]
    pub rollout_policy: RolloutPolicyResponse,

    #[serde(rename = "sdkCompatibilityBaseline")]
    pub sdk_compatibility_baseline: SdkCompatibilityBaselineResponse,
}
