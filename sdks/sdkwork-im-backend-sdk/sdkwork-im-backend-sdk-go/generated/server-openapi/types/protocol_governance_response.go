package types


type ProtocolGovernanceResponse struct {
	BusinessPolicyVocabulary BusinessPolicyVocabularyResponse `json:"businessPolicyVocabulary"`
	CapabilityProfile CapabilityProfileResponse `json:"capabilityProfile"`
	EffectiveSnapshot EffectiveProtocolSnapshotResponse `json:"effectiveSnapshot"`
	KillSwitch KillSwitchResponse `json:"killSwitch"`
	QuotaProfile QuotaProfileResponse `json:"quotaProfile"`
	RolloutPolicy RolloutPolicyResponse `json:"rolloutPolicy"`
	SdkCompatibilityBaseline SdkCompatibilityBaselineResponse `json:"sdkCompatibilityBaseline"`
}
