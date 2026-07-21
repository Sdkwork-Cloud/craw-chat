package com.sdkwork.im.backend.api.generated.model;


public class ProtocolGovernanceResponse {
    private BusinessPolicyVocabularyResponse businessPolicyVocabulary;
    private CapabilityProfileResponse capabilityProfile;
    private EffectiveProtocolSnapshotResponse effectiveSnapshot;
    private KillSwitchResponse killSwitch;
    private QuotaProfileResponse quotaProfile;
    private RolloutPolicyResponse rolloutPolicy;
    private SdkCompatibilityBaselineResponse sdkCompatibilityBaseline;

    public BusinessPolicyVocabularyResponse getBusinessPolicyVocabulary() {
        return this.businessPolicyVocabulary;
    }

    public void setBusinessPolicyVocabulary(BusinessPolicyVocabularyResponse businessPolicyVocabulary) {
        this.businessPolicyVocabulary = businessPolicyVocabulary;
    }

    public CapabilityProfileResponse getCapabilityProfile() {
        return this.capabilityProfile;
    }

    public void setCapabilityProfile(CapabilityProfileResponse capabilityProfile) {
        this.capabilityProfile = capabilityProfile;
    }

    public EffectiveProtocolSnapshotResponse getEffectiveSnapshot() {
        return this.effectiveSnapshot;
    }

    public void setEffectiveSnapshot(EffectiveProtocolSnapshotResponse effectiveSnapshot) {
        this.effectiveSnapshot = effectiveSnapshot;
    }

    public KillSwitchResponse getKillSwitch() {
        return this.killSwitch;
    }

    public void setKillSwitch(KillSwitchResponse killSwitch) {
        this.killSwitch = killSwitch;
    }

    public QuotaProfileResponse getQuotaProfile() {
        return this.quotaProfile;
    }

    public void setQuotaProfile(QuotaProfileResponse quotaProfile) {
        this.quotaProfile = quotaProfile;
    }

    public RolloutPolicyResponse getRolloutPolicy() {
        return this.rolloutPolicy;
    }

    public void setRolloutPolicy(RolloutPolicyResponse rolloutPolicy) {
        this.rolloutPolicy = rolloutPolicy;
    }

    public SdkCompatibilityBaselineResponse getSdkCompatibilityBaseline() {
        return this.sdkCompatibilityBaseline;
    }

    public void setSdkCompatibilityBaseline(SdkCompatibilityBaselineResponse sdkCompatibilityBaseline) {
        this.sdkCompatibilityBaseline = sdkCompatibilityBaseline;
    }
}
