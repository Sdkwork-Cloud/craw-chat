package com.sdkwork.im.backend.api.generated

data class ProtocolGovernanceResponse(
    val businessPolicyVocabulary: BusinessPolicyVocabularyResponse? = null,
    val capabilityProfile: CapabilityProfileResponse? = null,
    val effectiveSnapshot: EffectiveProtocolSnapshotResponse? = null,
    val killSwitch: KillSwitchResponse? = null,
    val quotaProfile: QuotaProfileResponse? = null,
    val rolloutPolicy: RolloutPolicyResponse? = null,
    val sdkCompatibilityBaseline: SdkCompatibilityBaselineResponse? = null
)
