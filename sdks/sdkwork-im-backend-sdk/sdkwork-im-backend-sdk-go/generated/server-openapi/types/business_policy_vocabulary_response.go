package types


type BusinessPolicyVocabularyResponse struct {
	CapabilityFlagsField string `json:"capabilityFlagsField"`
	HistoryVisibilityField string `json:"historyVisibilityField"`
	HistoryVisibilityModes []string `json:"historyVisibilityModes"`
	PolicyVersionField string `json:"policyVersionField"`
	RetentionPolicyRefField string `json:"retentionPolicyRefField"`
	RetentionPolicyScopes []string `json:"retentionPolicyScopes"`
}
